# Manatan Extension Format

The Manatan extension format is the first-class source system for Manatan. It is
Rust/WASM based, cross-platform, and media-first.

## Package

Extension packages use the `.manatan` extension and are zip archives:

```text
manifest.json
module.wasm
filters.json       optional
preferences.json   optional
assets/...         optional
```

`manifest.json` declares package metadata, permissions, sources, and supported
media kinds. `module.wasm` exports the source functions Manatan calls.

The examples directory includes both an all-in-one source and focused examples
for each media kind:

- `examples/basic`
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
  "contentTypes": ["manga", "video", "novel"],
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
      "contentTypes": ["manga", "video", "novel"],
      "capabilities": {
        "search": true,
        "latest": true,
        "filters": true,
        "preferences": true,
        "home": true,
        "hosterResolution": true
      },
      "listings": [
        { "id": "popular", "name": "Popular", "contentTypes": ["manga", "video", "novel"] },
        { "id": "latest", "name": "Latest", "contentTypes": ["manga", "video", "novel"] }
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

## Scope

This specification only describes Manatan-native extension packages and the
runtime contract they use.
