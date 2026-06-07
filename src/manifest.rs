use serde::{Deserialize, Serialize};

pub const MANIFEST_FILE: &str = "manifest.json";
pub const MODULE_FILE: &str = "module.wasm";
pub const CURRENT_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ContentType {
    Manga,
    Video,
    Novel,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ContentRating {
    Safe,
    Suggestive,
    Adult,
    Unknown,
}

impl Default for ContentRating {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionManifest {
    pub schema_version: u32,
    pub package_id: String,
    pub name: String,
    pub version: String,
    pub version_code: i64,
    #[serde(default)]
    pub minimum_manatan_version: Option<String>,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub homepage: Option<String>,
    #[serde(default)]
    pub repository: Option<String>,
    #[serde(default)]
    pub license: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub content_types: Vec<ContentType>,
    #[serde(default)]
    pub permissions: Permissions,
    #[serde(default)]
    pub sources: Vec<SourceManifest>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Permissions {
    #[serde(default)]
    pub network: Vec<String>,
    #[serde(default)]
    pub webview: bool,
    #[serde(default)]
    pub cookies: bool,
    #[serde(default)]
    pub storage: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceManifest {
    pub id: String,
    pub name: String,
    pub lang: String,
    #[serde(default)]
    pub base_url: Option<String>,
    #[serde(default)]
    pub content_types: Vec<ContentType>,
    #[serde(default)]
    pub content_rating: ContentRating,
    #[serde(default)]
    pub capabilities: SourceCapabilities,
    #[serde(default)]
    pub listings: Vec<ListingManifest>,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceCapabilities {
    #[serde(default)]
    pub search: bool,
    #[serde(default)]
    pub latest: bool,
    #[serde(default)]
    pub filters: bool,
    #[serde(default)]
    pub preferences: bool,
    #[serde(default)]
    pub home: bool,
    #[serde(default)]
    pub hoster_resolution: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListingManifest {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub content_types: Vec<ContentType>,
}
