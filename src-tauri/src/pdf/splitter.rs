//! Core PDF splitting logic.
//!
//! # Overview
//!
//! This module is intentionally **framework-agnostic**: it has no knowledge of
//! Tauri, windows, or any UI concern.  Those live in the sibling
//! [`commands`](crate::commands) module.  This keeps the splitting logic
//! unit-testable in isolation and reusable in other contexts (CLI companion,
//! server-side processing, etc.).
//!
//! # Strategy — Clone & Prune
//!
//! PDF documents are deeply interconnected.  Fonts, images, and content
//! streams are typically defined once and shared across many pages.  The
//! safest strategy for extracting a single page while preserving 100 %
//! fidelity is therefore:
//!
//! 1. **Load**   — parse the source document into memory *once*.
//! 2. **Clone**  — for each target page, clone the entire in-memory document.
//! 3. **Delete** — remove every page reference that is *not* the target.
//! 4. **Prune**  — call [`lopdf::Document::prune_objects`] to garbage-collect
//!    every object no longer reachable from the surviving page.
//! 5. **Renumber** — compact object IDs with
//!    [`lopdf::Document::renumber_objects`] to keep output file sizes small.
//! 6. **Save**   — write the single-page document to disk.
//!
//! Steps 2–6 run **concurrently** across all CPU cores via [`rayon`], so a
//! 100-page document is processed in roughly `⌈100 / num_cpus⌉` sequential
//! page-times rather than `100 × t_page`.

use std::{
    fs,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
    time::Instant,
};

use lopdf::Document;
use rayon::prelude::*;

use super::error::PdfError;

// ── Public types ──────────────────────────────────────────────────────────────

/// Parameters for a single split operation.
#[derive(Debug, Clone)]
pub struct SplitRequest {
    /// Absolute path to the source PDF file.
    pub input_path: PathBuf,
    /// Directory into which individual-page PDFs will be written.  Created
    /// automatically (including intermediate directories) if it does not exist.
    pub output_dir: PathBuf,
}

/// Outcome of a successful split operation.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SplitResult {
    /// Total number of pages found in the source document.
    pub total_pages: u32,
    /// Paths of every output file, sorted lexicographically.
    pub output_files: Vec<PathBuf>,
    /// Wall-clock time taken for the whole operation, in milliseconds.
    pub elapsed_ms: u64,
}

/// Progress snapshot emitted after each page finishes processing.
///
/// The callback may be called from multiple rayon worker threads concurrently,
/// so it must be both `Send` and `Sync`.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageProgress {
    /// 1-based count of pages that have been written to disk so far.
    pub current: u32,
    /// Total number of pages to process.
    pub total: u32,
    /// Filename of the most-recently completed output file
    /// (e.g. `"page_0042.pdf"`).
    pub file_name: String,
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Return the number of pages in the PDF at `path` without performing a full
/// split.
///
/// This is a lightweight pre-flight helper for the UI: load the document,
/// query the page tree, and return the count.
///
/// # Errors
///
/// | Variant | Cause |
/// |---------|-------|
/// | [`PdfError::FileNotFound`] | `path` does not point to a regular file. |
/// | [`PdfError::InvalidPdf`]   | The file cannot be parsed as a valid PDF.  |
/// | [`PdfError::NoPages`]      | The document exists but has zero pages.    |
pub fn get_page_count(path: &Path) -> Result<u32, PdfError> {
    if !path.is_file() {
        return Err(PdfError::FileNotFound {
            path: path.display().to_string(),
        });
    }

    let doc = Document::load(path)?;
    let count = doc.get_pages().len();

    if count == 0 {
        return Err(PdfError::NoPages);
    }

    u32::try_from(count)
        .map_err(|_| PdfError::Internal(format!("document has {count} pages, which overflows u32")))
}

