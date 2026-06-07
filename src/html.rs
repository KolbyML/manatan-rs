//! Small HTML string helpers for extension parsers.
//!
//! These helpers are intentionally dependency-light. They are useful for common
//! source-family parsing and smoke-test fixtures. Complex pages can still use a
//! dedicated parser crate from the extension.

pub fn strip_tags(html: impl AsRef<str>) -> String {
    let mut out = String::new();
    let mut in_tag = false;
    for ch in html.as_ref().chars() {
        match ch {
            '<' => in_tag = true,
            '>' => {
                in_tag = false;
                out.push(' ');
            }
            _ if !in_tag => out.push(ch),
            _ => {}
        }
    }
    html_unescape(&out)
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn html_unescape(value: &str) -> String {
    value
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ")
}

pub fn attr_after(haystack: &str, marker: &str, name: &str) -> Option<String> {
    let pos = haystack.find(marker)?;
    attr(&haystack[pos..], name)
}

pub fn attr(haystack: &str, attr: &str) -> Option<String> {
    for quote in ['"', '\''] {
        let needle = format!("{attr}={quote}");
        let start = haystack.find(&needle)? + needle.len();
        let rest = &haystack[start..];
        let end = rest.find(quote)?;
        return Some(html_unescape(&rest[..end]));
    }
    None
}

pub fn text_between(haystack: &str, start: &str, end: &str) -> Option<String> {
    let start_index = haystack.find(start)?;
    let after_start = &haystack[start_index..];
    let content_start = after_start
        .find('>')
        .map(|idx| idx + 1)
        .unwrap_or(start.len());
    let rest = &after_start[content_start..];
    let end_index = rest.find(end)?;
    Some(rest[..end_index].to_string())
}

pub fn text_before(haystack: &str, end: &str) -> Option<String> {
    let close = haystack.find('>')?;
    let rest = &haystack[close + 1..];
    let end_index = rest.find(end)?;
    Some(rest[..end_index].to_string())
}

pub fn id_block(html: &str, id: &str) -> Option<String> {
    let marker = format!("id=\"{id}\"");
    let id_pos = html
        .find(&marker)
        .or_else(|| html.find(&format!("id='{id}'")))?;
    balanced_block(html, id_pos)
}

pub fn class_block(html: &str, class_name: &str) -> Option<String> {
    let class_pos = html
        .find(&format!("class=\"{class_name}"))
        .or_else(|| html.find(&format!("class='{class_name}")))
        .or_else(|| html.find(class_name))?;
    balanced_block(html, class_pos)
}

pub fn balanced_block(html: &str, marker_pos: usize) -> Option<String> {
    let open_start = html[..marker_pos].rfind('<')?;
    let open_end = html[open_start..].find('>')? + open_start;
    let tag_name = html[open_start + 1..]
        .split_whitespace()
        .next()
        .unwrap_or("div");
    let close = format!("</{tag_name}>");
    let rest = &html[open_end + 1..];
    let close_pos = rest.find(&close)?;
    Some(rest[..close_pos].to_string())
}
