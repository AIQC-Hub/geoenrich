//! The four enrichment modules. Each builds an [`crate::pipeline::Enricher`] from
//! its data source and options, then hands off to `pipeline::run_module`.
//!
//! Scaffold status: the shared pipeline (read, de-duplicate, parallelize, join,
//! write) is complete and exercised by `tests/`. The per-location spatial lookups
//! are stubs that emit NaN / empty until the algorithms are implemented; each
//! `run` prints a one-line notice so a stub run is never mistaken for real data.

use std::path::{Path, PathBuf};

pub mod coast;
pub mod depth;
pub mod place;
pub mod sea;

/// Default output path when `--output` is omitted: `<stem>.<tag>.parquet` beside
/// the input.
pub(crate) fn default_output(input: &Path, tag: &str) -> PathBuf {
    let stem = input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    let name = format!("{stem}.{tag}.parquet");
    match input.parent() {
        Some(dir) if !dir.as_os_str().is_empty() => dir.join(name),
        _ => PathBuf::from(name),
    }
}

/// Printed once per stubbed module so scaffold output is clearly labeled.
pub(crate) fn stub_notice(module: &str, emits: &str) {
    eprintln!(
        "[geoenrich] {module}: scaffold stub, emitting {emits} until the algorithm is implemented"
    );
}
