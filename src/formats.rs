#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Format {
    // Images
    Png, Jpeg, Webp, Avif, Gif, Bmp, Tiff, Ico, Svg,
    // Documents
    Markdown, Html, Pdf, PlainText,
    // Data
    Json, Yaml, Toml, Csv, Xml,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Category {
    Image,
    Document,
    Data,
}

impl Format {
    pub fn from_extension(ext: &str) -> Option<Self> {
        Some(match ext.to_lowercase().as_str() {
            "png"             => Self::Png,
            "jpg" | "jpeg"   => Self::Jpeg,
            "webp"           => Self::Webp,
            "avif"           => Self::Avif,
            "gif"            => Self::Gif,
            "bmp"            => Self::Bmp,
            "tiff" | "tif"  => Self::Tiff,
            "ico"            => Self::Ico,
            "svg"            => Self::Svg,
            "md" | "markdown" => Self::Markdown,
            "html" | "htm"   => Self::Html,
            "pdf"            => Self::Pdf,
            "txt"            => Self::PlainText,
            "json"           => Self::Json,
            "yaml" | "yml"  => Self::Yaml,
            "toml"           => Self::Toml,
            "csv"            => Self::Csv,
            "xml"            => Self::Xml,
            _                => return None,
        })
    }

    pub fn from_name(name: &str) -> Option<Self> {
        Self::from_extension(name)
    }

    pub fn extension(&self) -> &'static str {
        match self {
            Self::Png       => "png",
            Self::Jpeg      => "jpg",
            Self::Webp      => "webp",
            Self::Avif      => "avif",
            Self::Gif       => "gif",
            Self::Bmp       => "bmp",
            Self::Tiff      => "tiff",
            Self::Ico       => "ico",
            Self::Svg       => "svg",
            Self::Markdown  => "md",
            Self::Html      => "html",
            Self::Pdf       => "pdf",
            Self::PlainText => "txt",
            Self::Json      => "json",
            Self::Yaml      => "yaml",
            Self::Toml      => "toml",
            Self::Csv       => "csv",
            Self::Xml       => "xml",
        }
    }

    pub fn category(&self) -> Category {
        match self {
            Self::Png | Self::Jpeg | Self::Webp | Self::Avif | Self::Gif
            | Self::Bmp | Self::Tiff | Self::Ico | Self::Svg => Category::Image,
            Self::Markdown | Self::Html | Self::Pdf | Self::PlainText => Category::Document,
            Self::Json | Self::Yaml | Self::Toml | Self::Csv | Self::Xml => Category::Data,
        }
    }
}
