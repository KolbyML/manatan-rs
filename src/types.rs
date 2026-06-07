use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub type Context = BTreeMap<String, String>;

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
    pub cover: Option<String>,
    pub url: Option<String>,
    pub authors: Vec<String>,
    pub artists: Vec<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub status: ItemStatus,
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
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MangaPage {
    pub content: PageContent,
    pub thumbnail: Option<String>,
    pub description: Option<String>,
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

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoEpisode {
    pub key: String,
    pub title: Option<String>,
    pub episode_number: Option<f32>,
    pub season_number: Option<f32>,
    pub date_uploaded: Option<i64>,
    pub thumbnail: Option<String>,
    pub url: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoStream {
    pub url: String,
    pub quality: Option<String>,
    pub format: Option<String>,
    pub headers: Context,
    pub subtitles: Vec<SubtitleTrack>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubtitleTrack {
    pub url: String,
    pub language: Option<String>,
    pub label: Option<String>,
    pub format: Option<String>,
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
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NovelText {
    pub html: Option<String>,
    pub text: Option<String>,
}
