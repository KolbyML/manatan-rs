//! Media-neutral WebView extraction helpers.
//!
//! Use this module when a source needs the target site to run in a real
//! platform WebView before data exists in JavaScript state, local storage, or
//! cookies.

use crate::abi::{
    ExtensionError, ExtensionResult, WebViewExtractRequest, WebViewExtractResponse,
    WebViewRequestCapture, WebViewWait, WebViewWaitUntil, webview_extract,
};
use serde::de::DeserializeOwned;
use std::collections::BTreeMap;

pub type Headers = BTreeMap<String, String>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExtractRequest {
    pub url: String,
    pub headers: Headers,
    pub user_agent: Option<String>,
    pub wait_until: WebViewWaitUntil,
    pub wait_for_script: Option<String>,
    pub wait_for_selector: Option<String>,
    pub wait_for_event: Option<String>,
    pub script: String,
    pub timeout_ms: u64,
    pub cookies: bool,
    pub headless: Option<bool>,
    pub capture_requests: Vec<WebViewRequestCapture>,
}

impl ExtractRequest {
    pub fn new(url: impl Into<String>, script: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            headers: Headers::new(),
            user_agent: None,
            wait_until: WebViewWaitUntil::LoadFinished,
            wait_for_script: None,
            wait_for_selector: None,
            wait_for_event: None,
            script: script.into(),
            timeout_ms: 30_000,
            cookies: true,
            headless: Some(true),
            capture_requests: Vec::new(),
        }
    }

    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }

    pub fn headers(mut self, headers: Headers) -> Self {
        self.headers.extend(headers);
        self
    }

    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    pub fn wait_until(mut self, wait_until: WebViewWaitUntil) -> Self {
        self.wait_until = wait_until;
        self
    }

    pub fn wait_for_script(mut self, script: impl Into<String>) -> Self {
        self.wait_for_script = Some(script.into());
        self
    }

    pub fn wait_for_selector(mut self, selector: impl Into<String>) -> Self {
        self.wait_for_selector = Some(selector.into());
        self
    }

    pub fn wait_for_event(mut self, event: impl Into<String>) -> Self {
        self.wait_for_event = Some(event.into());
        self
    }

    pub fn timeout_ms(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = timeout_ms;
        self
    }

    pub fn cookies(mut self, enabled: bool) -> Self {
        self.cookies = enabled;
        self
    }

    pub fn headless(mut self, headless: bool) -> Self {
        self.headless = Some(headless);
        self
    }

    pub fn capture(mut self, capture: WebViewRequestCapture) -> Self {
        self.capture_requests.push(capture);
        self
    }

    pub fn capture_url_contains(
        self,
        id: impl Into<String>,
        url_contains: impl Into<String>,
    ) -> Self {
        self.capture(WebViewRequestCapture {
            id: Some(id.into()),
            url_contains: Some(url_contains.into()),
            ..WebViewRequestCapture::default()
        })
    }

    pub fn into_abi(self) -> WebViewExtractRequest {
        WebViewExtractRequest {
            url: self.url,
            headers: self.headers.into_iter().collect(),
            user_agent: self.user_agent,
            wait_until: Some(self.wait_until),
            wait_for_script: self.wait_for_script,
            wait_for_selector: self.wait_for_selector,
            wait_for_event: self.wait_for_event,
            script: self.script,
            timeout_ms: Some(self.timeout_ms),
            cookies: self.cookies,
            headless: self.headless,
            capture_requests: self.capture_requests,
        }
    }
}

pub fn extract(request: ExtractRequest) -> ExtensionResult<WebViewExtractResponse> {
    webview_extract(&request.into_abi())
}

pub fn extract_text(request: ExtractRequest) -> ExtensionResult<String> {
    let response = extract(request)?;
    response.text.ok_or_else(|| ExtensionError {
        message: "webview extraction did not return a string payload".to_string(),
    })
}

pub fn extract_json<T>(request: ExtractRequest) -> ExtensionResult<T>
where
    T: DeserializeOwned,
{
    let response = extract(request)?;
    let value = response
        .json
        .or(response.value)
        .ok_or_else(|| ExtensionError {
            message: "webview extraction did not return a JSON payload".to_string(),
        })?;
    serde_json::from_value(value).map_err(|error| ExtensionError {
        message: format!("webview extraction JSON decode error: {error}"),
    })
}

#[doc(hidden)]
pub fn wait_from_extract_request(request: &WebViewExtractRequest) -> Option<WebViewWait> {
    if let Some(selector) = request.wait_for_selector.as_ref() {
        return Some(WebViewWait::Selector {
            selector: selector.clone(),
        });
    }
    if let Some(script) = request.wait_for_script.as_ref() {
        return Some(WebViewWait::Script {
            script: script.clone(),
        });
    }
    if let Some(event) = request.wait_for_event.as_ref() {
        return Some(WebViewWait::Event {
            name: event.clone(),
        });
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_request_serializes_generic_shape() {
        let request = ExtractRequest::new(
            "https://example.com",
            "Promise.resolve(JSON.stringify(window.__MANATAN_EXAMPLE_DATA__ || []))",
        )
        .wait_for_script("Array.isArray(window.__MANATAN_EXAMPLE_DATA__)")
        .timeout_ms(10_000)
        .header("Accept-Language", "en-US,en;q=0.9")
        .user_agent("Manatan Test")
        .cookies(true)
        .headless(true)
        .into_abi();
        let value = serde_json::to_value(request).unwrap();
        assert_eq!(value["url"], "https://example.com");
        assert_eq!(value["waitUntil"], "loadFinished");
        assert_eq!(
            value["waitForScript"],
            "Array.isArray(window.__MANATAN_EXAMPLE_DATA__)"
        );
        assert_eq!(value["timeoutMs"], 10_000);
        assert_eq!(value["cookies"], true);
        assert!(
            value["script"]
                .as_str()
                .unwrap()
                .contains("Promise.resolve")
        );
    }

    #[test]
    fn wait_prefers_selector_then_script_then_event() {
        let request = WebViewExtractRequest {
            url: "https://example.test".to_string(),
            wait_for_selector: Some("#app".to_string()),
            wait_for_script: Some("window.ready".to_string()),
            wait_for_event: Some("ready".to_string()),
            script: "window.payload".to_string(),
            ..default_extract_request()
        };
        assert_eq!(
            wait_from_extract_request(&request),
            Some(WebViewWait::Selector {
                selector: "#app".to_string()
            })
        );

        let request = WebViewExtractRequest {
            wait_for_selector: None,
            ..request
        };
        assert_eq!(
            wait_from_extract_request(&request),
            Some(WebViewWait::Script {
                script: "window.ready".to_string()
            })
        );

        let request = WebViewExtractRequest {
            wait_for_script: None,
            ..request
        };
        assert_eq!(
            wait_from_extract_request(&request),
            Some(WebViewWait::Event {
                name: "ready".to_string()
            })
        );
    }

    fn default_extract_request() -> WebViewExtractRequest {
        WebViewExtractRequest {
            url: String::new(),
            headers: Vec::new(),
            user_agent: None,
            wait_until: Some(WebViewWaitUntil::LoadFinished),
            wait_for_script: None,
            wait_for_selector: None,
            wait_for_event: None,
            script: String::new(),
            timeout_ms: Some(30_000),
            cookies: true,
            headless: Some(true),
            capture_requests: Vec::new(),
        }
    }
}
