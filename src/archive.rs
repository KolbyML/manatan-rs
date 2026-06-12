use std::io::{Cursor, Read};

use serde_json::Value;
use thiserror::Error;
use zip::ZipArchive;

use crate::manifest::{
    CURRENT_SCHEMA_VERSION, ExtensionManifest, MANIFEST_FILE, MODULE_FILE, SourceManifest,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionArchive {
    pub manifest: ExtensionManifest,
    pub module: Vec<u8>,
    pub filters: Option<Value>,
    pub preferences: Option<Value>,
}

#[derive(Debug, Error)]
pub enum ArchiveError {
    #[error("package zip could not be read: {0}")]
    Zip(#[from] zip::result::ZipError),
    #[error("package entry could not be read: {0}")]
    Read(#[from] std::io::Error),
    #[error("manifest is missing")]
    MissingManifest,
    #[error("module is missing")]
    MissingModule,
    #[error("manifest could not be parsed: {0}")]
    Manifest(#[from] serde_json::Error),
    #[error("unsupported schema version {0}")]
    UnsupportedSchema(u32),
    #[error("invalid package id {0:?}")]
    InvalidPackageId(String),
    #[error("invalid source id {0:?}")]
    InvalidSourceId(String),
    #[error("manifest must declare at least one source")]
    MissingSources,
    #[error("source {0:?} content type must match package content type")]
    SourceContentTypeMismatch(String),
}

pub fn parse_archive(bytes: &[u8]) -> Result<ExtensionArchive, ArchiveError> {
    let mut zip = ZipArchive::new(Cursor::new(bytes))?;
    let manifest_bytes =
        read_zip_entry(&mut zip, MANIFEST_FILE)?.ok_or(ArchiveError::MissingManifest)?;
    let mut manifest = serde_json::from_slice::<ExtensionManifest>(&manifest_bytes)?;
    normalize_manifest_permissions(&mut manifest);
    validate_manifest(&manifest)?;
    let module = read_zip_entry(&mut zip, MODULE_FILE)?.ok_or(ArchiveError::MissingModule)?;
    let filters = read_json_entry(&mut zip, "filters.json")?;
    let preferences = read_json_entry(&mut zip, "preferences.json")?;
    Ok(ExtensionArchive {
        manifest,
        module,
        filters,
        preferences,
    })
}

fn normalize_manifest_permissions(manifest: &mut ExtensionManifest) {
    for network in std::mem::take(&mut manifest.network) {
        if !manifest.permissions.network.contains(&network) {
            manifest.permissions.network.push(network);
        }
    }
    manifest.permissions.webview |= manifest.webview;
    manifest.permissions.cookies |= manifest.cookies;
    manifest.permissions.storage |= manifest.storage;
    manifest.webview = false;
    manifest.cookies = false;
    manifest.storage = false;
}

pub fn validate_manifest(manifest: &ExtensionManifest) -> Result<(), ArchiveError> {
    if manifest.schema_version != CURRENT_SCHEMA_VERSION {
        return Err(ArchiveError::UnsupportedSchema(manifest.schema_version));
    }
    if !is_valid_id(&manifest.package_id) {
        return Err(ArchiveError::InvalidPackageId(manifest.package_id.clone()));
    }
    if manifest.sources.is_empty() {
        return Err(ArchiveError::MissingSources);
    }
    for source in &manifest.sources {
        validate_source(manifest, source)?;
    }
    Ok(())
}

fn validate_source(
    manifest: &ExtensionManifest,
    source: &SourceManifest,
) -> Result<(), ArchiveError> {
    if !is_valid_id(&source.id) {
        return Err(ArchiveError::InvalidSourceId(source.id.clone()));
    }
    if source.content_type != manifest.content_type {
        return Err(ArchiveError::SourceContentTypeMismatch(source.id.clone()));
    }
    Ok(())
}

fn read_json_entry(
    zip: &mut ZipArchive<Cursor<&[u8]>>,
    name: &str,
) -> Result<Option<Value>, ArchiveError> {
    let Some(bytes) = read_zip_entry(zip, name)? else {
        return Ok(None);
    };
    Ok(Some(serde_json::from_slice::<Value>(&bytes)?))
}

fn read_zip_entry(
    zip: &mut ZipArchive<Cursor<&[u8]>>,
    name: &str,
) -> Result<Option<Vec<u8>>, ArchiveError> {
    let Ok(mut file) = zip.by_name(name) else {
        return Ok(None);
    };
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    Ok(Some(bytes))
}

fn is_valid_id(value: &str) -> bool {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_lowercase() || first.is_ascii_digit()) {
        return false;
    }
    chars.all(|ch| {
        ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '.' || ch == '_' || ch == '-'
    })
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use zip::{CompressionMethod, ZipWriter, write::FileOptions};

    use super::*;

    #[test]
    fn parses_valid_package() {
        let manifest = r#"{
            "schemaVersion": 1,
            "packageId": "example-manga",
            "name": "Example",
            "version": "1.0.0",
            "versionCode": 1,
            "contentType": "manga",
            "sources": [{
                "id": "example",
                "name": "Example",
                "lang": "en",
                "contentType": "manga",
                "capabilities": { "search": true }
            }]
        }"#;
        let bytes = package_bytes(manifest, b"\0asm");
        let archive = parse_archive(&bytes).expect("valid package");
        assert_eq!(archive.manifest.package_id, "example-manga");
        assert_eq!(archive.module, b"\0asm");
    }

    #[test]
    fn rejects_source_content_type_mismatch() {
        let manifest = r#"{
            "schemaVersion": 1,
            "packageId": "example-mismatch",
            "name": "Example",
            "version": "1.0.0",
            "versionCode": 1,
            "contentType": "manga",
            "sources": [{
                "id": "example",
                "name": "Example",
                "lang": "en",
                "contentType": "video"
            }]
        }"#;
        let bytes = package_bytes(manifest, b"\0asm");
        assert!(matches!(
            parse_archive(&bytes),
            Err(ArchiveError::SourceContentTypeMismatch(_))
        ));
    }

    #[test]
    fn merges_legacy_top_level_permissions_into_permissions_object() {
        let manifest = r#"{
            "schemaVersion": 1,
            "packageId": "example-video",
            "name": "Example",
            "version": "1.0.0",
            "versionCode": 1,
            "contentType": "video",
            "network": ["https://example.com"],
            "webview": true,
            "cookies": true,
            "sources": [{
                "id": "example",
                "name": "Example",
                "lang": "en",
                "contentType": "video"
            }]
        }"#;
        let bytes = package_bytes(manifest, b"\0asm");
        let archive = parse_archive(&bytes).expect("valid package");
        assert_eq!(
            archive.manifest.permissions.network,
            ["https://example.com"]
        );
        assert!(archive.manifest.permissions.webview);
        assert!(archive.manifest.permissions.cookies);
    }

    #[test]
    fn rejects_empty_package_id() {
        let manifest = r#"{
            "schemaVersion": 1,
            "packageId": "",
            "name": "Example",
            "version": "1.0.0",
            "versionCode": 1,
            "contentType": "manga",
            "sources": [{
                "id": "example",
                "name": "Example",
                "lang": "en",
                "contentType": "manga"
            }]
        }"#;
        let bytes = package_bytes(manifest, b"\0asm");
        assert!(matches!(
            parse_archive(&bytes),
            Err(ArchiveError::InvalidPackageId(_))
        ));
    }

    fn package_bytes(manifest: &str, module: &[u8]) -> Vec<u8> {
        let cursor = Cursor::new(Vec::new());
        let mut zip = ZipWriter::new(cursor);
        let options = FileOptions::default().compression_method(CompressionMethod::Stored);
        zip.start_file(MANIFEST_FILE, options).unwrap();
        zip.write_all(manifest.as_bytes()).unwrap();
        zip.start_file(MODULE_FILE, options).unwrap();
        zip.write_all(module).unwrap();
        zip.finish().unwrap().into_inner()
    }
}
