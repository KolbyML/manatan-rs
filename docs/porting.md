# Porting Sources to Manatan Extensions

Manatan extensions are Rust/WASM packages. A port usually maps one old source
class or plugin file to one Rust source module.

## Common Mapping

| Source concept | Manatan export |
| --- | --- |
| Popular/latest list | `manatan_manga_get_list`, `manatan_video_get_list`, `manatan_novel_get_list` |
| Search | `manatan_manga_search`, `manatan_video_search`, `manatan_novel_search` |
| Item details | `manatan_manga_get_details`, `manatan_video_get_details`, `manatan_novel_get_details` |
| Manga chapters | `manatan_manga_get_chapters` |
| Manga pages | `manatan_manga_get_pages` |
| Video episodes | `manatan_video_get_episodes` |
| Video streams | `manatan_video_get_streams` |
| Novel chapters/text | `manatan_novel_get_chapters`, `manatan_novel_get_text` |

## Practical Porting Steps

1. Create a Rust `cdylib` crate targeting `wasm32-unknown-unknown`.
2. Copy the source metadata into `manifest.json`.
3. Port HTTP requests and HTML/JSON parsing into Rust.
4. Return Manatan `CatalogItem`, chapter, page, episode, stream, or novel values.
5. Build `module.wasm`, zip it with `manifest.json`, and name the package `.manatan`.

## Build Example

```sh
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown --manifest-path examples/manga-source/Cargo.toml
mkdir -p /tmp/manga-source-manatan
cp examples/manga-source/manifest.json /tmp/manga-source-manatan/manifest.json
cp examples/manga-source/target/wasm32-unknown-unknown/release/manatan_example_manga_source.wasm /tmp/manga-source-manatan/module.wasm
(cd /tmp/manga-source-manatan && zip -r manga-source.manatan manifest.json module.wasm)
```

Install the resulting `.manatan` through Manatan's extension upload endpoint.
