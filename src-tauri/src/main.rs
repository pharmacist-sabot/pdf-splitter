// Prevents an additional console window on Windows in release builds.
// This attribute is harmless on macOS/Linux.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    pdf_splitter_lib::run();
}
