use std::{
    collections::HashMap,
    panic::AssertUnwindSafe,
    sync::{Arc, Mutex},
};

use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use thiserror::Error;
#[cfg(target_os = "ios")]
use wasmer::wasmi::Wasmi;
use wasmer::{
    AsStoreRef, Function, FunctionEnv, FunctionEnvMut, Imports, Instance, Memory, Module, Store,
};

use crate::{
    abi::{IMPORT_MODULE, unpack_ptr_len},
    archive::ExtensionArchive,
};

#[derive(Debug, Error)]
pub enum RunnerError {
    #[error("wasm module error: {0}")]
    Module(String),
    #[error("wasm instance error: {0}")]
    Instance(String),
    #[error("wasm memory is missing")]
    MissingMemory,
    #[error("wasm export {0:?} is missing")]
    MissingExport(String),
    #[error("wasm allocation failed")]
    AllocationFailed,
    #[error("wasm memory error: {0}")]
    Memory(String),
    #[error("wasm call error: {0}")]
    Call(String),
    #[error("extension error: {0}")]
    Extension(String),
    #[error("host call error: {0}")]
    Host(String),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}

pub trait HostCall: Send + Sync {
    fn call(&self, operation: &str, payload: &[u8]) -> Result<Vec<u8>, RunnerError>;
}

#[derive(Default)]
pub struct EmptyHost;

impl HostCall for EmptyHost {
    fn call(&self, operation: &str, payload: &[u8]) -> Result<Vec<u8>, RunnerError> {
        match operation {
            "cookies.get" => host_ok(serde_json::json!({
                "header": null,
                "cookies": []
            })),
            "cookies.set" => host_ok(serde_json::json!({
                "header": null,
                "cookies": []
            })),
            "storage.get" => host_ok(serde_json::json!({
                "value": null,
                "entries": []
            })),
            "storage.set" | "storage.delete" => host_ok(serde_json::json!({
                "value": null,
                "entries": []
            })),
            "storage.list" => host_ok(serde_json::json!({
                "value": null,
                "entries": []
            })),
            "system.time" => {
                let unix_millis = current_unix_millis()?;
                host_ok(serde_json::json!({
                    "unixMillis": unix_millis,
                    "unixSeconds": unix_millis / 1_000
                }))
            }
            "system.randomBytes" => {
                let request: serde_json::Value = serde_json::from_slice(payload)?;
                let length = request
                    .get("length")
                    .and_then(serde_json::Value::as_u64)
                    .unwrap_or(0)
                    .min(4096) as usize;
                let mut state = current_unix_millis()? as u64;
                let bytes = (0..length)
                    .map(|_| {
                        state ^= state << 13;
                        state ^= state >> 7;
                        state ^= state << 17;
                        state as u8
                    })
                    .collect::<Vec<_>>();
                host_ok(serde_json::json!({
                    "bytesBase64": base64_encode(&bytes)
                }))
            }
            _ => Err(RunnerError::Host(format!(
                "unsupported host operation {operation:?} with payload {} byte(s)",
                payload.len()
            ))),
        }
    }
}

fn host_ok(value: Value) -> Result<Vec<u8>, RunnerError> {
    serde_json::to_vec(&Ok::<Value, crate::abi::ExtensionError>(value)).map_err(RunnerError::from)
}

fn base64_encode(bytes: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0];
        let b1 = *chunk.get(1).unwrap_or(&0);
        let b2 = *chunk.get(2).unwrap_or(&0);
        out.push(TABLE[(b0 >> 2) as usize] as char);
        out.push(TABLE[(((b0 & 0x03) << 4) | (b1 >> 4)) as usize] as char);
        out.push(if chunk.len() > 1 {
            TABLE[(((b1 & 0x0f) << 2) | (b2 >> 6)) as usize] as char
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            TABLE[(b2 & 0x3f) as usize] as char
        } else {
            '='
        });
    }
    out
}

fn current_unix_millis() -> Result<i64, RunnerError> {
    let duration = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|error| RunnerError::Host(format!("system time error: {error}")))?;
    Ok(duration.as_millis() as i64)
}

