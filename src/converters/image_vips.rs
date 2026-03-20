//! Raster image operations via libvips (feature = "vips")
//!
//! libvips provides streaming, memory-mapped image processing that handles
//! large images without loading them fully into RAM.

use anyhow::Result;
use std::path::Path;
use crate::backends::vips_app;
use libvips::{ops, VipsImage};

/// Load an image via libvips and return dimensions (width, height).
pub fn load_via_vips(input: &Path) -> Result<(i32, i32)> {
    let _app = vips_app(); // ensure init
    let img = VipsImage::new_from_file(
        input.to_str().ok_or_else(|| anyhow::anyhow!("non-UTF8 path: {:?}", input))?
    )?;
    Ok((img.get_width(), img.get_height()))
}

/// Convert an image from input to output using libvips.
/// libvips infers formats from file extensions.
pub fn convert_via_vips(input: &Path, output: &Path) -> Result<()> {
    let _app = vips_app(); // ensure init
    let img = VipsImage::new_from_file(
        input.to_str().ok_or_else(|| anyhow::anyhow!("non-UTF8 path: {:?}", input))?
    )?;
    ops::write_to_file(&img,
        output.to_str().ok_or_else(|| anyhow::anyhow!("non-UTF8 output path: {:?}", output))?
    )?;
    Ok(())
}
