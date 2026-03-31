//! Tauri application entry point.

// Enforce a strict, idiomatic Rust style throughout the crate.
#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    // Forbid unsafe code — this application has no need for it.
    unsafe_code
)]
// Pedantic lints that produce too many false positives for Tauri glue code.
#![allow(
    // Tauri-generated code sometimes triggers this.
    clippy::used_underscore_binding,
    // Module-level docs are preferred over item-level docs for re-exports.
    clippy::module_name_repetitions,
)]

// module declarations

/// Tauri command handlers (thin wrappers around the `pdf` pipeline).
pub mod commands;

/// Framework-agnostic PDF processing pipeline.
pub mod pdf;

// public api

/// Build and run the Tauri application.
///
/// This function **never returns** on a successful run; it hands control to
/// the Tauri event loop.  On failure it panics with a descriptive message.
///
/// # Panics
///
/// Panics if the Tauri runtime cannot be initialised.  This should only
/// happen in abnormal conditions (e.g. missing system `WebView` support, which
/// cannot occur on macOS `>= 12` where `WebKit` is always present).
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // Plugin registration
        // Plugins must be registered before `invoke_handler` so that their
        // commands are available when the renderer calls `invoke()`.
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        // Command registration
        //
        // `generate_handler!` is a Tauri macro that builds the dispatch table
        // from the list of `#[tauri::command]` functions.  Every public command
        // in `commands.rs` must appear here.
        .invoke_handler(tauri::generate_handler![
            commands::get_page_count,
            commands::get_file_info,
            commands::pick_pdf_file,
            commands::pick_output_dir,
            commands::split_pdf,
            commands::reveal_in_finder,
        ])
        // Launch
        .run(tauri::generate_context!())
        .expect("fatal: Tauri application failed to start");
}
