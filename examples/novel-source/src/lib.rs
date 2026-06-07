use manatan_extension::{
    CatalogItem, ItemStatus, NovelChapter, NovelText, Paged, abi::ExtensionResult,
    manatan_json_export,
};
use serde_json::Value;

fn demo_novel(key: &str, title: &str) -> CatalogItem {
    CatalogItem {
        key: key.to_string(),
        title: title.to_string(),
        cover: Some("https://placehold.co/600x900/png?text=Manatan+Novel".to_string()),
        url: Some(format!("https://example.com/novel/{key}")),
        authors: vec!["Example Writer".to_string()],
        artists: Vec::new(),
        description: Some("A small novel entry returned from a Manatan WASM source.".to_string()),
        tags: vec!["fantasy".to_string(), "demo".to_string()],
        status: ItemStatus::Ongoing,
    }
}

fn novel_page() -> Paged<CatalogItem> {
    Paged {
        entries: vec![
            demo_novel("glass-library", "Glass Library"),
            demo_novel("map-of-rain", "Map of Rain"),
        ],
        has_next_page: false,
    }
}

fn novel_get_list(_request: Value) -> ExtensionResult<Paged<CatalogItem>> {
    Ok(novel_page())
}

fn novel_search(request: Value) -> ExtensionResult<Paged<CatalogItem>> {
    let query = request
        .get("query")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .trim();
    if query.is_empty() {
        return Ok(novel_page());
    }
    Ok(Paged {
        entries: vec![demo_novel("search-result", &format!("{query} Result"))],
        has_next_page: false,
    })
}

fn novel_get_details(request: Value) -> ExtensionResult<CatalogItem> {
    let key = request
        .get("item")
        .and_then(|item| item.get("key").or_else(|| item.get("url")))
        .and_then(Value::as_str)
        .unwrap_or("glass-library");
    Ok(demo_novel(key, "Glass Library"))
}

fn novel_get_chapters(_request: Value) -> ExtensionResult<Vec<NovelChapter>> {
    Ok(vec![
        NovelChapter {
            key: "chapter-1".to_string(),
            title: Some("A Borrowed Key".to_string()),
            chapter_number: Some(1.0),
            volume_number: Some(1.0),
            date_uploaded: None,
            url: Some("https://example.com/novel/glass-library/1".to_string()),
        },
        NovelChapter {
            key: "chapter-2".to_string(),
            title: Some("Stacks at Midnight".to_string()),
            chapter_number: Some(2.0),
            volume_number: Some(1.0),
            date_uploaded: None,
            url: Some("https://example.com/novel/glass-library/2".to_string()),
        },
    ])
}

fn novel_get_text(_request: Value) -> ExtensionResult<NovelText> {
    Ok(NovelText {
        html: Some(
            "<h1>A Borrowed Key</h1><p>The library woke before the city did.</p>".to_string(),
        ),
        text: Some("A Borrowed Key\n\nThe library woke before the city did.".to_string()),
    })
}

manatan_json_export!(manatan_novel_get_list, novel_get_list);
manatan_json_export!(manatan_novel_search, novel_search);
manatan_json_export!(manatan_novel_get_details, novel_get_details);
manatan_json_export!(manatan_novel_get_chapters, novel_get_chapters);
manatan_json_export!(manatan_novel_get_text, novel_get_text);
