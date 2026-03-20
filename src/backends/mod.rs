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
