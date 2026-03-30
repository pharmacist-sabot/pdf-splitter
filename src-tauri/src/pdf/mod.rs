//! PDF processing pipeline.
//!
//! This module exposes the public API for all PDF operations used by the
//! Tauri command layer.  Internal implementation details are kept in the
//! sub-modules below and are not part of the public interface.
//!
//! # Module layout
//!
//! | Sub-module | Responsibility |
//! |------------|----------------|
//! | [`error`]  | [`PdfError`] — the single error type for the whole pipeline. |
//! | [`splitter`] | [`split_pdf`] and [`get_page_count`] — the core logic. |
//!
//! # Usage
//!
//! ```rust,ignore
//! use crate::pdf::{SplitRequest, split_pdf, get_page_count};
//!
//! let count = get_page_count(Path::new("/tmp/document.pdf"))?;
//!
//! let result = split_pdf(
//!     SplitRequest {
//!         input_path:  PathBuf::from("/tmp/document.pdf"),
//!         output_dir:  PathBuf::from("/tmp/pages"),
//!     },
//!     |progress| println!("{}/{}", progress.current, progress.total),
//! )?;
//! ```

pub mod error;
pub mod splitter;

// ── Convenience re-exports ────────────────────────────────────────────────────

// Error type
pub use error::PdfError;

// Core split API
pub use splitter::{get_page_count, split_pdf, PageProgress, SplitRequest, SplitResult};
