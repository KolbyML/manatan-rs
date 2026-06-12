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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Viewer {
    LeftToRight,
    RightToLeft,
    Vertical,
    Webtoon,
    ContinuousVertical,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UpdateStrategy {
    Always,
    OnlyFetchOnce,
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
    pub initialized: bool,
    #[serde(default)]
    pub viewer: Option<Viewer>,
    #[serde(default)]
    pub update_strategy: Option<UpdateStrategy>,
    #[serde(default)]
    pub next_update_time: Option<i64>,
    #[serde(default)]
    pub alternate_covers: Vec<AlternateCover>,
    #[serde(default)]
    pub extra: JsonMap,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlternateCover {
    pub url: String,
    #[serde(default)]
    pub thumbnail: Option<String>,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub volume: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub headers: Context,
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
    pub page_count: Option<u32>,
    #[serde(default)]
    pub summary: Option<String>,
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
    Request {
        request: ImageRequest,
    },
    Lazy {
        key: String,
        #[serde(default)]
        url: Option<String>,
        #[serde(default)]
        page_url: Option<String>,
        #[serde(default)]
        context: Option<Context>,
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
    pub release_group: Option<String>,
    #[serde(default)]
    pub variant: Option<String>,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub size_bytes: Option<u64>,
    #[serde(default)]
    pub is_filler: bool,
    #[serde(default)]
    pub is_locked: bool,
    #[serde(default)]
    pub labels: Vec<String>,
    #[serde(default)]
    pub source_order: Option<i32>,
    #[serde(default)]
    pub extra: JsonMap,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoPlayerArg {
    pub name: String,
    pub value: String,
}

impl VideoPlayerArg {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoStream {
    pub url: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub hoster: Option<VideoHoster>,
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
    #[serde(default)]
    pub preferred: bool,
    #[serde(default)]
    pub initialized: bool,
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
    pub timestamps: Vec<MediaTimestamp>,
    #[serde(default)]
    pub mpv_args: Vec<VideoPlayerArg>,
    #[serde(default)]
    pub ffmpeg_stream_args: Vec<VideoPlayerArg>,
    #[serde(default)]
    pub ffmpeg_video_args: Vec<VideoPlayerArg>,
    #[serde(default)]
    pub internal_data: Option<String>,
    #[serde(default)]
    pub stream_kind: Option<VideoStreamKind>,
    #[serde(default)]
    pub torrent: Option<TorrentInfo>,
    #[serde(default)]
    pub debrid: Option<DebridInfo>,
    #[serde(default)]
    pub segment_processing: Option<SegmentProcessing>,
    #[serde(default)]
    pub extra: JsonMap,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum VideoStreamKind {
    Direct,
    Hls,
    Dash,
    Torrent,
    Magnet,
    Debrid,
    External,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TorrentInfo {
    #[serde(default)]
    pub magnet_url: Option<String>,
    #[serde(default)]
    pub torrent_url: Option<String>,
    #[serde(default)]
    pub file_index: Option<u32>,
    #[serde(default)]
    pub file_name: Option<String>,
    #[serde(default)]
    pub trackers: Vec<String>,
    #[serde(default)]
    pub seeders: Option<u32>,
    #[serde(default)]
    pub leechers: Option<u32>,
    #[serde(default)]
    pub size_bytes: Option<u64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebridInfo {
    #[serde(default)]
    pub provider: Option<String>,
    #[serde(default)]
    pub requires_account: bool,
    #[serde(default)]
    pub external_playback: bool,
    #[serde(default)]
    pub expires_at: Option<i64>,
    #[serde(default)]
    pub extra: JsonMap,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SegmentProcessing {
    #[serde(default)]
    pub proxy_playlist: bool,
    #[serde(default)]
    pub rewrite_segments: bool,
    #[serde(default)]
    pub strip_fake_image_header: bool,
    #[serde(default)]
    pub segment_headers: Context,
    #[serde(default)]
    pub extra: JsonMap,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoHoster {
    pub key: String,
    pub name: String,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub lazy: bool,
    #[serde(default)]
    pub video_count: Option<u32>,
    #[serde(default)]
    pub headers: Context,
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
pub struct MediaTimestamp {
    pub time_seconds: f64,
    pub label: String,
    #[serde(default)]
    pub kind: Option<String>,
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
    pub section: Option<String>,
    #[serde(default)]
    pub page: Option<u32>,
    #[serde(default)]
    pub release_group: Option<String>,
    #[serde(default)]
    pub word_count: Option<u32>,
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub is_locked: bool,
    #[serde(default)]
    pub thumbnail: Option<String>,
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
    pub javascript: Option<String>,
    #[serde(default)]
    pub uses_web_storage: bool,
    #[serde(default)]
    pub image_headers: Context,
    #[serde(default)]
    pub image_request: Option<ImageRequest>,
    #[serde(default)]
    pub image_requests: Vec<ImageRequest>,
    #[serde(default)]
    pub next_chapter_key: Option<String>,
    #[serde(default)]
    pub previous_chapter_key: Option<String>,
    #[serde(default)]
    pub extra: JsonMap,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageRequest {
    pub url: String,
    #[serde(default)]
    pub method: Option<String>,
    #[serde(default)]
    pub headers: Context,
    #[serde(default)]
    pub body_base64: Option<String>,
    #[serde(default)]
    pub credentials: Option<String>,
    #[serde(default)]
    pub referrer: Option<String>,
    #[serde(default)]
    pub referrer_policy: Option<String>,
    #[serde(default)]
    pub requires_proxy: bool,
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
pub struct HomeRequest {
    #[serde(default)]
    pub source_id: Option<String>,
    #[serde(default)]
    pub preferences: JsonMap,
    #[serde(default)]
    pub context: JsonMap,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UrlResolveRequest {
    pub url: String,
    #[serde(default)]
    pub source_id: Option<String>,
    #[serde(default)]
    pub preferences: JsonMap,
    #[serde(default)]
    pub context: JsonMap,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UrlResolveResult {
    #[serde(default)]
    pub item: Option<CatalogItem>,
    #[serde(default)]
    pub chapter: Option<serde_json::Value>,
    #[serde(default)]
    pub episode: Option<serde_json::Value>,
    #[serde(default)]
    pub page: Option<serde_json::Value>,
    #[serde(default)]
    pub search: Option<SearchRequest>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub extra: JsonMap,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HomeSection<T = CatalogItem> {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub subtitle: Option<String>,
    #[serde(default)]
    pub listing: Option<String>,
    #[serde(default)]
    pub style: Option<HomeSectionStyle>,
    pub entries: Vec<T>,
    #[serde(default)]
    pub has_more: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HomeSectionStyle {
    Compact,
    Cover,
    Banner,
    Featured,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MangaChapterUrlRequest<T = serde_json::Value, C = serde_json::Value> {
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
pub struct MangaPrepareChapterRequest<T = serde_json::Value, C = serde_json::Value> {
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
pub struct MangaPageImageRequest<
    T = serde_json::Value,
    C = serde_json::Value,
    P = serde_json::Value,
> {
    pub manga: T,
    pub chapter: C,
    pub page: P,
    #[serde(default)]
    pub page_index: Option<u32>,
    #[serde(default)]
    pub source_id: Option<String>,
    #[serde(default)]
    pub preferences: JsonMap,
    #[serde(default)]
    pub context: JsonMap,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MangaPageImage {
    pub url: String,
    #[serde(default)]
    pub request: Option<ImageRequest>,
    #[serde(default)]
    pub page_url: Option<String>,
    #[serde(default)]
    pub mime_type: Option<String>,
    #[serde(default)]
    pub headers: Context,
    #[serde(default)]
    pub context: Option<Context>,
    #[serde(default)]
    pub extra: JsonMap,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MangaPageImageProcessRequest<
    T = serde_json::Value,
    C = serde_json::Value,
    P = serde_json::Value,
> {
    pub manga: T,
    pub chapter: C,
    pub page: P,
    pub image_base64: String,
    #[serde(default)]
    pub mime_type: Option<String>,
    #[serde(default)]
    pub image_headers: Context,
    #[serde(default)]
    pub page_index: Option<u32>,
    #[serde(default)]
    pub source_id: Option<String>,
    #[serde(default)]
    pub preferences: JsonMap,
    #[serde(default)]
    pub context: JsonMap,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessedImage {
    pub image_base64: String,
    #[serde(default)]
    pub mime_type: Option<String>,
    #[serde(default)]
    pub extra: JsonMap,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MangaRelatedRequest<T = serde_json::Value> {
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
pub struct MangaMigrationRequest<T = serde_json::Value> {
    pub manga: T,
    pub target_source_id: String,
    #[serde(default)]
    pub source_id: Option<String>,
    #[serde(default)]
    pub preferences: JsonMap,
    #[serde(default)]
    pub context: JsonMap,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoEpisodeUrlRequest<T = serde_json::Value, E = serde_json::Value> {
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
pub struct NovelChapterRequest<T = serde_json::Value> {
    pub item: T,
    #[serde(default)]
    pub page: Option<u32>,
    #[serde(default)]
    pub source_id: Option<String>,
    #[serde(default)]
    pub preferences: JsonMap,
    #[serde(default)]
    pub context: JsonMap,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NovelChapterPage {
    pub entries: Vec<NovelChapter>,
    #[serde(default)]
    pub has_next_page: bool,
    #[serde(default)]
    pub section: Option<String>,
    #[serde(default)]
    pub next_page: Option<u32>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NovelChapterUrlRequest<T = serde_json::Value, C = serde_json::Value> {
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
pub struct VideoHosterRequest<T = serde_json::Value, E = serde_json::Value> {
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
pub struct VideoHosterStreamsRequest<T = serde_json::Value, E = serde_json::Value> {
    pub item: T,
    pub episode: E,
    pub hoster: VideoHoster,
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
    Range {
        id: String,
        title: String,
        #[serde(default)]
        min: Option<f64>,
        #[serde(default)]
        max: Option<f64>,
        #[serde(default)]
        step: Option<f64>,
        #[serde(default)]
        default: Option<RangeSelection>,
    },
    Sort {
        id: String,
        title: String,
        options: Vec<SortOption>,
        #[serde(default)]
        default: Option<SortSelection>,
    },
    Group {
        title: String,
        filters: Vec<FilterDefinition>,
        #[serde(default)]
        collapsed: bool,
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

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RangeSelection {
    #[serde(default)]
    pub from: Option<f64>,
    #[serde(default)]
    pub to: Option<f64>,
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
    Stepper {
        id: String,
        title: String,
        #[serde(default)]
        summary: Option<String>,
        #[serde(default)]
        min: Option<f64>,
        #[serde(default)]
        max: Option<f64>,
        #[serde(default)]
        step: Option<f64>,
        #[serde(default)]
        default: Option<f64>,
    },
    Segment {
        id: String,
        title: String,
        options: Vec<OptionItem>,
        #[serde(default)]
        summary: Option<String>,
        #[serde(default)]
        default: Option<String>,
    },
    Picker {
        id: String,
        title: String,
        options: Vec<OptionItem>,
        #[serde(default)]
        summary: Option<String>,
        #[serde(default)]
        default: Option<String>,
    },
    EditableList {
        id: String,
        title: String,
        #[serde(default)]
        summary: Option<String>,
        #[serde(default)]
        default: Vec<String>,
        #[serde(default)]
        placeholder: Option<String>,
    },
    Login {
        id: String,
        title: String,
        #[serde(default)]
        summary: Option<String>,
        #[serde(default)]
        username_label: Option<String>,
        #[serde(default)]
        password_label: Option<String>,
        #[serde(default)]
        webview_url: Option<String>,
    },
    Group {
        title: String,
        preferences: Vec<PreferenceDefinition>,
        #[serde(default)]
        summary: Option<String>,
        #[serde(default)]
        collapsed: bool,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abi::{WebViewRequest, WebViewScript, WebViewWait};
    use crate::manifest::{
        ContentRating, ContentType, SourceCapabilities, SourceManifest, UrlPattern, UrlPatternKind,
    };

    #[test]
    fn serializes_endgame_extension_fields() {
        let source = SourceManifest {
            id: "example".to_string(),
            name: "Example".to_string(),
            lang: "en".to_string(),
            base_url: Some("https://example.test".to_string()),
            content_type: ContentType::Video,
            content_rating: ContentRating::Safe,
            capabilities: SourceCapabilities {
                search: true,
                latest: true,
                filters: true,
                preferences: true,
                home: true,
                hoster_resolution: true,
                url_resolution: true,
            },
            listings: Vec::new(),
            url_patterns: vec![UrlPattern {
                pattern: "https://example.test/watch/*".to_string(),
                kind: Some(UrlPatternKind::Episode),
            }],
            tags: Vec::new(),
        };
        let source_value = serde_json::to_value(source).unwrap();
        assert_eq!(source_value["urlPatterns"][0]["kind"], "episode");
        assert_eq!(source_value["capabilities"]["urlResolution"], true);

        let webview = WebViewRequest {
            url: "https://example.test".to_string(),
            wait_for: Some(WebViewWait::Selector {
                selector: "#app".to_string(),
            }),
            scripts: vec![WebViewScript {
                id: Some("signature".to_string()),
                script: "window.signature".to_string(),
                run_at: None,
            }],
            return_html: true,
            ..Default::default()
        };
        let webview_value = serde_json::to_value(webview).unwrap();
        assert_eq!(webview_value["waitFor"]["type"], "selector");
        assert_eq!(webview_value["scripts"][0]["id"], "signature");
        assert_eq!(webview_value["returnHtml"], true);

        let video = VideoEpisode {
            key: "episode-1".to_string(),
            release_group: Some("Group".to_string()),
            variant: Some("sub".to_string()),
            labels: vec!["1080p".to_string()],
            is_filler: true,
            ..Default::default()
        };
        let video_value = serde_json::to_value(video).unwrap();
        assert_eq!(video_value["releaseGroup"], "Group");
        assert_eq!(video_value["isFiller"], true);

        let novel_text = NovelText {
            html: Some("<p>Hello</p>".to_string()),
            javascript: Some("reader()".to_string()),
            uses_web_storage: true,
            image_request: Some(ImageRequest {
                url: "https://example.test/image.jpg".to_string(),
                method: Some("GET".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let novel_value = serde_json::to_value(novel_text).unwrap();
        assert_eq!(novel_value["usesWebStorage"], true);
        assert_eq!(
            novel_value["imageRequest"]["url"],
            "https://example.test/image.jpg"
        );

        let stream = VideoStream {
            url: "https://cdn.example.test/master.m3u8".to_string(),
            is_hls: true,
            headers: [("Referer".to_string(), "https://example.test/".to_string())].into(),
            mpv_args: vec![VideoPlayerArg::new("profile", "fast")],
            ffmpeg_stream_args: vec![VideoPlayerArg::new("user_agent", "Manatan")],
            ffmpeg_video_args: vec![VideoPlayerArg::new(
                "headers",
                "Referer: https://example.test/",
            )],
            ..Default::default()
        };
        let stream_value = serde_json::to_value(stream).unwrap();
        assert_eq!(stream_value["headers"]["Referer"], "https://example.test/");
        assert_eq!(stream_value["mpvArgs"][0]["name"], "profile");
        assert_eq!(stream_value["mpvArgs"][0]["value"], "fast");
        assert_eq!(stream_value["ffmpegStreamArgs"][0]["name"], "user_agent");
        assert_eq!(stream_value["ffmpegVideoArgs"][0]["name"], "headers");
    }
}
