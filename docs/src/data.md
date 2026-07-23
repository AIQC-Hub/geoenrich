# Reference datasets

The datasets each command enriches from are large and are not bundled or
shipped. Each command takes its data path by flag (`--data`, or `--countries` /
`--municipalities` for `place`). The `nearest` command is the exception: its
reference data is a table you supply with `--to`, not a downloaded dataset.

## Download helper

`scripts/download_data.sh` fetches and unpacks any of the five sources into
`./data/`, one sub-directory per source, matching the example paths in the
command pages:

```bash
# GSHHG, GEBCO, Natural Earth, and GISCO need no details
scripts/download_data.sh download gshhg gebco countries lau

# the Marine Regions (IHO) download sits behind a short form
scripts/download_data.sh --mr-name "Your Name" --mr-email you@example.org \
  --mr-country Norway download iho
```

Selected datasets download in parallel. Existing archives are kept (`--force`
re-downloads), and the multi-GB GEBCO grid resumes an interrupted download. Run
`scripts/download_data.sh --help` for all options.

Caveats baked into the script:

- The GEBCO grid is multi-GB; the download resumes if interrupted.
- The GISCO LAU bundle nests one zip per projection; only the EPSG 4326
  (lon/lat) layer is unpacked, since the commands expect lon/lat.
- The Marine Regions (IHO) download submits the site's statistics form, so it
  requires `--mr-name`, `--mr-email`, and `--mr-country`. It verifies the
  response is a zip and fails loudly if the form rejects the request.

## Sources

| Dataset | Used by | Source |
|---------|---------|--------|
| GSHHG shorelines (ESRI shapefiles, resolution `f`) | `coast` | <https://www.soest.hawaii.edu/pwessel/gshhg/> |
| GEBCO bathymetry (gridded NetCDF) | `depth` | <https://www.gebco.net/> |
| IHO Sea Areas v3 (GeoJSON or shapefile) | `sea` | <https://www.marineregions.org/> |
| Natural Earth countries | `place` | <https://www.naturalearthdata.com/> |
| Eurostat GISCO LAU (municipalities) | `place` | <https://ec.europa.eu/eurostat/web/gisco> |