#[derive(Clone)]
struct RunnerEnv {
    memory: Option<Memory>,
    host: Arc<dyn HostCall>,
    results: Arc<Mutex<HostResults>>,
}

#[derive(Default)]
struct HostResults {
    next_rid: i32,
    items: HashMap<i32, Vec<u8>>,
}

impl RunnerEnv {
    fn new(host: Arc<dyn HostCall>) -> Self {
        Self {
            memory: None,
            host,
            results: Arc::new(Mutex::new(HostResults {
                next_rid: 1,
                items: HashMap::new(),
            })),
        }
    }

    fn read_bytes(
        &self,
        store: impl AsStoreRef,
        ptr: u32,
        len: u32,
    ) -> Result<Vec<u8>, RunnerError> {
        let memory = self.memory.as_ref().ok_or(RunnerError::MissingMemory)?;
        let view = memory.view(&store);
        let offset = ptr as u64;
        let mut bytes = vec![0_u8; len as usize];
        view.read(offset, &mut bytes)
            .map_err(|error| RunnerError::Memory(error.to_string()))?;
        Ok(bytes)
    }

    fn write_bytes(
        &self,
        store: impl AsStoreRef,
        ptr: u32,
        bytes: &[u8],
    ) -> Result<(), RunnerError> {
        let memory = self.memory.as_ref().ok_or(RunnerError::MissingMemory)?;
        let view = memory.view(&store);
        view.write(ptr as u64, bytes)
            .map_err(|error| RunnerError::Memory(error.to_string()))
    }
}

pub struct ExtensionRunner {
    archive: ExtensionArchive,
    host: Arc<dyn HostCall>,
}

impl ExtensionRunner {
    pub fn new(archive: ExtensionArchive) -> Self {
        Self::with_host(archive, Arc::new(EmptyHost))
    }

    pub fn with_host(archive: ExtensionArchive, host: Arc<dyn HostCall>) -> Self {
        Self { archive, host }
    }

    pub fn call_json<I, O>(&self, export_name: &str, input: &I) -> Result<O, RunnerError>
    where
        I: Serialize,
        O: DeserializeOwned,
    {
        let request = serde_json::to_vec(input)?;
        let response = self.call_bytes(export_name, &request)?;
        let result = serde_json::from_slice::<Result<O, crate::abi::ExtensionError>>(&response)?;
        result.map_err(|error| RunnerError::Extension(error.message))
    }

    pub fn call_value(&self, export_name: &str, input: Value) -> Result<Value, RunnerError> {
        self.call_json(export_name, &input)
    }

    pub fn has_export(&self, export_name: &str) -> Result<bool, RunnerError> {
        let mut store = new_store();
        let module = Module::new(&store, &self.archive.module)
            .map_err(|error| RunnerError::Module(error.to_string()))?;
        let env = FunctionEnv::new(&mut store, RunnerEnv::new(Arc::clone(&self.host)));
        let imports = build_imports(&mut store, &env);
        let instance = instantiate_module(&mut store, &module, &imports)?;
        Ok(instance.exports.get_function(export_name).is_ok())
    }

