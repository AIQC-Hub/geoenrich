# geoenrich

`geoenrich` is a Rust command-line tool that adds geospatial attributes to a
table of points. Give it a file with `longitude` and `latitude` columns and it
appends any of:

- **coast**: distance to the nearest shoreline (GSHHG).
- **depth**: bathymetric depth at the point (GEBCO).
- **sea**: the sea or ocean name at the point (IHO Sea Areas).
- **place**: the nearest country and municipality (Natural Earth + GISCO).

It reads and writes Parquet (default), CSV, TSV, and the gzip variants `csv.gz`
and `tsv.gz`. Each command reduces the input to unique rounded locations,
processes those in parallel, and joins the results back onto every row, so a file
with millions of rows but few distinct positions is cheap to enrich.

> **Status:** in progress. The CLI, I/O, config, parallel pipeline, and the
> pure-Rust projection are complete and tested. The `depth` (GEBCO grid lookup)
> and `coast` (nearest GSHHG shoreline by projected R-tree lookup) modules are
> implemented. The `sea` and `place` lookups are still stubbed and emit empty
> values (each run prints a notice). See `CLAUDE.md` for what is implemented
> and the planned algorithm per module.

## Install

```bash
cargo build --release
# binary at target/release/geoenrich
```

The `depth` command reads GEBCO NetCDF and links the HDF5 / NetCDF C libraries,
so you need the dev headers (as with ctddump):

```bash
# Ubuntu / Debian
sudo apt-get install libhdf5-dev libnetcdf-dev
# macOS
brew install hdf5
```

## Usage

```bash
geoenrich <command> <input> [options]
```

Every command shares these options:

| Option | Default | Meaning |
|--------|---------|---------|
| `-o, --output <FILE>` | `<stem>.<command>.parquet` | Output file |
| `--in-format <FMT>` | inferred, else parquet | `parquet`, `csv`, `tsv`, `csv.gz`, `tsv.gz`, `auto` |
| `--out-format <FMT>` | inferred, else parquet | same set |
| `--lon-col <NAME>` | `longitude` | Longitude column |
| `--lat-col <NAME>` | `latitude` | Latitude column |
| `--decimals <N>` | `3` | Rounding applied before de-duplicating |
| `-t, --threads <N>` | all cores | Worker threads |
| `-c, --config <TOML>` | none | Config file (CLI flags override it) |

The `coast`, `sea`, and `place` commands also take region options: a `--region`
preset (`baltic`, `norway`, `global`) or explicit `--min-lon/--max-lon/--min-lat/
--max-lat`, plus `--proj-lon0/--proj-lat0` for the distance projection center.
Defaults target the Baltic Sea.

### Examples

```bash
# Distance to coast, GSHHG resolution 'f', result in kilometers
geoenrich coast cores.parquet \
  --data ./data/gshhg/gshhg-shp-2.3.7/GSHHS_shp/f \
  --unit km -o cores.coast.parquet

# Bathymetric depth from a GEBCO grid, reading and writing gzipped CSV
geoenrich depth cores.csv.gz --data ./data/gebco/GEBCO_2024_sub_ice.nc \
  -o cores.depth.csv.gz

# Sea name, for the Norway region instead of the Baltic default
geoenrich sea cores.parquet --region norway \
  --data ./data/iho/iho_sea_areas.geojson

# Nearest country and municipality
geoenrich place cores.parquet \
  --countries ./data/naturalearth/ne_10m_admin_0_countries.shp \
  --municipalities ./data/gisco/lau.shp
```

Run `geoenrich <command> --help` for the full interface.

## Output columns

| Command | Appended columns |
|---------|------------------|
| `coast` | `dist_to_coast` (rename with `--column`) |
| `depth` | `bathymetry` (rename with `--column`) |
| `sea`   | `sea_name` (rename with `--column`) |
| `place` | `country`, `country_code`, `municipality` |

## Data

The reference datasets are downloaded separately (they are large and not bundled):

- GSHHG shorelines: https://www.soest.hawaii.edu/pwessel/gshhg/
- GEBCO bathymetry: https://www.gebco.net/
- IHO Sea Areas: https://www.marineregions.org/
- Natural Earth: https://www.naturalearthdata.com/
- Eurostat GISCO LAU: https://ec.europa.eu/eurostat/web/gisco

## License

MIT.