/// Split every page of the PDF at `request.input_path` into its own PDF file
/// inside `request.output_dir`.
///
/// Output files are named `page_0001.pdf`, `page_0002.pdf`, … in the original
/// page order, regardless of the rayon execution schedule.
///
/// `on_progress` is called **once per completed page**, possibly from multiple
/// rayon worker threads at the same time.  Keep it non-blocking (e.g. send on
/// a channel or atomically update a counter) so it does not stall the thread
/// pool.
///
/// # Errors
///
/// | Variant | Cause |
/// |---------|-------|
/// | [`PdfError::FileNotFound`] | `input_path` does not point to a regular file. |
/// | [`PdfError::InvalidPdf`]   | The file cannot be parsed as a valid PDF.  |
/// | [`PdfError::NoPages`]      | The document has zero pages to split.      |
/// | [`PdfError::Io`]           | Directory creation or file write failure.  |
pub fn split_pdf<F>(request: SplitRequest, on_progress: F) -> Result<SplitResult, PdfError>
where
    F: Fn(PageProgress) + Send + Sync,
{
    let started_at = Instant::now();

    // Destructure immediately so the compiler sees all fields are consumed.
    let SplitRequest {
        input_path,
        output_dir,
    } = request;

    // ── Validate input ────────────────────────────────────────────────────────

    if !input_path.is_file() {
        return Err(PdfError::FileNotFound {
            path: input_path.display().to_string(),
        });
    }

    // ── Prepare output directory ──────────────────────────────────────────────

    // `create_dir_all` is idempotent: it succeeds whether or not the directory
    // already exists, and it creates intermediate directories as needed.
    fs::create_dir_all(&output_dir)?;

    // ── Load source document (once) ───────────────────────────────────────────

    let source = Document::load(&input_path)?;

    // Collect page numbers in ascending order.  lopdf uses 1-based page
    // numbers as keys.
    let all_page_numbers: Vec<u32> = {
        let pages = source.get_pages();

        if pages.is_empty() {
            return Err(PdfError::NoPages);
        }

        let mut v: Vec<u32> = pages.keys().copied().collect();
        v.sort_unstable();
        v
    };

    let total = u32::try_from(all_page_numbers.len())
        .map_err(|_| PdfError::Internal("page count overflows u32".to_owned()))?;

    // ── Wrap shared data in `Arc` for safe multi-thread access ────────────────

    // `Arc<Document>` is `Send + Sync` because `lopdf::Document` contains only
    // plain collections without interior mutability.  Each rayon worker calls
    // `.clone()` on the inner value to obtain its own mutable copy.
    let source = Arc::new(source);

    // Immutable view of all page numbers, shared across workers.
    let all_page_numbers = Arc::new(all_page_numbers);

    // Shared atomic counter for progress reporting.
    let counter = Arc::new(AtomicU32::new(0));

    // Shared reference to the progress callback.
    let on_progress = Arc::new(on_progress);

    // Shared output directory path.
    let output_dir = Arc::new(output_dir);

    // ── Parallel processing ───────────────────────────────────────────────────

    // `par_iter().enumerate()` assigns a stable *sequence index* to each page
    // (0-based, matching sorted page order) regardless of the rayon execution
    // schedule.  This guarantees deterministic output filenames even when
    // workers finish out of order.
    let raw_results: Vec<Result<PathBuf, PdfError>> = all_page_numbers
        .par_iter()
        .enumerate()
        .map(|(seq_index, &page_num)| {
            process_single_page(
                seq_index,
                page_num,
                &source,
                &all_page_numbers,
                &output_dir,
                &counter,
                total,
                &on_progress,
            )
        })
        .collect();

    // ── Collect results ───────────────────────────────────────────────────────

    // If *any* page failed, surface the first error encountered.
    let mut output_files: Vec<PathBuf> = Vec::with_capacity(raw_results.len());
    for result in raw_results {
        output_files.push(result?);
    }

    // Sort for deterministic ordering regardless of rayon scheduling.
    output_files.sort_unstable();

    Ok(SplitResult {
        total_pages: total,
        output_files,
        elapsed_ms: u64::try_from(started_at.elapsed().as_millis()).unwrap_or(u64::MAX),
    })
}

// ── Private helpers ───────────────────────────────────────────────────────────

