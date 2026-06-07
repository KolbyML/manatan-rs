use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const IMPORT_MODULE: &str = "manatan";

pub fn pack_ptr_len(ptr: u32, len: u32) -> u64 {
    ((len as u64) << 32) | ptr as u64
}

pub fn unpack_ptr_len(value: u64) -> (u32, u32) {
    (value as u32, (value >> 32) as u32)
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    #[serde(default)]
    pub headers: Vec<(String, String)>,
    #[serde(default)]
    pub body_base64: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpResponse {
    pub status: u16,
    #[serde(default)]
    pub headers: Vec<(String, String)>,
    pub final_url: String,
    #[serde(default)]
    pub body_base64: Option<String>,
    #[serde(default)]
    pub text: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageRequest {
    #[serde(default)]
    pub namespace: Option<String>,
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub value: Option<Value>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageResponse {
    #[serde(default)]
    pub value: Option<Value>,
    #[serde(default)]
    pub entries: Vec<(String, Value)>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CookieRecord {
    pub name: String,
    pub value: String,
    pub domain: String,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub secure: Option<bool>,
    #[serde(default)]
    pub http_only: Option<bool>,
    #[serde(default)]
    pub expires_at: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CookieRequest {
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub cookies: Vec<CookieRecord>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CookieResponse {
    #[serde(default)]
    pub header: Option<String>,
    #[serde(default)]
    pub cookies: Vec<CookieRecord>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebViewRequest {
    pub url: String,
    #[serde(default)]
    pub wait_for: Option<String>,
    #[serde(default)]
    pub user_agent: Option<String>,
    #[serde(default)]
    pub headers: Vec<(String, String)>,
    #[serde(default)]
    pub timeout_ms: Option<u64>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebViewResponse {
    pub final_url: String,
    #[serde(default)]
    pub html: Option<String>,
    #[serde(default)]
    pub cookies: Vec<CookieRecord>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionError {
    pub message: String,
}

pub type ExtensionResult<T> = Result<T, ExtensionError>;

#[cfg(target_arch = "wasm32")]
pub fn host_call_json<I, O>(operation: &str, payload: &I) -> ExtensionResult<O>
where
    I: Serialize,
    O: DeserializeOwned,
{
    let payload = serde_json::to_vec(payload).map_err(|error| ExtensionError {
        message: format!("host request encode error: {error}"),
    })?;
    let response = host_call(operation, &payload)?;
    serde_json::from_slice::<ExtensionResult<O>>(&response).map_err(|error| ExtensionError {
        message: format!("host response decode error: {error}"),
    })?
}

#[cfg(not(target_arch = "wasm32"))]
pub fn host_call_json<I, O>(operation: &str, _payload: &I) -> ExtensionResult<O>
where
    I: Serialize,
    O: DeserializeOwned,
{
    Err(ExtensionError {
        message: format!("host operation {operation:?} is only available in WASM"),
    })
}

pub fn http_fetch(request: &HttpRequest) -> ExtensionResult<HttpResponse> {
    host_call_json("http.fetch", request)
}

pub fn storage_get(
    namespace: impl Into<String>,
    key: impl Into<String>,
) -> ExtensionResult<Option<Value>> {
    let response: StorageResponse = host_call_json(
        "storage.get",
        &StorageRequest {
            namespace: Some(namespace.into()),
            key: Some(key.into()),
            value: None,
        },
    )?;
    Ok(response.value)
}

pub fn storage_set(
    namespace: impl Into<String>,
    key: impl Into<String>,
    value: Value,
) -> ExtensionResult<()> {
    let _: StorageResponse = host_call_json(
        "storage.set",
        &StorageRequest {
            namespace: Some(namespace.into()),
            key: Some(key.into()),
            value: Some(value),
        },
    )?;
    Ok(())
}

pub fn storage_delete(namespace: impl Into<String>, key: impl Into<String>) -> ExtensionResult<()> {
    let _: StorageResponse = host_call_json(
        "storage.delete",
        &StorageRequest {
            namespace: Some(namespace.into()),
            key: Some(key.into()),
            value: None,
        },
    )?;
    Ok(())
}

pub fn storage_list(namespace: impl Into<String>) -> ExtensionResult<Vec<(String, Value)>> {
    let response: StorageResponse = host_call_json(
        "storage.list",
        &StorageRequest {
            namespace: Some(namespace.into()),
            key: None,
            value: None,
        },
    )?;
    Ok(response.entries)
}

pub fn cookies_get(url: impl Into<String>) -> ExtensionResult<CookieResponse> {
    host_call_json(
        "cookies.get",
        &CookieRequest {
            url: Some(url.into()),
            cookies: Vec::new(),
        },
    )
}

pub fn cookies_set(cookies: Vec<CookieRecord>) -> ExtensionResult<()> {
    let _: CookieResponse = host_call_json("cookies.set", &CookieRequest { url: None, cookies })?;
    Ok(())
}

pub fn webview_open(request: &WebViewRequest) -> ExtensionResult<WebViewResponse> {
    host_call_json("webview.open", request)
}

#[unsafe(no_mangle)]
pub extern "C" fn manatan_alloc(len: usize) -> *mut u8 {
    let mut buffer = Vec::<u8>::with_capacity(len);
    let ptr = buffer.as_mut_ptr();
    core::mem::forget(buffer);
    ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn manatan_dealloc(ptr: *mut u8, len: usize) {
    if !ptr.is_null() && len > 0 {
        let _ = unsafe { Vec::from_raw_parts(ptr, len, len) };
    }
}

#[cfg(target_arch = "wasm32")]
pub fn host_call(operation: &str, payload: &[u8]) -> Result<Vec<u8>, ExtensionError> {
    #[link(wasm_import_module = "manatan")]
    unsafe extern "C" {
        #[link_name = "manatan_host_call"]
        fn manatan_host_call(
            op_ptr: *const u8,
            op_len: usize,
            payload_ptr: *const u8,
            payload_len: usize,
        ) -> i32;
        #[link_name = "manatan_host_len"]
        fn manatan_host_len(rid: i32) -> i32;
        #[link_name = "manatan_host_read"]
        fn manatan_host_read(rid: i32, ptr: *mut u8) -> i32;
        #[link_name = "manatan_host_free"]
        fn manatan_host_free(rid: i32);
    }

    let rid = unsafe {
        manatan_host_call(
            operation.as_ptr(),
            operation.len(),
            payload.as_ptr(),
            payload.len(),
        )
    };
    if rid < 0 {
        return Err(ExtensionError {
            message: format!("host call failed: {rid}"),
        });
    }
    let len = unsafe { manatan_host_len(rid) };
    if len < 0 {
        unsafe { manatan_host_free(rid) };
        return Err(ExtensionError {
            message: format!("host call length failed: {len}"),
        });
    }
    let mut bytes = vec![0_u8; len as usize];
    let read = unsafe { manatan_host_read(rid, bytes.as_mut_ptr()) };
    unsafe { manatan_host_free(rid) };
    if read < 0 {
        return Err(ExtensionError {
            message: format!("host call read failed: {read}"),
        });
    }
    Ok(bytes)
}
