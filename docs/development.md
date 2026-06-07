# Manatan Extension Development

This is the recommended local loop for building `.manatan` extensions with the
Rust SDK.

## Quick Loop

Install the WASM target once:

```sh
rustup target add wasm32-unknown-unknown
```

Build and package an extension from its crate directory:

```sh
cargo run --bin manatan-dev -- package examples/manga-source
```

Verify one or more generated packages:

```sh
cargo run --bin manatan-dev -- verify examples/manga-source/example-manga.manatan
```

Call an export directly through the SDK runner:

```sh
cargo run --bin manatan-dev -- call examples/manga-source/example-manga.manatan manatan_manga_get_list '{}'
```

Run the default list smoke call for the package media kind:

```sh
cargo run --bin manatan-dev -- smoke examples/manga-source/example-manga.manatan
```

## What The Tool Checks

`manatan-dev verify` uses the same archive parser and manifest validation that
the runtime uses. A valid package must include:

- `manifest.json`
- `module.wasm`
- optional `filters.json`
- optional `preferences.json`
- optional `assets/...`

The manifest must use the current schema version, declare a single
`contentType`, and every source inside the package must match that media kind.

## Endgame Dev-Ex Targets

The strongest source-development loop is:

- `manatan-dev new`: scaffold a manga, video, or novel extension from a template.
- `manatan-dev package`: build `wasm32-unknown-unknown` and create `.manatan`.
- `manatan-dev verify`: validate package structure, manifest fields, and exports.
- `manatan-dev smoke`: run representative exports outside the app.
- `manatan-dev serve`: serve a local extension index for Manatan to install from.
- `manatan-dev logcat`: stream extension logs from the app during device testing.
- `manatan-test`: a custom WASM test runner so authors can write Rust tests that
  execute inside a Manatan-like host environment.

The current SDK includes the core pieces needed for this path: package parsing,
manifest validation, and a Wasmer-backed extension runner.
