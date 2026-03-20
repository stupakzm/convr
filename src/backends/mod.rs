/// Reports which C library backends are available at compile time.

pub struct BackendInfo {
    pub name: &'static str,
    pub available: bool,
}

pub fn available_backends() -> Vec<BackendInfo> {
    vec![
        BackendInfo {
            name: "heif",
            available: cfg!(feature = "heif"),
        },
        BackendInfo {
            name: "vips",
            available: cfg!(feature = "vips"),
        },
        BackendInfo {
            name: "mupdf",
            available: cfg!(feature = "mupdf-backend"),
        },
    ]
}

#[cfg(feature = "vips")]
mod vips_init {
    use libvips::VipsApp;
    use std::sync::OnceLock;

    static VIPS: OnceLock<VipsApp> = OnceLock::new();

    pub fn vips_app() -> &'static VipsApp {
        VIPS.get_or_init(|| {
            VipsApp::new("convr", false).expect("libvips initialization failed")
        })
    }
}

#[cfg(feature = "vips")]
pub use vips_init::vips_app;
