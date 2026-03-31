//! PDF processing pipeline.

pub mod error;
pub mod splitter;

pub use error::PdfError;
pub use splitter::{get_page_count, split_pdf, PageProgress, SplitRequest, SplitResult};
