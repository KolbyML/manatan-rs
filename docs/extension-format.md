# Manatan Extension Format

The Manatan extension format is the first-class source system for Manatan. It is
Rust/WASM based, cross-platform, and media-first.

## Package

Extension packages use the `.manatan` extension and are zip archives:

```text
manifest.json
module.wasm
filters.json       optional typed filter schema
preferences.json   optional typed source settings
assets/...         optional
```

`manifest.json` declares package metadata, permissions, sources, and the package
media kind. `module.wasm` exports the source functions Manatan calls.

Each package supports exactly one media kind: `manga`, `video`, or `novel`.
Create separate packages when a site needs sources for multiple media kinds.

The examples directory includes focused examples for each media kind:

- `examples/manga-source`
- `examples/video-source`
- `examples/novel-source`

## Manifest

```json
{
  "schemaVersion": 1,
  "packageId": "com.example.sources",
  "name": "Example Sources",
  "version": "1.0.0",
  "versionCode": 1,
  "minimumManatanVersion": "0.1.0",
  "contentType": "manga",
  "permissions": {
    "network": ["https://example.com"],
    "cookies": true,
    "webview": false,
    "storage": false
  },
  "sources": [
    {
      "id": "example",
      "name": "Example",
      "lang": "en",
      "baseUrl": "https://example.com",
      "contentType": "manga",
      "capabilities": {
        "search": true,
        "latest": true,
        "filters": true,
        "preferences": true,
        "home": true,
        "hosterResolution": true
      },
      "listings": [
        { "id": "popular", "name": "Popular" },
        { "id": "latest", "name": "Latest" }
      ]
    }
  ]
}
```

## WASM Exports

All export names are Manatan-native and stable:

```text
manatan_start
manatan_get_home
manatan_get_filters
manatan_get_preferences

manatan_manga_get_list
manatan_manga_search
manatan_manga_get_details
manatan_manga_get_chapters
manatan_manga_get_pages

manatan_video_get_list
manatan_video_search
manatan_video_get_details
manatan_video_get_episodes
manatan_video_get_hosters
manatan_video_get_streams
manatan_video_resolve_hoster

manatan_novel_get_list
manatan_novel_search
manatan_novel_get_details
manatan_novel_get_chapters
manatan_novel_get_text
```

The SDK should hide the ABI details behind traits, but the runtime keeps these
names stable so packages remain portable across desktop, Android, and iOS.

## Requests

List, search, details, chapter/page, episode/stream, novel text, and hoster
exports receive JSON request objects. The SDK exposes typed request structs:

- `ListRequest`
- `SearchRequest`
- `ItemRequest`
- `MangaChapterRequest`
- `MangaPageRequest`
- `VideoStreamRequest`
- `VideoHosterRequest`
- `VideoHosterStreamsRequest`
- `NovelTextRequest`
- `ResolveHosterRequest`

Every request can include `sourceId`, `preferences`, and `context`. Search
requests include typed `FilterValue` entries from the source filter schema.

## Filters And Preferences

`filters.json` is an array of `FilterDefinition` values. Supported filter types
are `header`, `separator`, `text`, `checkBox`, `triState`, `select`,
`multiSelect`, and `sort`.

`preferences.json` is an array of `PreferenceDefinition` values. Supported
preference types are `text`, `switch`, `select`, `multiSelect`, and `button`.
Manatan persists current values per source and passes them back to exports in
the request `preferences` object.

## Host Calls

Extensions can call Manatan host APIs through the SDK helpers:

| Helper | Operation | Permission |
| --- | --- | --- |
| `http_fetch` | `http.fetch` | `permissions.network` |
| `storage_get`, `storage_set`, `storage_delete`, `storage_list` | `storage.*` | `permissions.storage` |
| `cookies_get`, `cookies_set` | `cookies.*` | `permissions.cookies` |
| `webview_open` | `webview.open` | `permissions.webview` |

Storage is per extension source. Cookie calls use Manatan's shared cookie jar.
Webview calls use Manatan's platform webview bridge for challenge and login
flows, returning the final URL and synced cookies after navigation completes.

## Media Metadata

The common catalog model supports alternate titles, cover/banner images,
authors, artists, tags, rating, language, content rating, update time, status,
and an `extra` map for source-specific metadata.

Manga pages can provide request headers. Video streams can provide quality,
format, resolution, bitrate, codecs, duration, proxy flags, preferred/default
state, initialization state, hoster metadata, audio tracks, subtitles,
intro/outro segments, arbitrary timestamps, DRM info, player passthrough
arguments (`mpvArgs`, `ffmpegStreamArgs`, `ffmpegVideoArgs`), internal source
data, and extra metadata. Novel text can provide HTML or plain text, CSS, base
URL, image headers, previous/next chapter keys, and extra metadata.

Video sources can either return all streams directly from
`manatan_video_get_streams`, or expose a hoster-first flow:

1. `manatan_video_get_hosters` returns `VideoHoster` entries for an episode.
2. Manatan shows those hosters in the existing video flow.
3. `manatan_video_resolve_hoster` receives the selected hoster and returns
   `VideoStream` entries for playback.

## Scope

This specification only describes Manatan-native extension packages and the
runtime contract they use.
