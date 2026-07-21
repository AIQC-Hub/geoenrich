# Changelog

All notable changes to this project are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project aims to
follow [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