    fn call_bytes(&self, export_name: &str, request: &[u8]) -> Result<Vec<u8>, RunnerError> {
        let mut store = new_store();
        let module = Module::new(&store, &self.archive.module)
            .map_err(|error| RunnerError::Module(error.to_string()))?;
        let env = FunctionEnv::new(&mut store, RunnerEnv::new(Arc::clone(&self.host)));
        let imports = build_imports(&mut store, &env);
        let instance = instantiate_module(&mut store, &module, &imports)?;
        let memory = instance
            .exports
            .get_memory("memory")
            .map_err(|_| RunnerError::MissingMemory)?
            .clone();
        env.as_mut(&mut store).memory = Some(memory);
        if let Ok(start) = instance
            .exports
            .get_typed_function::<(), ()>(&store, crate::exports::START)
        {
            start
                .call(&mut store)
                .map_err(|error| RunnerError::Call(error.to_string()))?;
        }

        let alloc = instance
            .exports
            .get_typed_function::<i32, i32>(&store, "manatan_alloc")
            .map_err(|_| RunnerError::MissingExport("manatan_alloc".to_string()))?;
        let dealloc = instance
            .exports
            .get_typed_function::<(i32, i32), ()>(&store, "manatan_dealloc")
            .ok();
        let call = instance
            .exports
            .get_typed_function::<(i32, i32), i64>(&store, export_name)
            .map_err(|_| RunnerError::MissingExport(export_name.to_string()))?;

        let request_ptr = alloc
            .call(&mut store, request.len() as i32)
            .map_err(|error| RunnerError::Call(error.to_string()))?;
        if request_ptr <= 0 {
            return Err(RunnerError::AllocationFailed);
        }
        env.as_ref(&store)
            .write_bytes(&store, request_ptr as u32, request)?;

        let packed = call
            .call(&mut store, request_ptr, request.len() as i32)
            .map_err(|error| RunnerError::Call(error.to_string()))?;
        if let Some(dealloc) = &dealloc {
            let _ = dealloc.call(&mut store, request_ptr, request.len() as i32);
        }
        let (response_ptr, response_len) = unpack_ptr_len(packed as u64);
        if response_ptr == 0 || response_len == 0 {
            return Err(RunnerError::Call(
                "extension returned empty response".to_string(),
            ));
        }
        let response = env
            .as_ref(&store)
            .read_bytes(&store, response_ptr, response_len)?;
        if let Some(dealloc) = &dealloc {
            let _ = dealloc.call(&mut store, response_ptr as i32, response_len as i32);
        }
        Ok(response)
    }
}

fn new_store() -> Store {
    #[cfg(target_os = "ios")]
    {
        return Store::new(Wasmi::new());
    }

    #[cfg(not(target_os = "ios"))]
    {
        Store::default()
    }
}

fn build_imports(store: &mut Store, env: &FunctionEnv<RunnerEnv>) -> Imports {
    let mut imports = Imports::new();
    imports.define(
        IMPORT_MODULE,
        "manatan_host_call",
        Function::new_typed_with_env(store, env, host_call),
    );
    imports.define(
        IMPORT_MODULE,
        "manatan_host_len",
        Function::new_typed_with_env(store, env, host_len),
    );
    imports.define(
        IMPORT_MODULE,
        "manatan_host_read",
        Function::new_typed_with_env(store, env, host_read),
    );
    imports.define(
        IMPORT_MODULE,
        "manatan_host_free",
        Function::new_typed_with_env(store, env, host_free),
    );
    imports
}

fn instantiate_module(
    store: &mut Store,
    module: &Module,
    imports: &Imports,
) -> Result<Instance, RunnerError> {
    std::panic::catch_unwind(AssertUnwindSafe(|| Instance::new(store, module, imports)))
        .map_err(|panic| {
            let message = if let Some(message) = panic.downcast_ref::<&str>() {
                *message
            } else if let Some(message) = panic.downcast_ref::<String>() {
                message.as_str()
            } else {
                "unknown panic"
            };
            RunnerError::Instance(format!("panic: {message}"))
        })?
        .map_err(|error| RunnerError::Instance(error.to_string()))
}

fn host_call(
    mut env: FunctionEnvMut<RunnerEnv>,
    op_ptr: i32,
    op_len: i32,
    payload_ptr: i32,
    payload_len: i32,
) -> i32 {
    if op_ptr < 0 || op_len < 0 || payload_ptr < 0 || payload_len < 0 {
        return -1;
    }
    let operation = match env
        .data()
        .read_bytes(env.as_store_ref(), op_ptr as u32, op_len as u32)
    {
        Ok(bytes) => String::from_utf8_lossy(&bytes).to_string(),
        Err(_) => return -2,
    };
    let payload =
        match env
            .data()
            .read_bytes(env.as_store_ref(), payload_ptr as u32, payload_len as u32)
        {
            Ok(bytes) => bytes,
            Err(_) => return -3,
        };
    let result = match env.data().host.call(&operation, &payload) {
        Ok(result) => result,
        Err(error) => {
            let error = crate::abi::ExtensionError {
                message: error.to_string(),
            };
            serde_json::to_vec(&Err::<Value, _>(error)).unwrap_or_default()
        }
    };
    let mut results = match env.data_mut().results.lock() {
        Ok(results) => results,
        Err(_) => return -4,
    };
    let rid = results.next_rid;
    results.next_rid = results.next_rid.saturating_add(1).max(1);
    results.items.insert(rid, result);
    rid
}

