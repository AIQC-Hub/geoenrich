# Changelog

All notable changes to this project are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project aims to
follow [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `depth` module implemented against GEBCO gridded NetCDF: reads the `lat`/`lon`
  axes once, then maps each point to its nearest grid cell by arithmetic (O(1),
  no nearest-neighbor search) and reads the single `elevation` cell. Longitudes
  are normalized to `[-180, 180)` and off-grid points yield NaN. New `--positive`
  flag reports depth as positive below sea level. Reads link the system HDF5 /
  NetCDF libraries (`netcdf` crate).
- `coast` module implemented against GSHHG shorelines: boundary segments of the
  L1 (land/ocean) shapefile are cropped to the region box plus a 5 degree
  margin, projected through the region LAEA, and indexed in an `rstar` R-tree;
  each point gets the planar distance to the nearest segment in the chosen unit
  (`--unit km|m`). Segments are dropped, never clipped, so cropping cannot
  create artificial shoreline. `--data` accepts the `GSHHS_*_L1.shp` file or a
  GSHHG resolution directory containing one.
- `sea` module implemented against IHO Sea Areas (Marine Regions GeoJSON or
  shapefile): features are cropped whole to the region box plus margin, feature
  bounding boxes are indexed in an R-tree, and each point is resolved by an
  even-odd point-in-polygon test with a nearest-boundary fallback for points
  that fall just inland (fjords). New `--name-field` flag selects the name
  property (default `NAME`).
- Shared vector-geometry helpers in `geo::vector`: point-to-segment distance,
  tagged R-tree segments, even-odd point in polygon, and a `PolygonIndex`
  combining containment with a nearest-boundary fallback.

### Changed

- `depth` now requires `--data <GEBCO NetCDF file>` and `coast` and `sea`
  require `--data`; each errors clearly when its data source is omitted,
  instead of printing the scaffold stub notice and emitting NaN or empty
  values.

## [0.1.0] - 2026-07-22

### Added

- Project scaffold: `coast`, `depth`, `sea`, and `place` commands sharing one
  pipeline (read, de-duplicate rounded locations, enrich in parallel, join back,
  write).
- Multi-format I/O: Parquet (default), CSV, TSV, `csv.gz`, `tsv.gz`.
- Pure-Rust geometry: spherical LAEA projection and great-circle distance, with
  unit tests. No PROJ / GDAL dependency.
- Config resolution with a Baltic default, `--region` presets, and an optional
  TOML config file overridden by CLI flags.

### Not yet implemented

- The four modules' spatial lookups are stubs that emit NaN / empty values and
  print a notice; the reference-data readers (GSHHG, GEBCO, IHO, Natural Earth,
  GISCO) and their spatial indexes are pending.
