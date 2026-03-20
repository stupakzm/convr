//! Integration test: all three C library backends enabled simultaneously.
//!
//! Run with: cargo test --features heif,vips,mupdf-backend --test all_backends_test
//!
//! This test exists specifically to catch duplicate symbol linker errors
//! when libheif, libvips, and MuPDF are all linked into the same binary.

#[cfg(all(feature = "heif", feature = "vips", feature = "mupdf-backend"))]
mod all_backends {
    #[test]
    fn all_features_link_without_conflict() {
        // The fact that this test compiles and runs proves there are
        // no duplicate symbol errors across the three C library backends
        assert!(true, "all three C backends linked without conflict");
    }

    #[test]
    fn all_backends_reported_available() {
        let status = std::process::Command::new(env!("CARGO_BIN_EXE_convr"))
            .args(["--version"])
            .output()
            .expect("failed to run convr");
        assert!(status.status.success(), "convr --version should succeed with all features");
    }

    #[test]
    fn heif_extension_recognized_with_all_features() {
        let output = std::process::Command::new(env!("CARGO_BIN_EXE_convr"))
            .args(["--to", "png", "nonexistent.heic"])
            .output()
            .expect("failed to run convr");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(!stderr.contains("Unknown target format"),
            ".heic should be recognized with all features; got: {}", stderr);
    }

    #[test]
    fn pdf_extension_recognized_with_all_features() {
        let output = std::process::Command::new(env!("CARGO_BIN_EXE_convr"))
            .args(["--to", "png", "nonexistent.pdf"])
            .output()
            .expect("failed to run convr");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(!stderr.contains("Unknown target format"),
            ".pdf should be recognized with all features; got: {}", stderr);
    }
}

#[cfg(not(all(feature = "heif", feature = "vips", feature = "mupdf-backend")))]
mod partial_features {
    #[test]
    fn partial_features_note() {
        eprintln!("NOTE: all_backends_test requires --features heif,vips,mupdf-backend");
        assert!(true);
    }
}
