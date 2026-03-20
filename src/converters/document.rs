use crate::formats::Format;
use anyhow::{bail, Result};
use std::path::Path;

pub fn convert(input: &Path, src: &Format, output: &Path, target: &Format) -> Result<()> {
    match (src, target) {
        (Format::Markdown, Format::Html) => md_to_html(input, output),
        (Format::Html, Format::PlainText) => html_to_text(input, output),
        (Format::Markdown, Format::PlainText) => md_to_text(input, output),
        (Format::PlainText, Format::Html) => text_to_html(input, output),

        // PDF source — use MuPDF for rendering
        (Format::Pdf, Format::Png) | (Format::Pdf, Format::Jpeg) => {
            #[cfg(feature = "mupdf-backend")]
            {
                let tmp_png = output.with_extension("_mupdf_tmp.png");
                super::document_pdf::render_page_to_png(input, 0, 150.0, &tmp_png)?;
                if *target == Format::Png {
                    std::fs::rename(&tmp_png, output)?;
                } else {
                    let img = image::open(&tmp_png)?;
                    img.save(output)?;
                    std::fs::remove_file(&tmp_png)?;
                }
                Ok(())
            }
            #[cfg(not(feature = "mupdf-backend"))]
            anyhow::bail!("PDF rendering requires building with --features mupdf-backend")
        }

        // PDF pass-through (validate readability, then copy)
        (Format::Pdf, Format::Pdf) => {
            #[cfg(feature = "mupdf-backend")]
            {
                let _count = super::document_pdf::pdf_page_count(input)?;
                std::fs::copy(input, output)?;
                Ok(())
            }
            #[cfg(not(feature = "mupdf-backend"))]
            anyhow::bail!("PDF operations require building with --features mupdf-backend")
        }

        (src, target) => bail!("Document conversion {:?} → {:?} not yet supported", src, target),
    }
}

fn md_to_html(input: &Path, output: &Path) -> Result<()> {
    let md = std::fs::read_to_string(input)?;
    let parser = pulldown_cmark::Parser::new_ext(&md, pulldown_cmark::Options::all());
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);
    let full = format!(
        "<!DOCTYPE html><html><head><meta charset=\"utf-8\"></head><body>{html}</body></html>"
    );
    std::fs::write(output, full)?;
    Ok(())
}

fn html_to_text(input: &Path, output: &Path) -> Result<()> {
    let html = std::fs::read_to_string(input)?;
    // Strip tags with a simple regex-free approach
    let text = strip_html_tags(&html);
    std::fs::write(output, text)?;
    Ok(())
}

fn md_to_text(input: &Path, output: &Path) -> Result<()> {
    let md = std::fs::read_to_string(input)?;
    // Render to HTML first, then strip tags
    let parser = pulldown_cmark::Parser::new_ext(&md, pulldown_cmark::Options::all());
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);
    let text = strip_html_tags(&html);
    std::fs::write(output, text)?;
    Ok(())
}

fn text_to_html(input: &Path, output: &Path) -> Result<()> {
    let text = std::fs::read_to_string(input)?;
    let escaped = text
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;");
    let html = format!(
        "<!DOCTYPE html><html><head><meta charset=\"utf-8\"></head><body><pre>{escaped}</pre></body></html>"
    );
    std::fs::write(output, html)?;
    Ok(())
}

fn strip_html_tags(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut in_tag = false;
    for c in html.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(c),
            _ => {}
        }
    }
    // Clean up excessive whitespace
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}
