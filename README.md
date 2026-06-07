# manatan-extension

Manatan-native extension package types and archive parsing.

This crate is the runtime-facing contract for `.manatan` extension packages. It is
designed to be shared with the public `manatan-rs` SDK so extension authors and
Manatan use the same manifest schema, content model, and WASM export names.

The format is media-first:

- `manga`: catalogs, chapter lists, and image/text pages
- `video`: shows, episodes, streams, subtitles, and hosters
- `novel`: works, chapter lists, and readable text/html

The contract includes typed filters, source preferences, host HTTP, per-source
storage, cookies, webview challenge hooks, richer video stream metadata, manga
page headers, and novel reading metadata.

## Examples

- `examples/basic`: one source that implements manga, video, and novel exports
- `examples/manga-source`: focused manga catalog, chapters, and pages
- `examples/video-source`: focused video catalog, episodes, streams, and subtitles
- `examples/novel-source`: focused novel catalog, chapters, and text/html

Build any example with:

```sh
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown --manifest-path examples/manga-source/Cargo.toml
```

Package an extension by zipping its `manifest.json` with the generated WASM as
`module.wasm`:

```sh
mkdir -p /tmp/manatan-manga
cp examples/manga-source/manifest.json /tmp/manatan-manga/manifest.json
cp examples/manga-source/target/wasm32-unknown-unknown/release/manatan_example_manga_source.wasm /tmp/manatan-manga/module.wasm
(cd /tmp/manatan-manga && zip -r manga-source.manatan manifest.json module.wasm)
```

This crate only describes the Manatan extension format.
