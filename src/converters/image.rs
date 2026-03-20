use crate::formats::Format;
use anyhow::Result;
use std::path::Path;

pub fn convert(input: &Path, src: &Format, output: &Path, target: &Format) -> Result<()> {
    // HEIF/HEIC source — use libheif
    if *src == Format::Heif {
        #[cfg(feature = "heif")]
        {
            let img = super::image_heif::decode_heif(input)?;
            if *target == Format::Avif {
                let rgba = img.to_rgba8();
                let (width, height) = rgba.dimensions();
                let pixels: Vec<rgb::RGBA8> = rgba.pixels()
                    .map(|p| rgb::RGBA8 { r: p[0], g: p[1], b: p[2], a: p[3] })
                    .collect();
                let encoder = ravif::Encoder::new().with_quality(80.0).with_speed(4);
                let result = encoder.encode_rgba(ravif::Img::new(&pixels, width as usize, height as usize))?;
                std::fs::write(output, result.avif_file)?;
                return Ok(());
            }
            img.save(output)?;
            if *target == Format::Png {
                let opts = oxipng::Options::from_preset(3);
                oxipng::optimize(
                    &oxipng::InFile::Path(output.to_path_buf()),
                    &oxipng::OutFile::Path { path: Some(output.to_path_buf()), preserve_attrs: false },
                    &opts,
                )?;
            }
            return Ok(());
        }
        #[cfg(not(feature = "heif"))]
        anyhow::bail!("HEIF support requires building with --features heif");
    }

    // SVG source — use resvg
    if *src == Format::Svg {
        return svg_to_raster(input, output, target);
    }

    // AVIF target — use ravif
    if *target == Format::Avif {
        return to_avif(input, output);
    }

    // PNG target with oxipng optimization
    if *target == Format::Png {
        let img = image::open(input)?;
        img.save(output)?;
        // Optimize in place
        let opts = oxipng::Options::from_preset(3);
        oxipng::optimize(
            &oxipng::InFile::Path(output.to_path_buf()),
            &oxipng::OutFile::Path { path: Some(output.to_path_buf()), preserve_attrs: false },
            &opts,
        )?;
        return Ok(());
    }

    // All other raster ↔ raster: delegate to the `image` crate
    let img = image::open(input)?;
    img.save(output)?;
    Ok(())
}

fn svg_to_raster(input: &Path, output: &Path, target: &Format) -> Result<()> {
    let data = std::fs::read(input)?;
    let options = resvg::usvg::Options::default();
    let tree = resvg::usvg::Tree::from_data(&data, &options)?;
    let size = tree.size().to_int_size();
    let mut pixmap = tiny_skia::Pixmap::new(size.width(), size.height())
        .ok_or_else(|| anyhow::anyhow!("Failed to create pixmap"))?;
    resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());

    match target {
        Format::Png => {
            pixmap.save_png(output)?;
        }
        _ => {
            // Save as PNG bytes then re-encode via `image`
            let png_bytes = pixmap.encode_png()?;
            let img = image::load_from_memory(&png_bytes)?;
            img.save(output)?;
        }
    }
    Ok(())
}

fn to_avif(input: &Path, output: &Path) -> Result<()> {
    let img = image::open(input)?.to_rgba8();
    let (width, height) = img.dimensions();
    let pixels: Vec<rgb::RGBA8> = img
        .pixels()
        .map(|p| rgb::RGBA8 { r: p[0], g: p[1], b: p[2], a: p[3] })
        .collect();

    let encoder = ravif::Encoder::new()
        .with_quality(80.0)
        .with_speed(4);
    let result = encoder.encode_rgba(ravif::Img::new(&pixels, width as usize, height as usize))?;
    std::fs::write(output, result.avif_file)?;
    Ok(())
}
