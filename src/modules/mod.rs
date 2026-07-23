//! The four enrichment modules. Each builds an [`crate::pipeline::Enricher`] from
//! its data source and options, then hands off to `pipeline::run_module`.
//!
//! Implemented: `coast` (nearest GSHHG shoreline distance), `depth` (GEBCO grid
//! lookup), and `sea` (IHO Sea Areas point in polygon). The `place` lookup is
//! still a stub that emits empty values; its `run` prints a one-line notice so a
//! stub run is never mistaken for real data.

use std::error::Error;
use std::path::{Path, PathBuf};

use crate::geo::vector::Rings;

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

/// Read every polygon from a shapefile as (rings, attribute record) pairs,
/// skipping non-polygon shapes. Used by the sea and place modules; coast keeps
/// its own streaming read because it crops segments as it goes and never needs
/// whole rings in memory.
pub(crate) fn shp_polygons(
    path: &Path,
) -> Result<Vec<(Rings, shapefile::dbase::Record)>, Box<dyn Error>> {
    let mut reader = shapefile::Reader::from_path(path)
        .map_err(|e| format!("cannot read shapefile {}: {e}", path.display()))?;
    let mut out = Vec::new();
    for item in reader.iter_shapes_and_records() {
        let (shape, record) = item.map_err(|e| format!("reading {}: {e}", path.display()))?;
        if let shapefile::Shape::Polygon(poly) = shape {
            let rings: Rings = poly
                .rings()
                .iter()
                .map(|ring| ring.points().iter().map(|p| (p.x, p.y)).collect())
                .collect();
            out.push((rings, record));
        }
    }
    Ok(out)
}
