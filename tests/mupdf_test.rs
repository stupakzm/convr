//! Integration tests for MuPDF backend (feature = "mupdf-backend")
//!
//! Run with: cargo test --features mupdf-backend --test mupdf_test

#[cfg(feature = "mupdf-backend")]
mod mupdf_tests {
    use std::path::Path;

    #[test]
    fn mupdf_build_smoke() {
        // If this compiles and runs, MuPDF linked successfully
        assert!(true, "mupdf-backend feature compiled and linked");
    }

    #[test]
    fn mupdf_open_pdf_fixture() {
        let fixture = Path::new("tests/fixtures/sample.pdf");
        if !fixture.exists() {
            eprintln!("SKIP: tests/fixtures/sample.pdf not found — add a real PDF to run open test");
            return;
        }
        let doc = mupdf::Document::open(fixture.to_str().unwrap());
        assert!(doc.is_ok(), "should open sample.pdf without error: {:?}", doc.err());
        let doc = doc.unwrap();
        let count = doc.page_count().expect("page_count should work");
        assert!(count > 0, "sample.pdf should have at least 1 page, got {}", count);
    }

    #[test]
    fn mupdf_render_pdf_to_png() {
        let fixture = Path::new("tests/fixtures/sample.pdf");
        if !fixture.exists() {
            eprintln!("SKIP: tests/fixtures/sample.pdf not found");
            return;
        }
        let output = std::env::temp_dir().join("convr_mupdf_test_render.png");
        let status = std::process::Command::new(env!("CARGO_BIN_EXE_convr"))
            .args(["--to", "png", fixture.to_str().unwrap(), "--out", output.parent().unwrap().to_str().unwrap()])
            .status()
            .expect("failed to run convr");
        let _ = std::fs::remove_file(&output);
        assert!(status.success(), "PDF to PNG render should succeed");
    }
}

#[cfg(not(feature = "mupdf-backend"))]
mod mupdf_disabled_tests {
    #[test]
    fn mupdf_feature_disabled_smoke() {
        assert!(true, "binary compiles without mupdf-backend feature");
    }
}