/// Process a single page: clone the source document, prune all other pages,
/// write the result to disk, and emit a progress event.
///
/// Extracted into its own function to keep the parallel closure small and to
/// make the call site easier to read.
#[allow(clippy::too_many_arguments)] // All parameters are required; a struct would be over-engineering.
fn process_single_page(
    seq_index: usize,
    page_num: u32,
    source: &Arc<Document>,
    all_page_numbers: &Arc<Vec<u32>>,
    output_dir: &Arc<PathBuf>,
    counter: &Arc<AtomicU32>,
    total: u32,
    on_progress: &Arc<impl Fn(PageProgress) + Send + Sync>,
) -> Result<PathBuf, PdfError> {
    // ── Clone & Prune ─────────────────────────────────────────────────────────

    // `Arc::as_ref` + `Clone::clone` gives each worker a fully independent,
    // mutable copy of the document without unsafe sharing.
    let mut doc = source.as_ref().clone();

    // Build the delete list: every internal PDF page number except this one.
    let to_delete: Vec<u32> = all_page_numbers
        .iter()
        .filter(|&&p| p != page_num)
        .copied()
        .collect();

    doc.delete_pages(&to_delete);
    doc.prune_objects();
    doc.renumber_objects();

    // ── Write output ──────────────────────────────────────────────────────────

    // `seq_index` is 0-based; filenames are 1-based for human readability.
    let file_name = format!("page_{:04}.pdf", seq_index + 1);
    let output_path = output_dir.join(&file_name);
    doc.save(&output_path)?;

    // ── Report progress ───────────────────────────────────────────────────────

    // `fetch_add` returns the *previous* value, so we add 1 for the 1-based
    // display count.
    let current = counter.fetch_add(1, Ordering::Relaxed) + 1;
    on_progress(PageProgress {
        current,
        total,
        file_name,
    });

    Ok(output_path)
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    // ── Test helpers ──────────────────────────────────────────────────────────

    /// Build a minimal but structurally valid PDF with `page_count` blank
    /// pages using the lopdf API.  Panics on any internal error — this is
    /// intentional in a test context so failures surface as clear panics.
    fn make_minimal_pdf(page_count: usize) -> Vec<u8> {
        use lopdf::{dictionary, Document, Object};

        assert!(page_count > 0, "page_count must be at least 1");

        let mut doc = Document::with_version("1.7");

        // Reserve the Pages node ID so we can forward-reference it from each
        // Page's /Parent entry.
        let pages_id = doc.new_object_id();

        // Create individual Page objects.
        let kid_refs: Vec<Object> = (0..page_count)
            .map(|_| {
                let page = dictionary! {
                    "Type"      => "Page",
                    "Parent"    => Object::Reference(pages_id),
                    "MediaBox"  => Object::Array(vec![
                        Object::Integer(0),
                        Object::Integer(0),
                        Object::Integer(612),
                        Object::Integer(792),
                    ]),
                };
                let pid = doc.add_object(Object::Dictionary(page));
                Object::Reference(pid)
            })
            .collect();

        // Insert Pages node at the pre-reserved ID.
        let pages = dictionary! {
            "Type"  => "Pages",
            "Kids"  => Object::Array(kid_refs),
            "Count" => Object::Integer(i64::try_from(page_count).expect("page_count fits i64")),
        };
        doc.objects.insert(pages_id, Object::Dictionary(pages));

        // Catalog
        let catalog = dictionary! {
            "Type"  => "Catalog",
            "Pages" => Object::Reference(pages_id),
        };
        let catalog_id = doc.add_object(Object::Dictionary(catalog));
        doc.trailer.set("Root", Object::Reference(catalog_id));

        let mut buf = Vec::new();
        doc.save_to(&mut buf)
            .expect("test helper: failed to serialise PDF");
        buf
    }

    /// Write `bytes` to a temporary file inside `dir` and return its path.
    fn write_pdf(dir: &tempfile::TempDir, name: &str, bytes: &[u8]) -> PathBuf {
        let path = dir.path().join(name);
        fs::write(&path, bytes).expect("test helper: failed to write PDF");
        path
    }

    // ── get_page_count ────────────────────────────────────────────────────────

    #[test]
    fn get_page_count_returns_error_for_missing_file() {
        let result = get_page_count(Path::new("/nonexistent/path/file.pdf"));
        assert!(
            matches!(result, Err(PdfError::FileNotFound { .. })),
            "expected FileNotFound, got: {result:?}"
        );
    }

    #[test]
    fn get_page_count_single_page() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = write_pdf(&dir, "single.pdf", &make_minimal_pdf(1));
        assert_eq!(get_page_count(&path).expect("count"), 1);
    }

    #[test]
    fn get_page_count_multiple_pages() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = write_pdf(&dir, "multi.pdf", &make_minimal_pdf(5));
        assert_eq!(get_page_count(&path).expect("count"), 5);
    }

    // ── split_pdf — error paths ───────────────────────────────────────────────

    #[test]
    fn split_pdf_returns_error_for_missing_input() {
        let dir = tempfile::tempdir().expect("tempdir");
        let result = split_pdf(
            SplitRequest {
                input_path: PathBuf::from("/no/such/file.pdf"),
                output_dir: dir.path().to_path_buf(),
            },
            |_| {},
        );
        assert!(
            matches!(result, Err(PdfError::FileNotFound { .. })),
            "expected FileNotFound, got: {result:?}"
        );
    }

    // ── split_pdf — happy path ────────────────────────────────────────────────

    #[test]
    fn split_pdf_produces_correct_number_of_files() {
        let dir = tempfile::tempdir().expect("tempdir");
        let input = write_pdf(&dir, "source.pdf", &make_minimal_pdf(4));
        let out_dir = dir.path().join("output");

        let result = split_pdf(
            SplitRequest {
                input_path: input,
                output_dir: out_dir,
            },
            |_| {},
        )
        .expect("split should succeed");

        assert_eq!(result.total_pages, 4);
        assert_eq!(result.output_files.len(), 4);
    }

    #[test]
    fn split_pdf_all_output_files_exist_on_disk() {
        let dir = tempfile::tempdir().expect("tempdir");
        let input = write_pdf(&dir, "source.pdf", &make_minimal_pdf(3));
        let out_dir = dir.path().join("out");

        let result = split_pdf(
            SplitRequest {
                input_path: input,
                output_dir: out_dir,
            },
            |_| {},
        )
        .expect("split");

        for path in &result.output_files {
            assert!(path.exists(), "expected {path:?} to exist on disk");
        }
    }

    #[test]
    fn split_pdf_output_files_are_sorted() {
        let dir = tempfile::tempdir().expect("tempdir");
        let input = write_pdf(&dir, "source.pdf", &make_minimal_pdf(6));
        let out_dir = dir.path().join("sorted");

        let result = split_pdf(
            SplitRequest {
                input_path: input,
                output_dir: out_dir,
            },
            |_| {},
        )
        .expect("split");

        let mut expected = result.output_files.clone();
        expected.sort_unstable();
        assert_eq!(
            result.output_files, expected,
            "output_files should be lexicographically sorted"
        );
    }

    #[test]
    fn split_pdf_output_files_have_sequential_names() {
        let dir = tempfile::tempdir().expect("tempdir");
        let input = write_pdf(&dir, "source.pdf", &make_minimal_pdf(3));
        let out_dir = dir.path().join("named");

        let result = split_pdf(
            SplitRequest {
                input_path: input,
                output_dir: out_dir,
            },
            |_| {},
        )
        .expect("split");

        let names: Vec<String> = result
            .output_files
            .iter()
            .map(|p| p.file_name().unwrap().to_string_lossy().into_owned())
            .collect();

        assert_eq!(
            names,
            vec!["page_0001.pdf", "page_0002.pdf", "page_0003.pdf"]
        );
    }

    #[test]
    fn split_pdf_creates_output_dir_if_missing() {
        let dir = tempfile::tempdir().expect("tempdir");
        let input = write_pdf(&dir, "source.pdf", &make_minimal_pdf(1));
        // Deeply nested directory that does not exist yet.
        let out_dir = dir.path().join("a").join("b").join("c");

        assert!(!out_dir.exists(), "pre-condition: dir should not exist yet");

        split_pdf(
            SplitRequest {
                input_path: input,
                output_dir: out_dir.clone(),
            },
            |_| {},
        )
        .expect("split");

        assert!(
            out_dir.exists(),
            "output directory should have been created"
        );
    }

    // ── Progress callback ─────────────────────────────────────────────────────

    #[test]
    fn split_pdf_progress_callback_is_called_for_every_page() {
        let dir = tempfile::tempdir().expect("tempdir");
        let input = write_pdf(&dir, "source.pdf", &make_minimal_pdf(5));
        let out_dir = dir.path().join("prog");

        let log: Arc<Mutex<Vec<PageProgress>>> = Arc::new(Mutex::new(Vec::new()));
        let log_clone = Arc::clone(&log);

        split_pdf(
            SplitRequest {
                input_path: input,
                output_dir: out_dir,
            },
            move |p| {
                log_clone.lock().expect("mutex poisoned").push(p);
            },
        )
        .expect("split");

        let count = log.lock().expect("mutex").len();
        assert_eq!(count, 5, "callback should be called once per page");
    }

    #[test]
    fn split_pdf_progress_total_is_correct() {
        let dir = tempfile::tempdir().expect("tempdir");
        let input = write_pdf(&dir, "source.pdf", &make_minimal_pdf(4));
        let out_dir = dir.path().join("total");

        let log: Arc<Mutex<Vec<PageProgress>>> = Arc::new(Mutex::new(Vec::new()));
        let log_clone = Arc::clone(&log);

        split_pdf(
            SplitRequest {
                input_path: input,
                output_dir: out_dir,
            },
            move |p| {
                log_clone.lock().expect("mutex").push(p);
            },
        )
        .expect("split");

        let events: Vec<PageProgress> = log.lock().expect("mutex").clone();
        for event in &events {
            assert_eq!(event.total, 4, "every progress event should report total=4");
        }
    }

    #[test]
    fn split_pdf_progress_current_values_cover_full_range() {
        let dir = tempfile::tempdir().expect("tempdir");
        let input = write_pdf(&dir, "source.pdf", &make_minimal_pdf(3));
        let out_dir = dir.path().join("range");

        let log: Arc<Mutex<Vec<u32>>> = Arc::new(Mutex::new(Vec::new()));
        let log_clone = Arc::clone(&log);

        split_pdf(
            SplitRequest {
                input_path: input,
                output_dir: out_dir,
            },
            move |p| {
                log_clone.lock().expect("mutex").push(p.current);
            },
        )
        .expect("split");

        let mut currents = log.lock().expect("mutex").clone();
        currents.sort_unstable();

        assert_eq!(
            currents,
            vec![1, 2, 3],
            "current values 1..=total must all appear exactly once"
        );
    }

    // ── Output page integrity ─────────────────────────────────────────────────

    #[test]
    fn split_pdf_each_output_is_a_single_page_pdf() {
        let dir = tempfile::tempdir().expect("tempdir");
        let input = write_pdf(&dir, "source.pdf", &make_minimal_pdf(3));
        let out_dir = dir.path().join("integrity");

        let result = split_pdf(
            SplitRequest {
                input_path: input,
                output_dir: out_dir,
            },
            |_| {},
        )
        .expect("split");

        for path in &result.output_files {
            let doc = Document::load(path).expect("output PDF should be loadable");
            assert_eq!(
                doc.get_pages().len(),
                1,
                "{path:?} should contain exactly 1 page"
            );
        }
    }

    // ── SplitResult fields ────────────────────────────────────────────────────

    #[test]
    fn split_result_elapsed_ms_is_accessible() {
        let dir = tempfile::tempdir().expect("tempdir");
        let input = write_pdf(&dir, "source.pdf", &make_minimal_pdf(2));
        let out_dir = dir.path().join("elapsed");

        let result = split_pdf(
            SplitRequest {
                input_path: input,
                output_dir: out_dir,
            },
            |_| {},
        )
        .expect("split");

        // elapsed_ms may be 0 on very fast machines; we just assert the field
        // is present and of the correct type.
        let _: u64 = result.elapsed_ms;
    }
}
