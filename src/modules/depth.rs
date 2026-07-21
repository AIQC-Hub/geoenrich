//! Bathymetric depth from a GEBCO gridded NetCDF file.
//!
//! Planned algorithm: GEBCO is a regular lon/lat grid, so no nearest-neighbor
//! search (the R code's `nn2`) is needed. Read the `lon`/`lat` axes once to learn
//! the origin and spacing, then map each point straight to the enclosing cell by
//! arithmetic (optionally bilinear-interpolating the four surrounding cells).
//! This is O(1) per point and exact to the grid. The `netcdf` crate (linking
//! HDF5, as ctddump already does) reads the `elevation` variable in windows so
//! the whole grid need not be resident.
//!
//! Sign convention: GEBCO elevation is negative below sea level. The default
//! output column reports depth as returned (negative under water); a `--positive`
//! option to flip the sign can be added when the algorithm lands.

use std::error::Error;
use std::path::PathBuf;

use crate::cli::DepthArgs;
use crate::config::{resolve, Settings};
use crate::pipeline::{run_module, Enricher, OutputKind, OutputSpec, Value};

pub struct DepthEnricher {
    #[allow(dead_code)]
    data: Option<PathBuf>,
    column: String,
}

impl Enricher for DepthEnricher {
    fn outputs(&self) -> Vec<OutputSpec> {
        Vec::from([OutputSpec {
            name: self.column.clone(),
            kind: OutputKind::Float,
        }])
    }

    fn enrich(&self, _lon: f64, _lat: f64) -> Vec<Value> {
        // TODO: index into the GEBCO grid and read the elevation cell.
        Vec::from([Value::Float(f64::NAN)])
    }
}

pub fn run(args: DepthArgs) -> Result<(), Box<dyn Error>> {
    // Depth is a grid lookup, so it needs no region box or projection.
    let s: Settings = resolve(&args.common, None)?;
    let df = crate::io::read_frame(&args.common.input, args.common.in_format)?;
    let out_path = args
        .common
        .output
        .clone()
        .unwrap_or_else(|| super::default_output(&args.common.input, "depth"));

    let enr = DepthEnricher {
        data: args.data,
        column: args.column,
    };

    super::stub_notice("depth", "NaN");
    run_module(&enr, df, &s, &out_path, args.common.out_format)
}
