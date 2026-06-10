//! HTTP conveniences for Manatan host calls.
//!
//! Sources should prefer [`HttpClient::browser`] for real sites. It sends a
//! coherent browser header set, can attach the host cookie jar, and can retry
//! challenge pages through the host webview when enabled.

use crate::abi::{
    ExtensionResult, HttpRequest, HttpResponse, WebViewRequest, WebViewWait, cookies_get,
    cookies_set, http_fetch, webview_open,
};
use std::collections::BTreeMap;

pub type Headers = BTreeMap<String, String>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserProfile {
    pub user_agent: String,
    pub accept_language: String,
}

impl BrowserProfile {
    pub fn desktop() -> Self {
        Self {
            user_agent: DESKTOP_USER_AGENT.to_string(),
            accept_language: "en-US,en;q=0.9".to_string(),
        }
    }
}

impl Default for BrowserProfile {
    fn default() -> Self {
        Self::desktop()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum ChallengePolicy {
    #[default]
    Never,
    WebView {
        wait_for: Option<WebViewWait>,
        timeout_ms: Option<u64>,
    },
}

impl ChallengePolicy {
    pub fn webview() -> Self {
        Self::WebView {
            wait_for: None,
            timeout_ms: Some(45_000),
        }
    }

    pub fn with_wait_for(self, wait_for: impl Into<String>) -> Self {
        self.with_webview_wait(WebViewWait::Selector {
            selector: wait_for.into(),
        })
    }

    pub fn with_webview_wait(self, wait_for: WebViewWait) -> Self {
        match self {
            Self::Never => Self::WebView {
                wait_for: Some(wait_for),
                timeout_ms: Some(45_000),
            },
            Self::WebView { timeout_ms, .. } => Self::WebView {
                wait_for: Some(wait_for),
                timeout_ms,
            },
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct HttpClient {
    default_headers: Headers,
    browser_profile: Option<BrowserProfile>,
    cookie_url: Option<String>,
    challenge_policy: ChallengePolicy,
}

impl HttpClient {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn browser() -> Self {
        Self::new().with_browser_profile(BrowserProfile::desktop())
    }

    pub fn with_browser_profile(mut self, profile: BrowserProfile) -> Self {
        self.default_headers
            .insert("User-Agent".to_string(), profile.user_agent.clone());
        self.default_headers.insert(
            "Accept-Language".to_string(),
            profile.accept_language.clone(),
        );
        self.default_headers.insert(
            "Accept-Encoding".to_string(),
            "gzip, deflate, br".to_string(),
        );
        self.browser_profile = Some(profile);
        self
    }

    pub fn with_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.default_headers.insert(name.into(), value.into());
        self
    }

    pub fn with_referer(self, referer: impl Into<String>) -> Self {
        self.with_header("Referer", referer)
    }

    pub fn with_origin(self, origin: impl Into<String>) -> Self {
        self.with_header("Origin", origin)
    }

    pub fn with_desktop_user_agent(self) -> Self {
        let (name, value) = desktop_user_agent();
        self.with_header(name, value)
    }

    pub fn with_cookies_for(mut self, url: impl Into<String>) -> Self {
        self.cookie_url = Some(url.into());
        self
    }

    pub fn with_webview_challenge_fallback(mut self) -> Self {
        self.challenge_policy = ChallengePolicy::webview();
        self
    }

    pub fn get(&self, url: impl Into<String>) -> RequestBuilder<'_> {
        RequestBuilder::new(self, "GET", url)
    }

    pub fn post(&self, url: impl Into<String>) -> RequestBuilder<'_> {
        RequestBuilder::new(self, "POST", url)
    }

    pub fn get_text(&self, url: impl Into<String>) -> ExtensionResult<String> {
        self.get(url).send_text()
    }

    pub fn post_form_text(
        &self,
        url: impl Into<String>,
        form: &[(&str, &str)],
    ) -> ExtensionResult<String> {
        self.post(url).form(form).send_text()
    }

    pub fn post_json_text(
        &self,
        url: impl Into<String>,
        json: impl Into<String>,
    ) -> ExtensionResult<String> {
        self.post(url).json(json).send_text()
    }

    pub fn fetch_text(
        &self,
        method: impl Into<String>,
        url: impl Into<String>,
        body: Option<Vec<u8>>,
        headers: Headers,
    ) -> ExtensionResult<String> {
        Ok(self
            .fetch(method, url, body, headers)?
            .text
            .unwrap_or_default())
    }

    pub fn fetch(
        &self,
        method: impl Into<String>,
        url: impl Into<String>,
        body: Option<Vec<u8>>,
        headers: Headers,
    ) -> ExtensionResult<HttpResponse> {
        let builder = RequestBuilder::new(self, method, url).headers(headers);
        match body {
            Some(body) => builder.body(body).send(),
            None => builder.send(),
        }
    }

    fn browser_user_agent(&self) -> Option<String> {
        self.browser_profile
            .as_ref()
            .map(|profile| profile.user_agent.clone())
            .or_else(|| self.default_headers.get("User-Agent").cloned())
    }
}

#[derive(Clone, Debug)]
pub struct RequestBuilder<'a> {
    client: &'a HttpClient,
    method: String,
    url: String,
    headers: Headers,
    body: Option<Vec<u8>>,
    cookie_url: Option<String>,
    challenge_policy: Option<ChallengePolicy>,
}

impl<'a> RequestBuilder<'a> {
    fn new(client: &'a HttpClient, method: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            client,
            method: method.into(),
            url: url.into(),
            headers: Headers::new(),
            body: None,
            cookie_url: None,
            challenge_policy: None,
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

    pub fn referer(self, referer: impl Into<String>) -> Self {
        self.header("Referer", referer)
    }

    pub fn origin(self, origin: impl Into<String>) -> Self {
        self.header("Origin", origin)
    }

    pub fn browser_document(self) -> Self {
        self.header(
            "Accept",
            "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
        )
        .header("Upgrade-Insecure-Requests", "1")
        .header("Sec-Fetch-Dest", "document")
        .header("Sec-Fetch-Mode", "navigate")
        .header("Sec-Fetch-Site", "none")
    }

    pub fn xhr(self) -> Self {
        self.header("Accept", "application/json, text/javascript, */*; q=0.01")
            .header("X-Requested-With", "XMLHttpRequest")
            .header("Sec-Fetch-Dest", "empty")
            .header("Sec-Fetch-Mode", "cors")
            .header("Sec-Fetch-Site", "same-origin")
    }

    pub fn body(mut self, body: impl Into<Vec<u8>>) -> Self {
        self.body = Some(body.into());
        self
    }

    pub fn form(self, form: &[(&str, &str)]) -> Self {
        self.header("Content-Type", "application/x-www-form-urlencoded")
            .body(form_urlencoded(form).into_bytes())
    }

    pub fn json(self, json: impl Into<String>) -> Self {
        self.header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .body(json.into().into_bytes())
    }

    pub fn cookies_for(mut self, url: impl Into<String>) -> Self {
        self.cookie_url = Some(url.into());
        self
    }

    pub fn webview_challenge_fallback(mut self) -> Self {
        self.challenge_policy = Some(ChallengePolicy::webview());
        self
    }

    pub fn challenge_policy(mut self, policy: ChallengePolicy) -> Self {
        self.challenge_policy = Some(policy);
        self
    }

    pub fn send_text(self) -> ExtensionResult<String> {
        Ok(self.send()?.text.unwrap_or_default())
    }

    pub fn send(self) -> ExtensionResult<HttpResponse> {
        let policy = self
            .challenge_policy
            .clone()
            .unwrap_or_else(|| self.client.challenge_policy.clone());
        let request = self.build_request()?;
        let response = http_fetch(&request)?;
        if !is_challenge_response(&response) || matches!(policy, ChallengePolicy::Never) {
            return Ok(response);
        }
        self.resolve_challenge(policy, response)
    }

    fn build_request(&self) -> ExtensionResult<HttpRequest> {
        let mut headers = self.client.default_headers.clone();
        headers.extend(self.headers.clone());
        let cookie_url = self.cookie_url.as_ref().or(self.client.cookie_url.as_ref());
        if !headers.contains_key("Cookie") {
            let Some(cookie_url) = cookie_url else {
                return Ok(HttpRequest {
                    method: self.method.clone(),
                    url: self.url.clone(),
                    headers: headers.into_iter().collect(),
                    body_base64: self.body.as_deref().map(base64_encode),
                });
            };
            if let Some(cookie_header) = cookies_get(cookie_url)?.header {
                if !cookie_header.trim().is_empty() {
                    headers.insert("Cookie".to_string(), cookie_header);
                }
            }
        }
        Ok(HttpRequest {
            method: self.method.clone(),
            url: self.url.clone(),
            headers: headers.into_iter().collect(),
            body_base64: self.body.as_deref().map(base64_encode),
        })
    }

    fn resolve_challenge(
        &self,
        policy: ChallengePolicy,
        original: HttpResponse,
    ) -> ExtensionResult<HttpResponse> {
        let ChallengePolicy::WebView {
            wait_for,
            timeout_ms,
        } = policy
        else {
            return Ok(original);
        };

        let mut headers = self.client.default_headers.clone();
        headers.extend(self.headers.clone());
        headers.remove("Cookie");
        let webview = webview_open(&WebViewRequest {
            url: original.final_url.clone(),
            wait_for,
            wait_until: None,
            user_agent: self.client.browser_user_agent(),
            headers: headers.into_iter().collect(),
            timeout_ms,
            preload_scripts: Vec::new(),
            scripts: Vec::new(),
            return_html: true,
        })?;
        if !webview.cookies.is_empty() {
            cookies_set(webview.cookies)?;
        }

        let retry = http_fetch(&self.build_request()?)?;
        if !is_challenge_response(&retry) {
            return Ok(retry);
        }
        if self.method.eq_ignore_ascii_case("GET") {
            if let Some(html) = webview.html {
                return Ok(HttpResponse {
                    status: 200,
                    headers: Vec::new(),
                    final_url: webview.final_url,
                    body_base64: None,
                    text: Some(html),
                });
            }
        }
        Ok(retry)
    }
}

pub fn desktop_user_agent() -> (&'static str, &'static str) {
    ("User-Agent", DESKTOP_USER_AGENT)
}

pub fn browser_headers() -> Headers {
    HttpClient::browser().default_headers
}

pub fn form_urlencoded(form: &[(&str, &str)]) -> String {
    form.iter()
        .map(|(key, value)| format!("{}={}", url_encode(key), url_encode(value)))
        .collect::<Vec<_>>()
        .join("&")
}

pub fn origin_from_url(url: &str) -> Option<String> {
    let (scheme, rest) = url.split_once("://")?;
    let host = rest.split('/').next().filter(|host| !host.is_empty())?;
    Some(format!("{scheme}://{host}"))
}

pub fn is_challenge_response(response: &HttpResponse) -> bool {
    matches!(response.status, 403 | 429 | 503)
        || response
            .text
            .as_deref()
            .map(is_challenge_html)
            .unwrap_or(false)
}

pub fn is_challenge_html(html: &str) -> bool {
    let lower = html.to_ascii_lowercase();
    [
        "just a moment",
        "cf-browser-verification",
        "cf-challenge",
        "cloudflare-static",
        "challenge-platform",
        "ddos-guard",
        "ddos guard",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
}

pub fn url_encode(value: &str) -> String {
    let mut out = String::new();
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char)
            }
            b' ' => out.push('+'),
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}

pub fn base64_encode(bytes: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0];
        let b1 = *chunk.get(1).unwrap_or(&0);
        let b2 = *chunk.get(2).unwrap_or(&0);
        out.push(TABLE[(b0 >> 2) as usize] as char);
        out.push(TABLE[(((b0 & 0b0000_0011) << 4) | (b1 >> 4)) as usize] as char);
        if chunk.len() > 1 {
            out.push(TABLE[(((b1 & 0b0000_1111) << 2) | (b2 >> 6)) as usize] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(TABLE[(b2 & 0b0011_1111) as usize] as char);
        } else {
            out.push('=');
        }
    }
    out
}

const DESKTOP_USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_form_values() {
        assert_eq!(form_urlencoded(&[("q", "hello world")]), "q=hello+world");
        assert_eq!(form_urlencoded(&[("tag", "a&b")]), "tag=a%26b");
    }

    #[test]
    fn detects_challenge_html() {
        assert!(is_challenge_html("<title>Just a moment...</title>"));
        assert!(is_challenge_html(
            "<script src=\"/cdn-cgi/challenge-platform/x\"></script>"
        ));
        assert!(is_challenge_html("DDoS-Guard"));
        assert!(!is_challenge_html("<html><body>normal page</body></html>"));
    }

    #[test]
    fn extracts_origin() {
        assert_eq!(
            origin_from_url("https://example.com/path?q=1").as_deref(),
            Some("https://example.com")
        );
    }
}
