# Porting Sources to Manatan Extensions

Manatan extensions are Rust/WASM packages. A port usually maps one existing
source implementation to one Rust source module.

## Common Mapping

| Source concept | Manatan export |
| --- | --- |
| Popular/latest list | `manatan_manga_get_list`, `manatan_video_get_list`, `manatan_novel_get_list` |
| Search | `manatan_manga_search`, `manatan_video_search`, `manatan_novel_search` |
| Item details | `manatan_manga_get_details`, `manatan_video_get_details`, `manatan_novel_get_details` |
| Manga chapters | `manatan_manga_get_chapters` |
| Manga pages | `manatan_manga_get_pages` |
| Manga lazy image URL | `manatan_manga_resolve_page_image` |
| Manga chapter cleanup | `manatan_manga_prepare_chapter` |
| Manga home sections | `manatan_manga_get_home` |
| Manga related/covers/migration | `manatan_manga_get_related`, `manatan_manga_get_alternate_covers`, `manatan_manga_migrate` |
| Video episodes | `manatan_video_get_episodes` |
| Video hosters | `manatan_video_get_hosters` |
| Video streams | `manatan_video_get_streams`, `manatan_video_resolve_hoster` |
| Novel chapters/text | `manatan_novel_get_chapters`, `manatan_novel_get_text` |

## Practical Porting Steps

1. Create a Rust `cdylib` crate targeting `wasm32-unknown-unknown`.
2. Copy the source metadata into `manifest.json`.
3. Move search/list filters into typed `filters.json` definitions.
4. Move user settings into typed `preferences.json` definitions.
5. Port HTTP requests and HTML/JSON parsing into Rust.
6. Return Manatan `CatalogItem`, chapter, page, episode, stream, or novel values.
7. Build `module.wasm`, zip it with `manifest.json`, and name the package `.manatan`.

## Porting Surface

Use SDK host helpers for behavior that source plugins often need:

- HTTP requests: `http_fetch`
- Browser-like HTTP requests: `HttpClient::browser()`
- Cookie sharing: declare `permissions.cookies` and use
  `HttpClient::with_cookies_for` or `RequestBuilder::cookies_for` when the
  cookie scope differs from the request URL; Manatan resolves `cookieUrl` in the
  host runtime instead of exposing cookies to WASM
- Explicit login/session cookie management: `cookies_get` and `cookies_set`
- Per-source cache or auth storage: `storage_get`, `storage_set`,
  `storage_delete`, and `storage_list`
- Browser challenge or login pages: `webview_open`
- Site JavaScript state extraction: `webview::extract`, `webview::extract_text`,
  and `webview::extract_json`

For manga sources, preserve viewer direction, update strategy, canonical item
URLs, chapter URLs, scanlator/language data, page referers, image headers,
alternate covers, and related title links. Use lazy `PageContent::Lazy` plus
`manatan_manga_resolve_page_image` when the original source resolves image URLs
from a per-page HTML document or requires a tokenized late request. Use
`manatan_manga_prepare_chapter` when the source needs one final normalization
step before a chapter is stored or rendered.

For video sources, preserve stream metadata when available: quality, format,
resolution, bitrate, codecs, audio tracks, subtitles, intro/outro segments,
timestamps, DRM, player/FFmpeg passthrough arguments, preferred stream flags,
internal source data, and proxy/header requirements. Use the hoster-first exports
when the original source separates hoster extraction from stream resolution. For
novels, preserve text base URLs, image headers, chapter navigation, and any CSS
needed to render source content cleanly.

Some sites populate listings from JavaScript objects after the browser app
loads. Use `webview::extract` for those cases instead of trying to emulate the
site runtime with string rewriting. A typical source loads the page, waits for a
truthy JavaScript condition such as `typeof siteHistory !== 'undefined'`, and
returns a string or JSON payload from a promise-returning script.

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