fn host_len(env: FunctionEnvMut<RunnerEnv>, rid: i32) -> i32 {
    let Ok(results) = env.data().results.lock() else {
        return -1;
    };
    results
        .items
        .get(&rid)
        .map(|bytes| bytes.len() as i32)
        .unwrap_or(-2)
}

fn host_read(env: FunctionEnvMut<RunnerEnv>, rid: i32, ptr: i32) -> i32 {
    if ptr < 0 {
        return -1;
    }
    let bytes = {
        let Ok(results) = env.data().results.lock() else {
            return -2;
        };
        let Some(bytes) = results.items.get(&rid) else {
            return -3;
        };
        bytes.clone()
    };
    match env
        .data()
        .write_bytes(env.as_store_ref(), ptr as u32, &bytes)
    {
        Ok(()) => bytes.len() as i32,
        Err(_) => -4,
    }
}

fn host_free(env: FunctionEnvMut<RunnerEnv>, rid: i32) {
    if let Ok(mut results) = env.data().results.lock() {
        results.items.remove(&rid);
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::manifest::{CURRENT_SCHEMA_VERSION, ContentType, ExtensionManifest, SourceManifest};

    #[test]
    fn calls_wasm_export() {
        let module = wat::parse_str(
            r#"
            (module
              (memory (export "memory") 1)
              (global $heap (mut i32) (i32.const 4096))
              (func (export "manatan_alloc") (param $len i32) (result i32)
                (local $ptr i32)
                (local.set $ptr (global.get $heap))
                (global.set $heap (i32.add (global.get $heap) (local.get $len)))
                (local.get $ptr))
              (func (export "manatan_dealloc") (param i32) (param i32))
              (data (i32.const 1024) "{\"Ok\":{\"entries\":[],\"hasNextPage\":false}}")
              (func (export "manatan_manga_search") (param i32) (param i32) (result i64)
                (i64.or
                  (i64.extend_i32_u (i32.const 1024))
                  (i64.shl
                    (i64.extend_i32_u (i32.const 41))
                    (i64.const 32)))))
            "#,
        )
        .expect("wat");
        let archive = ExtensionArchive {
            manifest: manifest(),
            module,
            filters: None,
            preferences: None,
        };
        let runner = ExtensionRunner::new(archive);
        let value = runner
            .call_value(crate::exports::MANGA_SEARCH, json!({"query":"test"}))
            .expect("call");
        assert_eq!(value["hasNextPage"], false);
        assert_eq!(value["entries"].as_array().map(Vec::len), Some(0));
    }

    #[test]
    fn detects_optional_sdk_exports() {
        let module = wat::parse_str(
            r#"
            (module
              (memory (export "memory") 1)
              (global $heap (mut i32) (i32.const 4096))
              (func (export "manatan_alloc") (param $len i32) (result i32)
                (local $ptr i32)
                (local.set $ptr (global.get $heap))
                (global.set $heap (i32.add (global.get $heap) (local.get $len)))
                (local.get $ptr))
              (func (export "manatan_dealloc") (param i32) (param i32))
              (data (i32.const 1024) "{\"Ok\":null}")
              (func $empty (param i32) (param i32) (result i64)
                (i64.or
                  (i64.extend_i32_u (i32.const 1024))
                  (i64.shl
                    (i64.extend_i32_u (i32.const 11))
                    (i64.const 32))))
              (export "manatan_get_home" (func $empty))
              (export "manatan_get_filters" (func $empty))
              (export "manatan_get_preferences" (func $empty))
              (export "manatan_manga_get_manga_url" (func $empty))
              (export "manatan_manga_get_chapter_url" (func $empty))
              (export "manatan_manga_handle_url" (func $empty))
              (export "manatan_manga_prepare_chapter" (func $empty))
              (export "manatan_manga_resolve_page_image" (func $empty))
              (export "manatan_manga_process_page_image" (func $empty))
              (export "manatan_manga_get_alternate_covers" (func $empty))
              (export "manatan_manga_get_related" (func $empty))
              (export "manatan_manga_migrate" (func $empty))
              (export "manatan_video_get_hosters" (func $empty))
              (export "manatan_video_resolve_hoster" (func $empty))
              (export "manatan_video_get_home" (func $empty))
              (export "manatan_video_get_item_url" (func $empty))
              (export "manatan_video_get_episode_url" (func $empty))
              (export "manatan_video_handle_url" (func $empty))
              (export "manatan_novel_get_home" (func $empty))
              (export "manatan_novel_get_chapters_page" (func $empty))
              (export "manatan_novel_get_novel_url" (func $empty))
              (export "manatan_novel_get_chapter_url" (func $empty))
              (export "manatan_novel_handle_url" (func $empty)))
            "#,
        )
        .expect("wat");
        let runner = ExtensionRunner::new(ExtensionArchive {
            manifest: manifest(),
            module,
            filters: None,
            preferences: None,
        });

        for export_name in [
            crate::exports::GET_HOME,
            crate::exports::GET_FILTERS,
            crate::exports::GET_PREFERENCES,
            crate::exports::MANGA_GET_MANGA_URL,
            crate::exports::MANGA_GET_CHAPTER_URL,
            crate::exports::MANGA_HANDLE_URL,
            crate::exports::MANGA_PREPARE_CHAPTER,
            crate::exports::MANGA_RESOLVE_PAGE_IMAGE,
            crate::exports::MANGA_PROCESS_PAGE_IMAGE,
            crate::exports::MANGA_GET_ALTERNATE_COVERS,
            crate::exports::MANGA_GET_RELATED,
            crate::exports::MANGA_MIGRATE,
            crate::exports::VIDEO_GET_HOSTERS,
            crate::exports::VIDEO_RESOLVE_HOSTER,
            crate::exports::VIDEO_GET_HOME,
            crate::exports::VIDEO_GET_ITEM_URL,
            crate::exports::VIDEO_GET_EPISODE_URL,
            crate::exports::VIDEO_HANDLE_URL,
            crate::exports::NOVEL_GET_HOME,
            crate::exports::NOVEL_GET_CHAPTERS_PAGE,
            crate::exports::NOVEL_GET_NOVEL_URL,
            crate::exports::NOVEL_GET_CHAPTER_URL,
            crate::exports::NOVEL_HANDLE_URL,
        ] {
            assert!(
                runner.has_export(export_name).expect("has_export"),
                "{export_name}"
            );
        }
        assert!(
            !runner
                .has_export(crate::exports::MANGA_GET_HOME)
                .expect("has_export")
        );
    }

    fn manifest() -> ExtensionManifest {
        ExtensionManifest {
            schema_version: CURRENT_SCHEMA_VERSION,
            package_id: "example-runner".to_string(),
            name: "Runner Test".to_string(),
            version: "1.0.0".to_string(),
            version_code: 1,
            minimum_manatan_version: None,
            author: None,
            description: None,
            homepage: None,
            repository: None,
            license: None,
            icon: None,
            content_type: ContentType::Manga,
            permissions: Default::default(),
            network: Vec::new(),
            webview: false,
            cookies: false,
            storage: false,
            sources: vec![SourceManifest {
                id: "runner".to_string(),
                name: "Runner".to_string(),
                lang: "en".to_string(),
                base_url: None,
                content_type: ContentType::Manga,
                content_rating: Default::default(),
                capabilities: Default::default(),
                listings: Vec::new(),
                url_patterns: Vec::new(),
                tags: Vec::new(),
            }],
        }
    }
}
