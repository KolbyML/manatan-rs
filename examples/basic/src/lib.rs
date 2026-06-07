use manatan_extension::{abi::ExtensionResult, manatan_json_export};
use serde_json::{Value, json};

fn empty_page() -> Value {
    json!({
        "entries": [{
            "key": "demo",
            "title": "Demo Entry",
            "cover": null,
            "url": "https://example.com/demo",
            "authors": ["Manatan"],
            "artists": [],
            "description": "A tiny example entry from a Manatan WASM extension.",
            "tags": ["example"],
            "status": "ongoing"
        }],
        "hasNextPage": false
    })
}

fn manga_list(_request: Value) -> ExtensionResult<Value> {
    Ok(empty_page())
}

fn manga_search(_request: Value) -> ExtensionResult<Value> {
    Ok(empty_page())
}

fn manga_details(request: Value) -> ExtensionResult<Value> {
    Ok(request
        .get("manga")
        .cloned()
        .unwrap_or_else(|| empty_page()["entries"][0].clone()))
}

fn manga_chapters(_request: Value) -> ExtensionResult<Value> {
    Ok(json!([{
        "key": "chapter-1",
        "title": "Chapter 1",
        "chapterNumber": 1.0,
        "volumeNumber": null,
        "dateUploaded": null,
        "scanlators": [],
        "language": "en",
        "thumbnail": null,
        "url": "https://example.com/demo/chapter-1"
    }]))
}

fn manga_pages(_request: Value) -> ExtensionResult<Value> {
    Ok(json!([{
        "content": {
            "url": {
                "url": "https://placehold.co/900x1300/png",
                "context": null
            }
        },
        "thumbnail": null,
        "description": null
    }]))
}

fn video_list(_request: Value) -> ExtensionResult<Value> {
    Ok(empty_page())
}

fn video_search(_request: Value) -> ExtensionResult<Value> {
    Ok(empty_page())
}

fn video_details(request: Value) -> ExtensionResult<Value> {
    Ok(request
        .get("item")
        .cloned()
        .unwrap_or_else(|| empty_page()["entries"][0].clone()))
}

fn video_episodes(_request: Value) -> ExtensionResult<Value> {
    Ok(json!([{
        "key": "episode-1",
        "title": "Episode 1",
        "episodeNumber": 1.0,
        "seasonNumber": 1.0,
        "dateUploaded": null,
        "thumbnail": null,
        "url": "https://example.com/demo/episode-1"
    }]))
}

fn video_streams(_request: Value) -> ExtensionResult<Value> {
    Ok(json!([{
        "url": "https://test-streams.mux.dev/x36xhzz/x36xhzz.m3u8",
        "quality": "auto",
        "format": "hls",
        "headers": {},
        "subtitles": []
    }]))
}

fn novel_list(_request: Value) -> ExtensionResult<Value> {
    Ok(empty_page())
}

fn novel_search(_request: Value) -> ExtensionResult<Value> {
    Ok(empty_page())
}

fn novel_details(request: Value) -> ExtensionResult<Value> {
    Ok(request
        .get("item")
        .cloned()
        .unwrap_or_else(|| empty_page()["entries"][0].clone()))
}

fn novel_chapters(_request: Value) -> ExtensionResult<Value> {
    Ok(json!([{
        "key": "chapter-1",
        "title": "Chapter 1",
        "chapterNumber": 1.0,
        "volumeNumber": null,
        "dateUploaded": null,
        "url": "https://example.com/demo/chapter-1"
    }]))
}

fn novel_text(_request: Value) -> ExtensionResult<Value> {
    Ok(json!({
        "html": "<p>This is a tiny example chapter from a Manatan WASM extension.</p>",
        "text": "This is a tiny example chapter from a Manatan WASM extension."
    }))
}

manatan_json_export!(manatan_manga_get_list, manga_list);
manatan_json_export!(manatan_manga_search, manga_search);
manatan_json_export!(manatan_manga_get_details, manga_details);
manatan_json_export!(manatan_manga_get_chapters, manga_chapters);
manatan_json_export!(manatan_manga_get_pages, manga_pages);

manatan_json_export!(manatan_video_get_list, video_list);
manatan_json_export!(manatan_video_search, video_search);
manatan_json_export!(manatan_video_get_details, video_details);
manatan_json_export!(manatan_video_get_episodes, video_episodes);
manatan_json_export!(manatan_video_get_streams, video_streams);

manatan_json_export!(manatan_novel_get_list, novel_list);
manatan_json_export!(manatan_novel_search, novel_search);
manatan_json_export!(manatan_novel_get_details, novel_details);
manatan_json_export!(manatan_novel_get_chapters, novel_chapters);
manatan_json_export!(manatan_novel_get_text, novel_text);
