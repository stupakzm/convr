use crate::{convert, detect, formats::Format};
use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub fn expand_inputs(inputs: &[String]) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for pattern in inputs {
        let matches: Vec<_> = glob::glob(pattern)?.filter_map(|e| e.ok()).collect();
        if matches.is_empty() {
            // Treat as literal path
            files.push(PathBuf::from(pattern));
        } else {
            files.extend(matches);
        }
    }
    Ok(files)
}

pub fn run(files: Vec<PathBuf>, target: Format, out_dir: Option<PathBuf>) -> Result<()> {
    let total = files.len();
    let pb = ProgressBar::new(total as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("=>-"),
    );

    let errors = Arc::new(AtomicUsize::new(0));
    let errors_clone = Arc::clone(&errors);

    files.par_iter().for_each(|input| {
        let Some(src_fmt) = detect::detect(input) else {
            pb.println(format!("  skip  {} (unknown format)", input.display()));
            pb.inc(1);
            return;
        };

        let output = resolve_output(input, &target, out_dir.as_deref());

        match convert::convert(input, &src_fmt, &output, &target) {
            Ok(_) => {
                pb.println(format!("  ok    {} -> {}", input.display(), output.display()));
            }
            Err(e) => {
                pb.println(format!("  error {} : {}", input.display(), e));
                errors_clone.fetch_add(1, Ordering::Relaxed);
            }
        }
        pb.inc(1);
    });

    pb.finish_and_clear();

    let err_count = errors.load(Ordering::Relaxed);
    if err_count > 0 {
        anyhow::bail!("{} file(s) failed to convert", err_count);
    }
    println!("Done. Converted {total} file(s).");
    Ok(())
}

fn resolve_output(input: &PathBuf, target: &Format, out_dir: Option<&std::path::Path>) -> PathBuf {
    let stem = input.file_stem().unwrap_or_default();
    let file_name = format!("{}.{}", stem.to_string_lossy(), target.extension());
    match out_dir {
        Some(dir) => dir.join(file_name),
        None => input.with_file_name(file_name),
    }
}
