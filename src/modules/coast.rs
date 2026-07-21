//! Distance to the nearest coast, from GSHHG shoreline polygons.
//!
//! Planned algorithm (pure Rust, no PROJ):
//!   1. Read the GSHHG shapefiles (resolution `f`), keep polygons, make valid.
//!   2. Crop to the region box (filter, do not slice, so no artificial edges).
//!   3. Project polygon boundary segments through the region LAEA and index them
//!      in an `rstar` R-tree.
//!   4. Per location: project the point, query the R-tree for the nearest
//!      segments, and take the minimum planar distance (Snyder LAEA meters).
//! This mirrors the R workflow's projected-distance approach without GDAL.

use std::error::Error;
use std::path::PathBuf;

use crate::cli::{CoastArgs, DistUnit};
use crate::config::{resolve, Settings};
use crate::geo::Laea;
use crate::pipeline::{run_module, Enricher, OutputKind, OutputSpec, Value};

pub struct CoastEnricher {
    #[allow(dead_code)]
    data: Option<PathBuf>,
    unit: DistUnit,
    column: String,
    // Region projection for planar distances (used once the index is built).
    #[allow(dead_code)]
    proj: Laea,
}

impl Enricher for CoastEnricher {
    fn outputs(&self) -> Vec<OutputSpec> {
        Vec::from([OutputSpec {
            name: self.column.clone(),
            kind: OutputKind::Float,
        }])
    }

    fn enrich(&self, _lon: f64, _lat: f64) -> Vec<Value> {
        // TODO: nearest-segment distance via the LAEA + rstar index.
        let _ = self.unit;
        Vec::from([Value::Float(f64::NAN)])
    }
}

pub fn run(args: CoastArgs) -> Result<(), Box<dyn Error>> {
    let s: Settings = resolve(&args.common, Some(&args.region))?;
    let df = crate::io::read_frame(&args.common.input, args.common.in_format)?;
    let out_path = args
        .common
        .output
        .clone()
        .unwrap_or_else(|| super::default_output(&args.common.input, "coast"));

    let enr = CoastEnricher {
        data: args.data,
        unit: args.unit,
        column: args.column,
        proj: Laea::new(s.proj_lon0, s.proj_lat0),
    };

    super::stub_notice("coast", "NaN");
    run_module(&enr, df, &s, &out_path, args.common.out_format)
}
