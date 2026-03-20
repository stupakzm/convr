//! HEIF/HEIC decoding via libheif-rs (feature = "heif")

use anyhow::Result;
use std::path::Path;
use libheif_rs::{HeifContext, RgbChroma, ColorSpace};

/// Decode a HEIF/HEIC file into an image::DynamicImage for further conversion.
pub fn decode_heif(input: &Path) -> Result<image::DynamicImage> {
    let ctx = HeifContext::read_from_file(
        input.to_str().ok_or_else(|| anyhow::anyhow!("non-UTF8 path: {:?}", input))?
    )?;
    let handle = ctx.primary_image_handle()?;
    let img = handle.decode(ColorSpace::Rgb(RgbChroma::Rgb), None)?;
    let planes = img.planes();
    let interleaved = planes.interleaved
        .ok_or_else(|| anyhow::anyhow!("expected interleaved RGB plane from HEIF decoder"))?;
    let width = img.width();
    let height = img.height();
    let stride = interleaved.stride;
    // interleaved.data may have padding per row; copy only pixel bytes
    let mut data = Vec::with_capacity((width * height * 3) as usize);
    for row in 0..height {
        let start = (row as usize) * stride;
        let end = start + (width as usize * 3);
        data.extend_from_slice(&interleaved.data[start..end]);
    }
    image::RgbImage::from_raw(width, height, data)
        .map(image::DynamicImage::ImageRgb8)
        .ok_or_else(|| anyhow::anyhow!("RGB buffer size mismatch: {}x{}", width, height))
}
