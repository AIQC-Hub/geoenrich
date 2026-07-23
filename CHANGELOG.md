# Changelog

All notable changes to this project are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project aims to
follow [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `scripts/download_data.sh`: downloads and unpacks the five reference
  datasets into `data/`, one sub-directory per source, matching the README
  example paths. Selected datasets download in parallel; existing archives
  are kept (`--force` re-downloads) and the multi-GB GEBCO grid resumes an
  interrupted download. The Marine Regions (IHO) download submits the site's
  form, so it needs `--mr-name`, `--mr-email`, and `--mr-country`, and it
  fails loudly when the form rejects the request instead of leaving a broken
  archive. The GISCO LAU bundle's EPSG 4326 (lon/lat) shapefile is unpacked
  from its nested zip, since geoenrich needs lon/lat coordinates.

## [0.2.0] - 2026-07-23

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
- `place` module implemented against Natural Earth countries and, optionally,
  GISCO LAU municipalities: both lookups resolve a point by containment first
  and nearest boundary otherwise, appending `country`, `country_code` (ISO
  alpha-3 where available; the Natural Earth `-99` placeholder becomes null),
  and `municipality`. Attribute fields are auto-detected from candidate lists,
  so minor schema drift between dataset versions needs no flags.
- Shared vector-geometry helpers in `geo::vector`: point-to-segment distance,
  tagged R-tree segments, even-odd point in polygon, and a `PolygonIndex`
  combining containment with a nearest-boundary fallback.

### Changed

- `depth` now requires `--data <GEBCO NetCDF file>`, `coast` and `sea` require
  `--data`, and `place` requires `--countries`; each errors clearly when its
  data source is omitted. With every module implemented, the scaffold stub
  notice is gone. `--municipalities` stays optional: without it the
  `municipality` column is empty and a note says so.

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
