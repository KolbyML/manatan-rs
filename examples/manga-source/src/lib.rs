use manatan_extension::{
    CatalogItem, ItemStatus, MangaChapter, MangaPage, PageContent, Paged, abi::ExtensionResult,
    manatan_json_export,
};
use serde_json::Value;

fn demo_manga(key: &str, title: &str) -> CatalogItem {
    CatalogItem {
        key: key.to_string(),
        title: title.to_string(),
        cover: Some("https://placehold.co/600x900/png?text=Manatan+Manga".to_string()),
        url: Some(format!("https://example.com/manga/{key}")),
        authors: vec!["Example Author".to_string()],
        artists: vec!["Example Artist".to_string()],
        description: Some("A small manga entry returned from a Manatan WASM source.".to_string()),
        tags: vec!["action".to_string(), "demo".to_string()],
        status: ItemStatus::Ongoing,
        ..Default::default()
    }
}

fn manga_page() -> Paged<CatalogItem> {
    Paged {
        entries: vec![
            demo_manga("iron-lantern", "Iron Lantern"),
            demo_manga("paper-city", "Paper City"),
        ],
        has_next_page: false,
    }
}

fn manga_get_list(_request: Value) -> ExtensionResult<Paged<CatalogItem>> {
    Ok(manga_page())
}

fn manga_search(request: Value) -> ExtensionResult<Paged<CatalogItem>> {
    let query = request
        .get("query")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .trim();
    if query.is_empty() {
        return Ok(manga_page());
    }
    Ok(Paged {
        entries: vec![demo_manga("search-result", &format!("{query} Result"))],
        has_next_page: false,
    })
}

fn manga_get_details(request: Value) -> ExtensionResult<CatalogItem> {
    let key = request
        .get("manga")
        .and_then(|manga| manga.get("key").or_else(|| manga.get("url")))
        .and_then(Value::as_str)
        .unwrap_or("iron-lantern");
    Ok(demo_manga(key, "Iron Lantern"))
}

fn manga_get_chapters(_request: Value) -> ExtensionResult<Vec<MangaChapter>> {
    Ok(vec![
        MangaChapter {
            key: "chapter-1".to_string(),
            title: Some("The First Light".to_string()),
            chapter_number: Some(1.0),
            volume_number: Some(1.0),
            date_uploaded: None,
            scanlators: vec!["Manatan Scan Group".to_string()],
            language: Some("en".to_string()),
            thumbnail: None,
            url: Some("https://example.com/manga/iron-lantern/1".to_string()),
            ..Default::default()
        },
        MangaChapter {
            key: "chapter-2".to_string(),
            title: Some("A Door Opens".to_string()),
            chapter_number: Some(2.0),
            volume_number: Some(1.0),
            date_uploaded: None,
            scanlators: vec!["Manatan Scan Group".to_string()],
            language: Some("en".to_string()),
            thumbnail: None,
            url: Some("https://example.com/manga/iron-lantern/2".to_string()),
            ..Default::default()
        },
    ])
}

fn manga_get_pages(_request: Value) -> ExtensionResult<Vec<MangaPage>> {
    Ok(vec![
        MangaPage {
            content: PageContent::Url {
                url: "https://placehold.co/900x1300/png?text=Page+1".to_string(),
                context: None,
            },
            thumbnail: None,
            description: Some("Page 1".to_string()),
            ..Default::default()
        },
        MangaPage {
            content: PageContent::Url {
                url: "https://placehold.co/900x1300/png?text=Page+2".to_string(),
                context: None,
            },
            thumbnail: None,
            description: Some("Page 2".to_string()),
            ..Default::default()
        },
    ])
}

manatan_json_export!(manatan_manga_get_list, manga_get_list);
manatan_json_export!(manatan_manga_search, manga_search);
manatan_json_export!(manatan_manga_get_details, manga_get_details);
manatan_json_export!(manatan_manga_get_chapters, manga_get_chapters);
manatan_json_export!(manatan_manga_get_pages, manga_get_pages);
