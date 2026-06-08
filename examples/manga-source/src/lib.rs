use manatan_extension::{
    AlternateCover, CatalogItem, HomeSection, HomeSectionStyle, ItemStatus, MangaChapter,
    MangaPage, MangaPageImage, PageContent, Paged, Viewer, abi::ExtensionResult,
    export_manga_source, source::MangaSource, webview,
};
use serde_json::{Value, json};

const SOURCE: Source = Source;

struct Source;

impl MangaSource for Source {
    fn list(&self, request: Value) -> ExtensionResult<Paged<CatalogItem>> {
        manga_get_list(request)
    }
    fn search(&self, request: Value) -> ExtensionResult<Paged<CatalogItem>> {
        manga_search(request)
    }
    fn details(&self, request: Value) -> ExtensionResult<CatalogItem> {
        manga_get_details(request)
    }
    fn chapters(&self, request: Value) -> ExtensionResult<Vec<MangaChapter>> {
        manga_get_chapters(request)
    }
    fn pages(&self, request: Value) -> ExtensionResult<Vec<MangaPage>> {
        manga_get_pages(request)
    }
    fn home(&self, request: Value) -> ExtensionResult<Vec<HomeSection<CatalogItem>>> {
        manga_get_home(request)
    }
    fn resolve_page_image(&self, request: Value) -> ExtensionResult<MangaPageImage> {
        manga_resolve_page_image(request)
    }
    fn alternate_covers(&self, request: Value) -> ExtensionResult<Vec<AlternateCover>> {
        manga_get_alternate_covers(request)
    }
    fn related(&self, request: Value) -> ExtensionResult<Vec<CatalogItem>> {
        manga_get_related(request)
    }
}

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
        viewer: Some(Viewer::RightToLeft),
        initialized: true,
        ..Default::default()
    }
}

fn manga_get_home(_request: Value) -> ExtensionResult<Vec<HomeSection<CatalogItem>>> {
    Ok(vec![
        HomeSection {
            id: "featured".to_string(),
            title: "Featured".to_string(),
            style: Some(HomeSectionStyle::Featured),
            entries: vec![demo_manga("iron-lantern", "Iron Lantern")],
            has_more: false,
            ..Default::default()
        },
        HomeSection {
            id: "popular".to_string(),
            title: "Popular".to_string(),
            listing: Some("popular".to_string()),
            style: Some(HomeSectionStyle::Cover),
            entries: manga_page().entries,
            has_more: false,
            ..Default::default()
        },
    ])
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

fn manga_get_list(request: Value) -> ExtensionResult<Paged<CatalogItem>> {
    if request.get("listing").and_then(Value::as_str) == Some("webview-demo") {
        return webview_demo_listing();
    }
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
            content: PageContent::Lazy {
                key: "page-2".to_string(),
                url: Some("https://example.com/manga/iron-lantern/1/page/2".to_string()),
                page_url: Some("https://example.com/manga/iron-lantern/1".to_string()),
                context: None,
            },
            thumbnail: None,
            description: Some("Page 2".to_string()),
            ..Default::default()
        },
    ])
}

fn manga_resolve_page_image(request: Value) -> ExtensionResult<MangaPageImage> {
    let page_key = request
        .get("page")
        .and_then(|page| page.get("content"))
        .and_then(|content| content.get("lazy"))
        .and_then(|lazy| lazy.get("key"))
        .and_then(Value::as_str)
        .unwrap_or("page-1");
    Ok(MangaPageImage {
        url: format!("https://placehold.co/900x1300/png?text={page_key}"),
        page_url: Some("https://example.com/manga/iron-lantern/1".to_string()),
        headers: [("referer".to_string(), "https://example.com".to_string())]
            .into_iter()
            .collect(),
        extra: [("resolvedFrom".to_string(), json!("lazy"))]
            .into_iter()
            .collect(),
        ..Default::default()
    })
}

fn manga_get_alternate_covers(_request: Value) -> ExtensionResult<Vec<AlternateCover>> {
    Ok(vec![AlternateCover {
        url: "https://placehold.co/600x900/png?text=Alt+Cover".to_string(),
        language: Some("en".to_string()),
        volume: Some("1".to_string()),
        ..Default::default()
    }])
}

fn manga_get_related(_request: Value) -> ExtensionResult<Vec<CatalogItem>> {
    Ok(vec![demo_manga(
        "iron-lantern-side-story",
        "Iron Lantern: Side Story",
    )])
}

fn webview_demo_listing() -> ExtensionResult<Paged<CatalogItem>> {
    let payload = webview::extract_text(
        webview::ExtractRequest::new(
            "https://example.com/reader",
            "Promise.resolve(JSON.stringify(window.__MANATAN_EXAMPLE_DATA__ || []))",
        )
        .wait_for_script("Array.isArray(window.__MANATAN_EXAMPLE_DATA__)")
        .timeout_ms(10_000),
    )?;
    let entries = serde_json::from_str::<Vec<Value>>(&payload)
        .unwrap_or_default()
        .into_iter()
        .take(20)
        .enumerate()
        .map(|(index, item)| {
            let title = item
                .get("title")
                .or_else(|| item.get("name"))
                .and_then(Value::as_str)
                .unwrap_or("WebView Series");
            let key = item
                .get("slug")
                .or_else(|| item.get("id"))
                .or_else(|| item.get("url"))
                .and_then(Value::as_str)
                .map(ToString::to_string)
                .unwrap_or_else(|| format!("webview-series-{index}"));
            demo_manga(&key, title)
        })
        .collect();
    Ok(Paged {
        entries,
        has_next_page: false,
    })
}

export_manga_source!(SOURCE);
