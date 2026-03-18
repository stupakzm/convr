use crate::formats::Format;
use std::path::Path;

/// Detect format from file extension, falling back to magic bytes.
pub fn detect(path: &Path) -> Option<Format> {
    // Try extension first
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        if let Some(fmt) = Format::from_extension(ext) {
            return Some(fmt);
        }
    }
    // Fall back to magic bytes
    if let Ok(bytes) = std::fs::read(path) {
        if let Some(kind) = infer::get(&bytes) {
            return Format::from_extension(kind.extension());
        }
    }
    None
}
