//! A library and CLI utility for querying word etymologies
//! from the inimitable [EtymOnline.com](https://etymonline.com).

use anyhow::Result;
use regex::Regex;
use scraper::{Html, Selector};

/// An etymology as retrieved from EtymOnline.com.
pub struct Etymology {
    /// The search term used for looking up an entry.
    pub word: String,
    /// Container for the word, along with its part of speech, from the entry.
    pub label: String,
    /// The entry as retrieved from EtymOnline.com. Currently this content
    /// is formatted for terminal output, including bold and italics.
    // TODO: add `etymology_html` field to make formatting optional.
    pub etymology: String,
}

impl Etymology {
    /// Performs a lookup via EtymOnline.com for the given word.
    /// Fallible, as it can fail via network error, or simply
    /// not find an entry. Error types are non-specific for simplicity's sake.
    pub fn new(word: &str) -> Result<Self> {
        let results_html = query_etym_online(word)?;
        let etymology_html = Etymology::extract_etymology_html(&results_html)?;
        let etymology = Etymology::beautify(&etymology_html)?;
        let label = Etymology::extract_word_name(&results_html)?;
        Ok(Etymology {
            word: word.to_owned(),
            label,
            etymology,
        })
    }

    /// Substitute HTML formatting for italics with terminal escape codes.
    /// Does NOT intelligently determine whether terminal is interactive.
    pub fn beautify(etym_html: &str) -> Result<String> {
        let re_italics = Regex::new(r#"<span class="\w+ notranslate">(?P<word>[^<]+)</span>"#)?;
        // Use manual terminal escape codes for italics
        let e: String = re_italics
            .replace_all(etym_html, "\x1b[0;3m${word}\x1b[23m")
            .to_string();
        let html = Html::parse_fragment(&e);
        // Search for container "div" which was added in `extract_etymology_html` fn.
        let sel = Selector::parse("div")
            .map_err(|_| anyhow::anyhow!("Failed to find HTML div for beautification"))?;
        Ok(html.select(&sel).next().unwrap().text().collect::<String>())
    }

    /// From the flight payload of a query, excise just the first definition found.
    ///
    /// The payload is a React Server Component ("flight") stream. Word entries
    /// appear as plain JSON, e.g. `"word":{...,"etymology":"...","thumbnail":...}`.
    /// The etymology value is either inline HTML (a JSON string, so quotes are
    /// `\"`-escaped) or a lazy reference like `"$26"`, pointing to a text chunk
    /// declared elsewhere in the stream as `<id>:T<hex length>,<raw html>`.
    pub fn extract_etymology_html(payload: &str) -> Result<String> {
        // Capture a JSON string value, honoring backslash escapes.
        let re = Regex::new(r#""etymology":"((?:[^"\\]|\\.)*)","thumbnail""#)?;
        let caps = re
            .captures(payload)
            .ok_or_else(|| anyhow::anyhow!("Failed to find etymology within payload"))?;
        let value = caps.get(1).unwrap().as_str();
        let html = match value.strip_prefix('$') {
            Some(id) => resolve_flight_text(payload, id)?,
            None => json_unescape(value),
        };
        // Pad with custom div, so we can easily retrieve the entirety again
        // in `beautify`.
        Ok(format!("<div>{html}</div>"))
    }

    /// Extract the entry name, e.g. `Viking (n.)`
    pub fn extract_word_name(payload: &str) -> Result<String> {
        let re = Regex::new(
            r#""word":"((?:[^"\\]|\\.)*)","canonical_word":"(?:[^"\\]|\.)*","type":\d+,"property":"((?:[^"\\]|\\.)*)""#,
        )?;
        let caps = re
            .captures(payload)
            .ok_or_else(|| anyhow::anyhow!("Failed to find word name within payload"))?;
        let word = json_unescape(caps.get(1).unwrap().as_str());
        let property = json_unescape(caps.get(2).unwrap().as_str());
        if property.is_empty() {
            Ok(word)
        } else {
            Ok(format!("{word} {property}"))
        }
    }
}

/// Resolve a flight-stream text chunk reference, e.g. `$26` names the chunk
/// declared as `26:T<hex>,...` where `<hex>` is the byte length of the raw
/// (unescaped) text following the comma.
fn resolve_flight_text(payload: &str, id: &str) -> Result<String> {
    let re = Regex::new(&format!(r"(?m)^{}:T([0-9a-f]+),", regex::escape(id)))?;
    let caps = re
        .captures(payload)
        .ok_or_else(|| anyhow::anyhow!("Failed to resolve flight text chunk '{id}'"))?;
    let len = usize::from_str_radix(caps.get(1).unwrap().as_str(), 16)?;
    let start = caps.get(0).unwrap().end();
    let mut end = (start + len).min(payload.len());
    while end > start && !payload.is_char_boundary(end) {
        end -= 1;
    }
    Ok(payload[start..end].to_owned())
}

/// Decode the escapes permitted in a JSON string: `\"`, `\\`, `\n`, `\r`,
/// `\t`, and `\uXXXX` (which Next.js uses for `<`, `>`, and `&`).
fn json_unescape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('n') => out.push('\n'),
            Some('r') => out.push('\r'),
            Some('t') => out.push('\t'),
            Some('u') => {
                let hex: String = chars.by_ref().take(4).collect();
                match u32::from_str_radix(&hex, 16).ok().and_then(char::from_u32) {
                    Some(c) => out.push(c),
                    None => {
                        out.push_str("\\u");
                        out.push_str(&hex);
                    }
                }
            }
            Some(c) => out.push(c),
            None => out.push('\\'),
        }
    }
    out
}

