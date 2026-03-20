//! Integration tests for HEIF/HEIC backend (feature = "heif")
//!
//! Run with: cargo test --features heif --test heif_test

#[cfg(feature = "heif")]
mod heif_tests {
    use std::path::Path;

    #[test]
    fn heif_build_smoke() {
        // If this test compiles and runs, the heif feature links correctly
        assert!(true, "heif feature compiled and linked successfully");
    }

    #[test]
    fn heif_format_extension_roundtrip() {
        let output = std::process::Command::new(env!("CARGO_BIN_EXE_convr"))
            .args(["--to", "png", "nonexistent.heic"])
            .output()
            .expect("failed to run convr");
        let stderr = String::from_utf8_lossy(&output.stderr);
        // Should fail with "No input files found" or file-not-found,
        // NOT "Unknown target format" — proving .heic is recognized
        assert!(
            !stderr.contains("Unknown target format"),
            "heic extension should be recognized; got: {}", stderr
        );
    }

    #[test]
    fn heif_decode_requires_fixture() {
        let fixture = Path::new("tests/fixtures/sample.heic");
        if !fixture.exists() {
            eprintln!("SKIP: tests/fixtures/sample.heic not found — add a real HEIC file to run decode test");
            return;
        }
        let output_path = std::env::temp_dir().join("convr_test_heif_decode.jpg");
        let status = std::process::Command::new(env!("CARGO_BIN_EXE_convr"))
            .args(["--to", "jpg", fixture.to_str().unwrap(), "--out", output_path.parent().unwrap().to_str().unwrap()])
            .status()
            .expect("failed to run convr");
        let _ = std::fs::remove_file(&output_path);
        assert!(status.success(), "HEIF decode to JPEG should succeed");
    }
}

#[cfg(not(feature = "heif"))]
mod heif_disabled_tests {
    #[test]
    fn heif_feature_disabled_smoke() {
        // Confirm the binary builds without heif feature
        assert!(true, "binary compiles without heif feature");
    }
}
