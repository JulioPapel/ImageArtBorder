//! Container-level metadata preservation (JPEG marker splice, PNG chunks).

mod jpeg;
mod png;

pub use jpeg::{metadata_segment_bytes, splice_jpeg};
pub use png::finalize_png;
