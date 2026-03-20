//! PDF read/write/render via MuPDF (feature = "mupdf-backend")
//!
//! MuPDF is bundled and compiled from source by mupdf-sys. No system install needed.
//! License: AGPL-3.0 — distributing binaries with this feature requires AGPL compliance.

use anyhow::Result;
use std::path::Path;
use mupdf::Document;

/// Open a PDF and return the page count.
pub fn pdf_page_count(path: &Path) -> Result<u32> {
    let doc = Document::open(
        path.to_str().ok_or_else(|| anyhow::anyhow!("non-UTF8 path: {:?}", path))?
    )?;
    let count = doc.page_count()?;
    Ok(count as u32)
}

/// Open a PDF document.
pub fn open_pdf(path: &Path) -> Result<Document> {
    Ok(Document::open(
        path.to_str().ok_or_else(|| anyhow::anyhow!("non-UTF8 path: {:?}", path))?
    )?)
}

/// Render a single PDF page to a PNG image file.
pub fn render_page_to_png(pdf_path: &Path, page_idx: i32, dpi: f32, output: &Path) -> Result<()> {
    let doc = open_pdf(pdf_path)?;
    let page = doc.load_page(page_idx)?;
    let scale = dpi / 72.0;
    let matrix = mupdf::Matrix::new_scale(scale, scale);
    let pixmap = page.to_pixmap(&matrix, &mupdf::Colorspace::device_rgb(), 1.0, true)?;
    let png_bytes = pixmap.to_png()?;
    std::fs::write(output, png_bytes)?;
    Ok(())
}
