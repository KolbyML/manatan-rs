use manatan_extension::{
    CatalogItem, ItemStatus, Paged, SubtitleTrack, VideoEpisode, VideoHoster, VideoStream,
    abi::ExtensionResult, export_video_source, source::VideoSource,
};
use serde_json::Value;
use std::collections::BTreeMap;

const SOURCE: Source = Source;

struct Source;

impl VideoSource for Source {
    fn list(&self, request: Value) -> ExtensionResult<Paged<CatalogItem>> {
        video_get_list(request)
    }
    fn search(&self, request: Value) -> ExtensionResult<Paged<CatalogItem>> {
        video_search(request)
    }
    fn details(&self, request: Value) -> ExtensionResult<CatalogItem> {
        video_get_details(request)
    }
    fn episodes(&self, request: Value) -> ExtensionResult<Vec<VideoEpisode>> {
        video_get_episodes(request)
    }
    fn streams(&self, request: Value) -> ExtensionResult<Vec<VideoStream>> {
        video_get_streams(request)
    }
    fn hosters(&self, request: Value) -> ExtensionResult<Vec<VideoHoster>> {
        video_get_hosters(request)
    }
    fn resolve_hoster(&self, request: Value) -> ExtensionResult<Vec<VideoStream>> {
        video_resolve_hoster(request)
    }
}

fn demo_show(key: &str, title: &str) -> CatalogItem {
    CatalogItem {
        key: key.to_string(),
        title: title.to_string(),
        cover: Some("https://placehold.co/600x900/png?text=Manatan+Video".to_string()),
        url: Some(format!("https://example.com/video/{key}")),
        authors: Vec::new(),
        artists: Vec::new(),
        description: Some("A small video entry returned from a Manatan WASM source.".to_string()),
        tags: vec!["adventure".to_string(), "demo".to_string()],
        status: ItemStatus::Ongoing,
        ..Default::default()
    }
}

fn video_page() -> Paged<CatalogItem> {
    Paged {
        entries: vec![
            demo_show("night-market", "Night Market"),
            demo_show("signal-tower", "Signal Tower"),
        ],
        has_next_page: false,
    }
}

fn video_get_list(_request: Value) -> ExtensionResult<Paged<CatalogItem>> {
    Ok(video_page())
}

fn video_search(request: Value) -> ExtensionResult<Paged<CatalogItem>> {
    let query = request
        .get("query")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .trim();
    if query.is_empty() {
        return Ok(video_page());
    }
    Ok(Paged {
        entries: vec![demo_show("search-result", &format!("{query} Result"))],
        has_next_page: false,
    })
}

fn video_get_details(request: Value) -> ExtensionResult<CatalogItem> {
    let key = request
        .get("item")
        .and_then(|item| item.get("key").or_else(|| item.get("url")))
        .and_then(Value::as_str)
        .unwrap_or("night-market");
    Ok(demo_show(key, "Night Market"))
}

fn video_get_episodes(_request: Value) -> ExtensionResult<Vec<VideoEpisode>> {
    Ok(vec![
        VideoEpisode {
            key: "episode-1".to_string(),
            title: Some("Arrival".to_string()),
            episode_number: Some(1.0),
            season_number: Some(1.0),
            date_uploaded: None,
            thumbnail: Some("https://placehold.co/640x360/png?text=Episode+1".to_string()),
            url: Some("https://example.com/video/night-market/1".to_string()),
            ..Default::default()
        },
        VideoEpisode {
            key: "episode-2".to_string(),
            title: Some("Lantern Street".to_string()),
            episode_number: Some(2.0),
            season_number: Some(1.0),
            date_uploaded: None,
            thumbnail: Some("https://placehold.co/640x360/png?text=Episode+2".to_string()),
            url: Some("https://example.com/video/night-market/2".to_string()),
            ..Default::default()
        },
    ])
}

fn video_get_streams(_request: Value) -> ExtensionResult<Vec<VideoStream>> {
    let mut headers = BTreeMap::new();
    headers.insert("Referer".to_string(), "https://example.com".to_string());
    Ok(vec![VideoStream {
        url: "https://test-streams.mux.dev/x36xhzz/x36xhzz.m3u8".to_string(),
        hoster: Some(demo_hoster()),
        quality: Some("auto".to_string()),
        format: Some("hls".to_string()),
        is_hls: true,
        preferred: true,
        headers,
        subtitles: vec![SubtitleTrack {
            url: "https://example.com/subtitles/night-market-en.vtt".to_string(),
            language: Some("en".to_string()),
            label: Some("English".to_string()),
            format: Some("vtt".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    }])
}

fn video_get_hosters(_request: Value) -> ExtensionResult<Vec<VideoHoster>> {
    Ok(vec![demo_hoster()])
}

fn video_resolve_hoster(request: Value) -> ExtensionResult<Vec<VideoStream>> {
    video_get_streams(request)
}

fn demo_hoster() -> VideoHoster {
    VideoHoster {
        key: "demo-hls".to_string(),
        name: "Demo HLS".to_string(),
        url: Some("https://test-streams.mux.dev".to_string()),
        lazy: true,
        video_count: Some(1),
        ..Default::default()
    }
}

export_video_source!(SOURCE);
