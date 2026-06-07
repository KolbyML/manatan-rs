use std::{
    env,
    error::Error,
    ffi::OsStr,
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
    process::Command,
};

use manatan_extension::{
    ContentType, ExtensionArchive, ExtensionManifest, MANIFEST_FILE, MODULE_FILE, exports,
    parse_archive, runner::ExtensionRunner,
};
use serde_json::Value;
use zip::{CompressionMethod, ZipWriter, write::FileOptions};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    let Some(command) = args.next() else {
        print_usage();
        return Ok(());
    };

    match command.as_str() {
        "package" | "pkg" => {
            let extension_dir = args
                .next()
                .map(PathBuf::from)
                .unwrap_or(env::current_dir()?);
            let output = args.next().map(PathBuf::from);
            package_extension(&extension_dir, output.as_deref())?;
        }
        "verify" => {
            let files = collect_paths(args);
            if files.is_empty() {
                return Err("usage: manatan-dev verify <package.manatan> [...]".into());
            }
            for path in files {
                verify_package(&path)?;
            }
        }
        "call" => {
            let package = required_arg(&mut args, "package.manatan")?;
            let export = required_arg(&mut args, "export")?;
            let input = args.next().unwrap_or_else(|| "{}".to_string());
            call_export(Path::new(&package), &export, &input)?;
        }
        "smoke" => {
            let package = required_arg(&mut args, "package.manatan")?;
            smoke_package(Path::new(&package))?;
        }
        _ => {
            print_usage();
            return Err(format!("unknown command {command:?}").into());
        }
    }

    Ok(())
}

fn print_usage() {
    eprintln!(
        "Usage:
  manatan-dev package [extension-dir] [output.manatan]
  manatan-dev verify <package.manatan> [...]
  manatan-dev smoke <package.manatan>
  manatan-dev call <package.manatan> <export> [json]"
    );
}

fn required_arg(args: &mut impl Iterator<Item = String>, name: &str) -> Result<String> {
    args.next()
        .ok_or_else(|| format!("missing required argument <{name}>").into())
}

fn collect_paths(args: impl Iterator<Item = String>) -> Vec<PathBuf> {
    args.map(PathBuf::from).collect()
}

fn package_extension(extension_dir: &Path, output: Option<&Path>) -> Result<()> {
    if !extension_dir.exists() {
        return Err(format!(
            "extension directory does not exist: {}",
            extension_dir.display()
        )
        .into());
    }

    let manifest_path = extension_dir.join(MANIFEST_FILE);
    let manifest = read_manifest(&manifest_path)?;
    run_cargo_build(extension_dir)?;
    let wasm_path = find_release_wasm(extension_dir)?;

    let output_path = output
        .map(Path::to_path_buf)
        .unwrap_or_else(|| extension_dir.join(format!("{}.manatan", manifest.package_id)));
    create_package(extension_dir, &wasm_path, &output_path)?;
    let bytes = fs::read(&output_path)?;
    let archive = parse_archive(&bytes)?;
    print_package_summary(&output_path, &archive, bytes.len() as u64);
    Ok(())
}

fn run_cargo_build(extension_dir: &Path) -> Result<()> {
    let status = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .current_dir(extension_dir)
        .status()?;
    if !status.success() {
        return Err("cargo build failed".into());
    }
    Ok(())
}

fn find_release_wasm(extension_dir: &Path) -> Result<PathBuf> {
    let build_dir = extension_dir
        .join("target")
        .join("wasm32-unknown-unknown")
        .join("release");
    let entries = fs::read_dir(&build_dir)
        .map_err(|error| format!("failed to read {}: {error}", build_dir.display()))?;

    let mut wasm_files = Vec::new();
    for entry in entries {
        let path = entry?.path();
        if path.extension() == Some(OsStr::new("wasm")) {
            wasm_files.push(path);
        }
    }
    wasm_files.sort();
    wasm_files
        .into_iter()
        .next()
        .ok_or_else(|| format!("no wasm file found in {}", build_dir.display()).into())
}

