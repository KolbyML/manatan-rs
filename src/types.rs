use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub type Context = BTreeMap<String, String>;
pub type JsonMap = BTreeMap<String, serde_json::Value>;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Paged<T> {
    pub entries: Vec<T>,
    pub has_next_page: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ItemStatus {
    Unknown,
    Ongoing,
    Completed,
    Cancelled,
    Hiatus,
}

impl Default for ItemStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogItem {
    pub key: String,
    pub title: String,
    #[serde(default)]
    pub alternate_titles: Vec<String>,
    pub cover: Option<String>,
    #[serde(default)]
    pub banner: Option<String>,
    pub url: Option<String>,
    pub authors: Vec<String>,
    pub artists: Vec<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub rating: Option<f32>,
    #[serde(default)]
    pub content_rating: Option<String>,
    #[serde(default)]
    pub latest_update: Option<i64>,
    pub status: ItemStatus,
    #[serde(default)]
    pub extra: JsonMap,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MangaChapter {
    pub key: String,
    pub title: Option<String>,
    pub chapter_number: Option<f32>,
    pub volume_number: Option<f32>,
    pub date_uploaded: Option<i64>,
    pub scanlators: Vec<String>,
    pub language: Option<String>,
    pub thumbnail: Option<String>,
    pub url: Option<String>,
    #[serde(default)]
    pub source_order: Option<i32>,
    #[serde(default)]
    pub is_locked: bool,
    #[serde(default)]
    pub extra: JsonMap,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MangaPage {
    pub content: PageContent,
    pub thumbnail: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub headers: Context,
    #[serde(default)]
    pub extra: JsonMap,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PageContent {
    Url {
        url: String,
        context: Option<Context>,
    },
    Text {
        text: String,
    },
    ImageBytes {
        bytes: Vec<u8>,
        mime_type: String,
    },
    ArchiveEntry {
        archive_url: String,
        entry_path: String,
    },
}

impl Default for PageContent {
    fn default() -> Self {
        Self::Text {
            text: String::new(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoEpisode {
    pub key: String,
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    pub episode_number: Option<f32>,
    pub season_number: Option<f32>,
    pub date_uploaded: Option<i64>,
    pub thumbnail: Option<String>,
    pub url: Option<String>,
    #[serde(default)]
    pub duration_seconds: Option<f64>,
    #[serde(default)]
    pub source_order: Option<i32>,
    #[serde(default)]
    pub extra: JsonMap,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoStream {
    pub url: String,
    #[serde(default)]
    pub name: Option<String>,
    pub quality: Option<String>,
    pub format: Option<String>,
    #[serde(default)]
    pub resolution: Option<String>,
    #[serde(default)]
    pub bitrate: Option<u64>,
    #[serde(default)]
    pub video_codec: Option<String>,
    #[serde(default)]
    pub audio_codec: Option<String>,
    #[serde(default)]
    pub size_bytes: Option<u64>,
    #[serde(default)]
    pub duration_seconds: Option<f64>,
    #[serde(default)]
    pub is_hls: bool,
    #[serde(default)]
    pub is_dash: bool,
    #[serde(default)]
    pub is_backup: bool,
    #[serde(default)]
    pub requires_proxy: bool,
    pub headers: Context,
    #[serde(default)]
    pub audio_tracks: Vec<AudioTrack>,
    pub subtitles: Vec<SubtitleTrack>,
    #[serde(default)]
    pub intro: Option<MediaSegment>,
    #[serde(default)]
    pub outro: Option<MediaSegment>,
    #[serde(default)]
    pub drm: Option<DrmInfo>,
    #[serde(default)]
    pub extra: JsonMap,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubtitleTrack {
    pub url: String,
    pub language: Option<String>,
    pub label: Option<String>,
    pub format: Option<String>,
    #[serde(default)]
    pub headers: Context,
    #[serde(default)]
    pub is_default: bool,
    #[serde(default)]
    pub is_forced: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioTrack {
    pub url: Option<String>,
    pub language: Option<String>,
    pub label: Option<String>,
    pub format: Option<String>,
    #[serde(default)]
    pub headers: Context,
    #[serde(default)]
    pub is_default: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaSegment {
    pub start_seconds: f64,
    pub end_seconds: f64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrmInfo {
    pub scheme: String,
    pub license_url: Option<String>,
    #[serde(default)]
    pub headers: Context,
    #[serde(default)]
    pub extra: JsonMap,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NovelChapter {
    pub key: String,
    pub title: Option<String>,
    pub chapter_number: Option<f32>,
    pub volume_number: Option<f32>,
    pub date_uploaded: Option<i64>,
    pub url: Option<String>,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub source_order: Option<i32>,
    #[serde(default)]
    pub extra: JsonMap,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NovelText {
    pub html: Option<String>,
    pub text: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub base_url: Option<String>,
    #[serde(default)]
    pub css: Option<String>,
    #[serde(default)]
    pub image_headers: Context,
    #[serde(default)]
    pub next_chapter_key: Option<String>,
    #[serde(default)]
    pub previous_chapter_key: Option<String>,
    #[serde(default)]
    pub extra: JsonMap,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListRequest {
    pub listing: String,
    pub page: u32,
    #[serde(default)]
    pub source_id: Option<String>,
    #[serde(default)]
    pub preferences: JsonMap,
    #[serde(default)]
    pub context: JsonMap,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchRequest {
    pub query: String,
    pub page: u32,
    #[serde(default)]
    pub filters: Vec<FilterValue>,
    #[serde(default)]
    pub source_id: Option<String>,
    #[serde(default)]
    pub preferences: JsonMap,
    #[serde(default)]
    pub context: JsonMap,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemRequest<T = serde_json::Value> {
    pub item: T,
    #[serde(default)]
    pub source_id: Option<String>,
    #[serde(default)]
    pub preferences: JsonMap,
    #[serde(default)]
    pub context: JsonMap,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MangaChapterRequest<T = serde_json::Value> {
    pub manga: T,
    #[serde(default)]
    pub source_id: Option<String>,
    #[serde(default)]
    pub preferences: JsonMap,
    #[serde(default)]
    pub context: JsonMap,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MangaPageRequest<T = serde_json::Value, C = serde_json::Value> {
    pub manga: T,
    pub chapter: C,
    #[serde(default)]
    pub source_id: Option<String>,
    #[serde(default)]
    pub preferences: JsonMap,
    #[serde(default)]
    pub context: JsonMap,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoStreamRequest<T = serde_json::Value, E = serde_json::Value> {
    pub item: T,
    pub episode: E,
    #[serde(default)]
    pub source_id: Option<String>,
    #[serde(default)]
    pub preferences: JsonMap,
    #[serde(default)]
    pub context: JsonMap,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NovelTextRequest<T = serde_json::Value, C = serde_json::Value> {
    pub item: T,
    pub chapter: C,
    #[serde(default)]
    pub source_id: Option<String>,
    #[serde(default)]
    pub preferences: JsonMap,
    #[serde(default)]
    pub context: JsonMap,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolveHosterRequest {
    pub url: String,
    #[serde(default)]
    pub headers: Context,
    #[serde(default)]
    pub source_id: Option<String>,
    #[serde(default)]
    pub preferences: JsonMap,
    #[serde(default)]
    pub context: JsonMap,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterValue {
    pub id: String,
    #[serde(default)]
    pub value: serde_json::Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum FilterDefinition {
    Header {
        title: String,
    },
    Separator,
    Text {
        id: String,
        title: String,
        #[serde(default)]
        default: Option<String>,
    },
    CheckBox {
        id: String,
        title: String,
        #[serde(default)]
        default: bool,
    },
    TriState {
        id: String,
        title: String,
        #[serde(default)]
        default: Option<bool>,
    },
    Select {
        id: String,
        title: String,
        options: Vec<OptionItem>,
        #[serde(default)]
        default: Option<String>,
    },
    MultiSelect {
        id: String,
        title: String,
        options: Vec<OptionItem>,
        #[serde(default)]
        default: Vec<String>,
    },
    Sort {
        id: String,
        title: String,
        options: Vec<SortOption>,
        #[serde(default)]
        default: Option<SortSelection>,
    },
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionItem {
    pub value: String,
    pub label: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SortOption {
    pub value: String,
    pub label: String,
    #[serde(default)]
    pub supports_ascending: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SortSelection {
    pub value: String,
    #[serde(default)]
    pub ascending: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum PreferenceDefinition {
    Text {
        id: String,
        title: String,
        #[serde(default)]
        summary: Option<String>,
        #[serde(default)]
        default: Option<String>,
        #[serde(default)]
        secure: bool,
    },
    Switch {
        id: String,
        title: String,
        #[serde(default)]
        summary: Option<String>,
        #[serde(default)]
        default: bool,
    },
    Select {
        id: String,
        title: String,
        options: Vec<OptionItem>,
        #[serde(default)]
        summary: Option<String>,
        #[serde(default)]
        default: Option<String>,
    },
    MultiSelect {
        id: String,
        title: String,
        options: Vec<OptionItem>,
        #[serde(default)]
        summary: Option<String>,
        #[serde(default)]
        default: Vec<String>,
    },
    Button {
        id: String,
        title: String,
        #[serde(default)]
        summary: Option<String>,
    },
}
