//! Integration tests for libvips backend (feature = "vips")
//!
//! Run with: cargo test --features vips --test vips_test

#[cfg(feature = "vips")]
mod vips_tests {
    use std::path::Path;

    #[test]
    fn vips_build_smoke() {
        // If this compiles and runs, libvips linked successfully
        assert!(true, "vips feature compiled and linked");
    }

    #[test]
    fn vips_convert_png_to_jpeg() {
        // Create a minimal PNG test image using the image crate
        let tmp_dir = std::env::temp_dir().join("convr_vips_test");
        let _ = std::fs::create_dir_all(&tmp_dir);
        let input = tmp_dir.join("test_input.png");
        let output = tmp_dir.join("test_output.jpg");

        // Generate a 10x10 red PNG
        let img = image::RgbImage::from_fn(10, 10, |_, _| image::Rgb([255u8, 0, 0]));
        img.save(&input).expect("failed to save test PNG");

        // Convert via convr binary
        let status = std::process::Command::new(env!("CARGO_BIN_EXE_convr"))
            .args(["--to", "jpg", input.to_str().unwrap(), "--out", tmp_dir.to_str().unwrap()])
            .status()
            .expect("failed to run convr");

        assert!(status.success(), "PNG to JPEG via vips should succeed");
        assert!(output.exists(), "output JPEG should exist at {:?}", output);

        // Clean up
        let _ = std::fs::remove_dir_all(&tmp_dir);
    }
}

#[cfg(not(feature = "vips"))]
mod vips_disabled_tests {
    #[test]
    fn vips_feature_disabled_smoke() {
        assert!(true, "binary compiles without vips feature");
    }
}
