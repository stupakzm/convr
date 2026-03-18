use crate::converters::{data, document, image};
use crate::formats::{Category, Format};
use anyhow::{bail, Result};
use std::path::Path;

pub fn convert(input: &Path, src: &Format, output: &Path, target: &Format) -> Result<()> {
    if let Some(parent) = output.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)?;
        }
    }

    match (src.category(), target.category()) {
        (Category::Image, Category::Image) => image::convert(input, src, output, target),
        (Category::Data, Category::Data) => data::convert(input, src, output, target),
        (Category::Document, Category::Document) => document::convert(input, src, output, target),
        (Category::Document, Category::Image) => bail!("Document → Image not supported yet"),
        (Category::Image, Category::Document) => bail!("Image → Document not supported yet"),
        (src_cat, tgt_cat) => bail!("Cross-category conversion {:?} → {:?} not supported", src_cat, tgt_cat),
    }
}