/// Perform HTTP GET to query EtymOnline.com.
/// Requires a search term. Currently NOT URL-encoded.
///
/// Since late 2025, EtymOnline fronts its HTML pages with a Cloudflare
/// JavaScript challenge that defeats any non-browser client. The site is a
/// Next.js app, though, and its React Server Component endpoint answers plainly
/// when the request carries the `RSC: 1` header: instead of HTML, we receive a
/// "flight" payload containing the fully server-rendered entry data as JSON.
///
/// Returns the raw flight payload.
fn query_etym_online(word: &str) -> Result<String> {
    // TODO: we should urlescape the word, in case it has spaces
    let url = format!("https://www.etymonline.com/search?q={word}");
    ureq::get(&url)
        .set("RSC", "1")
        .call()?
        .into_string()
        .map_err(|_| anyhow::anyhow!("Failed to query EtymOnline; network error?"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_flight_payload() {
        let payload = include_str!("../tests/fixture-viking.rsc");
        // The flight payload is a React Server Component stream carrying the
        // server-rendered search results as JSON.
        assert!(payload.contains("entries found"));
        assert!(payload.contains(r#""word":"Viking""#));
        // Lazy text chunks are declared as `<id>:T<hex length>,`
        assert!(Regex::new(r"(?m)^[0-9a-f]+:T[0-9a-f]+,").unwrap().is_match(payload));
    }

    #[test]
    fn html_markup_removed_from_etym() {
        let payload = include_str!("../tests/fixture-viking.rsc");

        // Test extraction of etymology HTML (referenced via $id chunk)
        let etym_html = Etymology::extract_etymology_html(&payload).unwrap();
        assert!(etym_html.contains("Scandinavian pirate"));
        assert!(etym_html.contains("vikingr"));

        // Test extraction and beautification
        let label = Etymology::extract_word_name(&payload).unwrap();
        assert_eq!(label, "Viking (n.)");

        let etymology = Etymology::beautify(&etym_html).unwrap();
        assert!(etymology.contains("Scandinavian pirate"));
        assert!(etymology.contains("vikingr"));
        // HTML markup should be removed
        assert!(!etymology.contains("<span class=\"foreign notranslate\">"));
        assert!(!etymology.contains("<p>"));
    }

    #[test]
    fn scrimshaw_inline_etymology() {
        let payload = include_str!("../tests/fixture-scrimshaw.rsc");

        // Test extraction of etymology HTML (inline, not $ref)
        let etym_html = Etymology::extract_etymology_html(&payload).unwrap();
        assert!(etym_html.contains("shell or piece of ivory"));
        assert!(etym_html.contains("scrimshon"));

        // Test extraction and beautification
        let label = Etymology::extract_word_name(&payload).unwrap();
        assert_eq!(label, "scrimshaw (n.)");

        let etymology = Etymology::beautify(&etym_html).unwrap();
        assert!(etymology.contains("shell or piece of ivory"));
        assert!(etymology.contains("scrimshon"));
        // Should NOT contain cruft from the payload
        assert!(!etymology.contains("localStorage"));
        assert!(!etymology.contains("Log in"));
        assert!(!etymology.contains("Remove Ads"));
        // HTML markup should be removed
        assert!(!etymology.contains("<span class=\"foreign notranslate\">"));
        assert!(!etymology.contains("<p>"));
    }
}
