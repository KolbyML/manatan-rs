use serde::{Deserialize, Serialize};

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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionError {
    pub message: String,
}

pub type ExtensionResult<T> = Result<T, ExtensionError>;

#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub extern "C" fn manatan_alloc(len: usize) -> *mut u8 {
    let mut buffer = Vec::<u8>::with_capacity(len);
    let ptr = buffer.as_mut_ptr();
    core::mem::forget(buffer);
    ptr
}

#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn manatan_dealloc(ptr: *mut u8, len: usize) {
    if !ptr.is_null() && len > 0 {
        let _ = unsafe { Vec::from_raw_parts(ptr, len, len) };
    }
}

#[cfg(target_arch = "wasm32")]
pub fn host_call(operation: &str, payload: &[u8]) -> Result<Vec<u8>, ExtensionError> {
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

#[macro_export]
macro_rules! manatan_json_export {
    ($export_name:ident, $handler:path) => {
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $export_name(ptr: u32, len: u32) -> u64 {
            let input = unsafe { core::slice::from_raw_parts(ptr as *const u8, len as usize) };
            let result = (|| -> $crate::abi::ExtensionResult<Vec<u8>> {
                let value =
                    serde_json::from_slice(input).map_err(|error| $crate::abi::ExtensionError {
                        message: format!("request decode error: {error}"),
                    })?;
                let output = $handler(value)?;
                serde_json::to_vec(&Ok::<_, $crate::abi::ExtensionError>(output)).map_err(|error| {
                    $crate::abi::ExtensionError {
                        message: format!("response encode error: {error}"),
                    }
                })
            })();
            let bytes = match result {
                Ok(bytes) => bytes,
                Err(error) => serde_json::to_vec(&Err::<serde_json::Value, _>(error))
                    .unwrap_or_else(|_| b"{\"Err\":{\"message\":\"fatal encode error\"}}".to_vec()),
            };
            let len = bytes.len() as u32;
            let ptr = $crate::abi::manatan_alloc(bytes.len()) as *mut u8;
            unsafe { core::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len()) };
            $crate::abi::pack_ptr_len(ptr as u32, len)
        }
    };
}
