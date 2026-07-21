//! Sea / ocean name from IHO Sea Areas polygons (point in polygon).
//!
//! Planned algorithm:
//!   1. Read the IHO Sea Areas (Marine Regions GeoJSON or shapefile), make valid.
//!   2. Index polygon bounding boxes in an `rstar` R-tree.
//!   3. Per location: query candidate polygons and test containment with
//!      `geo::Contains`. If a point falls just inland (common in narrow fjords),
//!      fall back to the nearest polygon by planar (region-LAEA) distance.
//! Point-in-polygon in lon/lat is fine here: sea boundaries are coarse relative
//! to the rounding, so no projection is needed for the containment test itself.

use std::error::Error;
use std::path::PathBuf;

use crate::cli::SeaArgs;
use crate::config::{resolve, BBox, Settings};
use crate::pipeline::{run_module, Enricher, OutputKind, OutputSpec, Value};

pub struct SeaEnricher {
    #[allow(dead_code)]
    data: Option<PathBuf>,
    column: String,
    // Region box, used to crop the polygon set at load time.
    #[allow(dead_code)]
    bbox: BBox,
}

impl Enricher for SeaEnricher {
    fn outputs(&self) -> Vec<OutputSpec> {
        Vec::from([OutputSpec {
            name: self.column.clone(),
            kind: OutputKind::Text,
        }])
    }

    fn enrich(&self, _lon: f64, _lat: f64) -> Vec<Value> {
        // TODO: point-in-polygon against IHO areas, nearest-polygon fallback.
        Vec::from([Value::Text(None)])
    }
}

pub fn run(args: SeaArgs) -> Result<(), Box<dyn Error>> {
    let s: Settings = resolve(&args.common, Some(&args.region))?;
    let df = crate::io::read_frame(&args.common.input, args.common.in_format)?;
    let out_path = args
        .common
        .output
        .clone()
        .unwrap_or_else(|| super::default_output(&args.common.input, "sea"));

    let enr = SeaEnricher {
        data: args.data,
        column: args.column,
        bbox: s.bbox,
    };

    super::stub_notice("sea", "empty");
    run_module(&enr, df, &s, &out_path, args.common.out_format)
}