fn create_package(extension_dir: &Path, wasm_path: &Path, output_path: &Path) -> Result<()> {
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let file = File::create(output_path)?;
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .unix_permissions(0o644);

    add_file(
        &mut zip,
        options,
        &extension_dir.join(MANIFEST_FILE),
        MANIFEST_FILE,
    )?;
    add_file(&mut zip, options, wasm_path, MODULE_FILE)?;
    add_optional_file(&mut zip, options, extension_dir, "filters.json")?;
    add_optional_file(&mut zip, options, extension_dir, "preferences.json")?;
    add_optional_dir(
        &mut zip,
        options,
        &extension_dir.join("assets"),
        Path::new("assets"),
    )?;
    zip.finish()?;
    Ok(())
}

fn add_file(
    zip: &mut ZipWriter<File>,
    options: FileOptions,
    path: &Path,
    archive_name: &str,
) -> Result<()> {
    let mut bytes = Vec::new();
    File::open(path)?.read_to_end(&mut bytes)?;
    zip.start_file(archive_name, options)?;
    zip.write_all(&bytes)?;
    Ok(())
}

fn add_optional_file(
    zip: &mut ZipWriter<File>,
    options: FileOptions,
    extension_dir: &Path,
    name: &str,
) -> Result<()> {
    let path = extension_dir.join(name);
    if path.exists() {
        add_file(zip, options, &path, name)?;
    }
    Ok(())
}

fn add_optional_dir(
    zip: &mut ZipWriter<File>,
    options: FileOptions,
    path: &Path,
    archive_prefix: &Path,
) -> Result<()> {
    if !path.exists() {
        return Ok(());
    }

    let mut entries = Vec::new();
    collect_files(path, &mut entries)?;
    entries.sort();
    for entry in entries {
        let relative = entry.strip_prefix(path)?;
        let archive_name = archive_prefix
            .join(relative)
            .to_string_lossy()
            .replace('\\', "/");
        add_file(zip, options, &entry, &archive_name)?;
    }
    Ok(())
}

fn collect_files(dir: &Path, entries: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            collect_files(&path, entries)?;
        } else if path.is_file() {
            entries.push(path);
        }
    }
    Ok(())
}

fn verify_package(path: &Path) -> Result<()> {
    let bytes = fs::read(path)?;
    let archive = parse_archive(&bytes)?;
    print_package_summary(path, &archive, bytes.len() as u64);
    Ok(())
}

fn print_package_summary(path: &Path, archive: &ExtensionArchive, size: u64) {
    let manifest = &archive.manifest;
    println!("{}:", path.display());
    println!("  packageId: {}", manifest.package_id);
    println!("  name: {}", manifest.name);
    println!(
        "  version: {} ({})",
        manifest.version, manifest.version_code
    );
    println!(
        "  contentType: {}",
        content_type_name(&manifest.content_type)
    );
    println!("  sources: {}", manifest.sources.len());
    println!("  module: {} bytes", archive.module.len());
    println!("  size: {size} bytes");
    println!(
        "  optional: filters={} preferences={}",
        archive.filters.is_some(),
        archive.preferences.is_some()
    );
}

fn call_export(path: &Path, export: &str, input: &str) -> Result<()> {
    let archive = load_archive(path)?;
    let request: Value = serde_json::from_str(input)?;
    let response = ExtensionRunner::new(archive).call_value(export, request)?;
    println!("{}", serde_json::to_string_pretty(&response)?);
    Ok(())
}

fn smoke_package(path: &Path) -> Result<()> {
    let archive = load_archive(path)?;
    let export = match archive.manifest.content_type {
        ContentType::Manga => exports::MANGA_GET_LIST,
        ContentType::Video => exports::VIDEO_GET_LIST,
        ContentType::Novel => exports::NOVEL_GET_LIST,
    };
    let response = ExtensionRunner::new(archive).call_value(export, serde_json::json!({}))?;
    println!("{}", serde_json::to_string_pretty(&response)?);
    Ok(())
}

fn load_archive(path: &Path) -> Result<ExtensionArchive> {
    let bytes = fs::read(path)?;
    Ok(parse_archive(&bytes)?)
}

fn read_manifest(path: &Path) -> Result<ExtensionManifest> {
    let bytes = fs::read(path)?;
    Ok(serde_json::from_slice(&bytes)?)
}

fn content_type_name(content_type: &ContentType) -> &'static str {
    match content_type {
        ContentType::Manga => "manga",
        ContentType::Video => "video",
        ContentType::Novel => "novel",
    }
}
