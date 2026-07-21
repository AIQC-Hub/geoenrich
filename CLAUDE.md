# CLAUDE.md

Guidance for Claude Code when working in this repository.

## Project Overview

`geoenrich` is a Rust CLI that enriches a table of `longitude`/`latitude` points
with four geospatial attributes, one per top-level command:

- `coast`: distance to the nearest shoreline (GSHHG shorelines).
- `depth`: bathymetric depth at the point (GEBCO gridded bathymetry).
- `sea`: sea / ocean name (IHO Sea Areas, point in polygon).
- `place`: nearest country and municipality (Natural Earth + Eurostat GISCO).

It is a sibling to `ctddump` and follows the same house style, but is a separate
package on purpose: it must stay light and reusable across several downstream
projects, so it does not depend on `ctddump` and adds spatial dependencies only
as each algorithm needs them.

Input and output can be Parquet (default), CSV, TSV, and the gzip variants
`csv.gz` / `tsv.gz`. Every module reads the input, reduces it to unique locations
with rounded coordinates (3 decimals by default), enriches those unique locations
in parallel, then joins the results back onto every input row.

## Documentation style

Do not use em dashes in any human-facing text: `README.md`, `CHANGELOG.md`, docs,
generated output, help text, and log lines. Use a colon, comma, parentheses, a
semicolon, or a reworded sentence instead. (Carried over from `ctddump`.)

## Scaffold status

This repository was scaffolded design-first. What is complete and what is stubbed:

- Complete: the CLI, config resolution, multi-format I/O, and the shared pipeline
  (`pipeline::run_module`) that de-duplicates locations, parallelizes, joins, and
  writes. The pure-Rust LAEA projection and great-circle distance in
  `src/geo/projection.rs` are implemented and unit-tested. The dummy-enricher
  integration test in `tests/pipeline.rs` exercises the whole path.
- Implemented: the `depth` module (`src/modules/depth.rs`), a GEBCO NetCDF grid
  lookup keyed on `netcdf` (linking system HDF5). Nearest-cell by arithmetic,
  serialized reads under a mutex (the system HDF5 serial build is not thread
  safe), per-thread HDF5 diagnostic silencing, and a `tests/depth.rs` integration
  test that builds a small synthetic grid.
- Stubbed: the `coast`, `sea`, and `place` per-location spatial lookups. Each
  `enrich` returns NaN (coast) or empty (sea, place) and each `run` prints a
  one-line notice so a stub run is never mistaken for real data. The remaining
  `Cargo.toml` spatial dependencies (`geo`, `rstar`, `shapefile`, `geojson`) are
  commented out until the algorithm that needs them is written.

Each module file's header comment states the planned algorithm.

## Commands

```bash
cargo build
cargo test                         # unit + integration tests
cargo run -- <command> --help      # discover any command's interface
cargo run -- coast input.parquet --data ./data/gshhg/.../GSHHS_shp/f
```

The full CLI is defined with `clap` in `src/cli.rs` and is self-documenting via
`--help` at every level.

## Architecture

Single-stage `clap` dispatch:

1. `src/cli.rs`: the `Cli` / `Commands` structure and the flattened `CommonArgs`
   (input, output, format, columns, decimals, threads) and `RegionArgs`
   (bounding box + projection center) shared across commands.
2. `src/lib.rs`: `run(cli)` matches the command and calls the module's `run`.
3. `src/config.rs`: `resolve(common, region)` merges the built-in default, the
   optional TOML config, and the CLI flags into a `Settings`. Precedence for the
   region box / projection center is `preset/default < config file < CLI flag`.

**Pipeline** (`src/pipeline.rs`): the `Enricher` trait is the entire per-module
surface. A module declares its `outputs()` (column name + `Float`/`Text`) and
computes `enrich(lon, lat) -> Vec<Value>`. `run_module` does the rest: extract
`lon`/`lat` (cast to f64, nulls to NaN), round and de-duplicate into unique
locations (integer-scaled keys, so the join never compares floats), enrich the
unique set with rayon, expand the results back to one value per input row, hstack
the new columns, and write. NaN coordinates get no key and therefore null output.

**I/O** (`src/io.rs`): `resolve_format` infers the format from the extension
(Parquet fallback); `read_frame` / `write_frame` handle all five formats. Gzip is
done with `flate2` (decompress to memory on read, wrap the writer on write) rather
than a Polars feature, so it behaves the same across Polars versions. Parquet
writes use `set_parallel(false)` for the same reason as ctddump.

**Geometry** (`src/geo/`): pure Rust, no PROJ / GDAL, so downstream projects need
no extra system libraries. `Laea` is a spherical Lambert Azimuthal Equal-Area
projection centered on the region (the reference R workflow used EPSG:3035 LAEA
for distances); planar distance in that projection is accurate for the
nearest-coast query at regional scale. `haversine_m` is the great-circle distance
used for reference and for refining index candidates. Sub-meter accuracy, if ever
needed, means an ellipsoidal LAEA in place of the spherical one.

**Modules** (`src/modules/`): `coast`, `depth`, `sea`, `place`. Each builds its
`Enricher` from a data-source path and options and calls `run_module`. Shared
helpers: `default_output` (the `<stem>.<tag>.parquet` fallback) and `stub_notice`.

## Data sources (not bundled)

The reference datasets are large and are not committed or shipped. A module takes
its data path by flag (`--data`, or `--countries` / `--municipalities` for
`place`). Sources:

- GSHHG shorelines: https://www.soest.hawaii.edu/pwessel/gshhg/ (ESRI shapefiles,
  resolution `f`).
- GEBCO bathymetry: https://www.gebco.net/ (gridded NetCDF; the depth module
  links HDF5 via the `netcdf` crate, same system dependency as ctddump).
- IHO Sea Areas v3: Marine Regions (https://www.marineregions.org/), GeoJSON or
  shapefile.
- Natural Earth countries: https://www.naturalearthdata.com/.
- Eurostat GISCO LAU (municipalities): https://ec.europa.eu/eurostat/web/gisco.

A `scripts/` directory with download helpers (in the ctddump style) can be added
once the modules read real data.

## Regions

Defaults target the Baltic Sea (box 8, 31, 53, 66), matching the R examples.
Other regions come from `--region` presets (`baltic`, `norway`, `global`) or
explicit `--min-lon/--max-lon/--min-lat/--max-lat` and `--proj-lon0/--proj-lat0`.
Add presets in `config::preset_bbox`. The `place` municipality lookup will also
need a per-region country list (the R snippet's ISO3 set).

## Streaming (future)

`read_frame` currently loads the whole input to memory because the join touches
every row; the enrichment set itself is only the unique locations, so it is
always small. For very large inputs, the ctddump pattern applies: a first pass to
collect unique locations, then a second streamed pass that appends columns
`chunk`-by-`chunk` via a `BatchedWriter`. Note that as a caveat in the module docs
before implementing it.

## Git Workflow

Match ctddump: permanent `main` (stable) and `develop` (integration) branches;
day-to-day work on `develop`. Commit and push only when the user asks. Commit
messages end with `Co-Authored-By: Claude Opus 4.8 <noreply@anthropic.com>`.
