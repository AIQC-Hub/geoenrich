//! Nearest country and municipality for an offshore point.
//!
//! Points at sea sit inside no land polygon, so both lookups are nearest-feature:
//!   - Country: Natural Earth country polygons; assign the nearest one. Appends
//!     `country` and `country_code`.
//!   - Municipality: GISCO LAU polygons; assign the nearest one. Appends
//!     `municipality`. Because LAU is large, it is loaded only for the countries
//!     overlapping the region box (a preset list per region, like the R snippet's
//!     ISO3 set).
//!
//! Planned algorithm: project boundaries through the region LAEA, index segments
//! in an `rstar` R-tree, and take the nearest by planar distance. The country and
//! municipality lookups share that machinery over two polygon sets.

use std::error::Error;
use std::path::PathBuf;

use crate::cli::PlaceArgs;
use crate::config::{resolve, Settings};
use crate::geo::Laea;
use crate::pipeline::{run_module, Enricher, OutputKind, OutputSpec, Value};

pub struct PlaceEnricher {
    #[allow(dead_code)]
    countries: Option<PathBuf>,
    #[allow(dead_code)]
    municipalities: Option<PathBuf>,
    #[allow(dead_code)]
    proj: Laea,
}

impl Enricher for PlaceEnricher {
    fn outputs(&self) -> Vec<OutputSpec> {
        Vec::from([
            OutputSpec { name: "country".into(), kind: OutputKind::Text },
            OutputSpec { name: "country_code".into(), kind: OutputKind::Text },
            OutputSpec { name: "municipality".into(), kind: OutputKind::Text },
        ])
    }

    fn enrich(&self, _lon: f64, _lat: f64) -> Vec<Value> {
        // TODO: nearest country polygon, then nearest LAU polygon.
        Vec::from([Value::Text(None), Value::Text(None), Value::Text(None)])
    }
}

pub fn run(args: PlaceArgs) -> Result<(), Box<dyn Error>> {
    let s: Settings = resolve(&args.common, Some(&args.region))?;
    let df = crate::io::read_frame(&args.common.input, args.common.in_format)?;
    let out_path = args
        .common
        .output
        .clone()
        .unwrap_or_else(|| super::default_output(&args.common.input, "place"));

    let enr = PlaceEnricher {
        countries: args.countries,
        municipalities: args.municipalities,
        proj: Laea::new(s.proj_lon0, s.proj_lat0),
    };

    super::stub_notice("place", "empty");
    run_module(&enr, df, &s, &out_path, args.common.out_format)
}
