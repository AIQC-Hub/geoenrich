//! Pure-Rust geometry: no PROJ / GDAL, so consuming projects need no extra
//! system libraries. Currently a spherical LAEA projection (for planar distances
//! in a region) and a great-circle distance. Vector-geometry helpers (point in
//! polygon, nearest feature) will land here alongside `geo` + `rstar` when the
//! coast/sea/place algorithms are implemented.

pub mod projection;

pub use projection::{haversine_m, Laea};
