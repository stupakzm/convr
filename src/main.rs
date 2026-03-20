mod backends;
mod batch;
mod convert;
mod detect;
mod formats;
mod converters {
    pub mod data;
    pub mod document;
    pub mod image;
    #[cfg(feature = "heif")]
    pub mod image_heif;
    #[cfg(feature = "vips")]
    pub mod image_vips;
    #[cfg(feature = "mupdf-backend")]
    pub mod document_pdf;
}

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "convr", about = "Universal file converter", version)]
struct Cli {
    /// Target format (e.g. pdf, avif, yaml)
    #[arg(long, short)]
    to: String,

    /// Output directory (default: same as input)
    #[arg(long, short)]
    out: Option<PathBuf>,

    /// Input files (supports globs: *.png)
    inputs: Vec<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let target = formats::Format::from_name(&cli.to)
        .ok_or_else(|| anyhow::anyhow!("Unknown target format: {}", cli.to))?;

    let files = batch::expand_inputs(&cli.inputs)?;
    if files.is_empty() {
        anyhow::bail!("No input files found");
    }

    batch::run(files, target, cli.out)
}
