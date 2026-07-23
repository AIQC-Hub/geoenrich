# Installation

## Build from source

geoenrich is a Rust project, so a recent stable toolchain is all you need to
build the binary:

```bash
git clone https://github.com/AIQC-Hub/geoenrich
cd geoenrich
cargo build --release
# binary at target/release/geoenrich
```

## System dependencies

Only the [`depth`](./commands/depth.md) command needs anything beyond the Rust
toolchain: it reads GEBCO NetCDF and links the HDF5 / NetCDF C libraries, so you
need their development headers (the same system dependency as `ctddump`):

```bash
# Ubuntu / Debian
sudo apt-get install libhdf5-dev libnetcdf-dev

# macOS
brew install hdf5
```

The other four commands (`coast`, `sea`, `place`, `nearest`) use only pure-Rust
geometry and have no system dependencies.

## Reference data

The datasets each command enriches from (shorelines, bathymetry, sea polygons,
country and municipality boundaries) are large and are not bundled. Download the
ones you need with the helper script, described under
[Reference datasets](./data.md).

## Check it works

```bash
geoenrich --help
geoenrich coast --help
```

Every command is self-documenting through `--help` at each level.
