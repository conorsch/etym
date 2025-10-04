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

    /// From raw HTML results of query, excise just the first definition found.
    pub fn extract_etymology_html(raw_html: &str) -> Result<String> {
        // Try new format first (JSON in JavaScript)
        // The structure has word entries with: "word":"...", "property":"...", "etymology":"..."
        // We need to find a complete word entry (not other JSON in the page)
        // The etymology field contains escaped HTML and ends with ","thumbnail"
        // We match everything up to the closing quote+comma: \","
        let re_word_entry = Regex::new(
            r#"\\"word\\":\\"[^\\]+\\",\\"canonical_word\\":\\"[^\\]+\\",\\"type\\":\d+,\\"property\\":\\"[^\\]*\\",\\"etymology\\":\\"(.*?)\\",\\"thumbnail\\""#
        )?;

        if let Some(caps) = re_word_entry.captures(raw_html) {
            let etym_value = caps.get(1).unwrap().as_str();

            // If it's a reference like "$2e", we need to find the referenced content
            if etym_value.starts_with('$') {
                let ref_id = &etym_value[1..]; // Remove the $
                // The pattern is: one push has "2e:Taf2," and the NEXT push has the content
                // First, find the push that declares the reference
                let ref_pattern = format!(r#"{}:[^,]+,"#, regex::escape(ref_id));
                let re_ref = Regex::new(&ref_pattern)?;

                if let Some(ref_match) = re_ref.find(raw_html) {
                    // Now find the next self.__next_f.push after this position
                    let after_ref = &raw_html[ref_match.end()..];
                    let re_next_push = Regex::new(r#"self\.__next_f\.push\(\[1,"([^"]*(?:\\.[^"]*)*)"\]\)"#)?;

                    if let Some(next_caps) = re_next_push.captures(after_ref) {
                        let etym_html = next_caps.get(1).unwrap().as_str();
                        // The HTML is escaped with \u003c for < and \u003e for >
                        let decoded = etym_html
                            .replace(r"\u003c", "<")
                            .replace(r"\u003e", ">")
                            .replace(r"\n", "\n")
                            .replace(r#"\""#, "\"");
                        return Ok(format!("<div>{}</div>", decoded));
                    }
                }
            } else if etym_value.contains(r"\u003c") || etym_value.contains(r"u003c") {
                // It's inline escaped HTML
                let decoded = etym_value
                    .replace(r"\u003c", "<")
                    .replace(r"\u003e", ">")
                    .replace(r"u003c", "<")
                    .replace(r"u003e", ">")
                    .replace(r"\n", "\n")
                    .replace(r#"\\""#, "\"")  // \" in the JSON string
                    .replace(r#"\""#, "\"");   // Also handle \" directly
                return Ok(format!("<div>{}</div>", decoded));
            }
        }

        // Fallback to old format
        let d = Html::parse_document(raw_html);
        let section_selector = Selector::parse("section")
            .map_err(|_| anyhow::anyhow!("Failed to parse HTML section for entry"))?;
        for x in d.select(&section_selector) {
            if let Some(y) = x.value().attr("class") {
                if y.starts_with("word__def") {
                    // Pad with custom div, so we can easily retrieve the entirety again
                    // in `beautify`.
                    let etym_html = format!("<div>{}</div>", x.inner_html());
                    return Ok(etym_html);
                }
            }
        }
        Ok(raw_html.to_string())
    }

    /// Extract the entry name, e.g. `Viking (n.)`
    pub fn extract_word_name(raw_html: &str) -> Result<String> {
        // Try new format first (JSON in JavaScript)
        // In the HTML, quotes are escaped as \"
        let re_word = Regex::new(r#"\\"word\\":\\"([^\\]+)\\",\\"canonical_word\\":\\"[^\\]+\\",\\"type\\":\d+,\\"property\\":\\"([^\\]*)\\""#)?;
        if let Some(caps) = re_word.captures(raw_html) {
            let word = caps.get(1).unwrap().as_str();
            let property = caps.get(2).unwrap().as_str();
            if property.is_empty() {
                return Ok(word.to_string());
            } else {
                return Ok(format!("{} {}", word, property));
            }
        }

        // Fallback to old format
        let d = Html::parse_document(raw_html);
        let section_selector = Selector::parse("span[id^='etymonline_v_']")
            .map_err(|_| anyhow::anyhow!("Failed to parse 'span' element for word entry"))?;
        if let Some(x) = d.select(&section_selector).next() {
            let word_name = x.text().collect::<String>();
            return Ok(word_name);
        }
        anyhow::bail!("Failed to find word name within HTML")
    }
}

/// Perform HTTP GET to query EtymOnline.com.
/// Requires a search term. Currently NOT URL-encoded.
/// Returns raw HTML results.
fn query_etym_online(word: &str) -> Result<String> {
    // TODO: we should urlescape the word, in case it has spaces
    let url = format!("https://www.etymonline.com/search?q={word}");
    // Fetch HTML
    ureq::get(&url)
        .call()?
        .into_string()
        .map_err(|_| anyhow::anyhow!("Failed to query EtymOnline; network error?"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_html() {
        let raw_html = include_str!("../tests/fixture-viking.html");
        // Test that we can parse the new HTML structure
        // The new format is server-side rendered with JSON data in script tags
        assert!(raw_html.contains("entries found"));
        // Quotes are escaped in the script tags
        assert!(raw_html.contains(r#"\"word\":\"Viking\""#));

        // Also verify we can parse it as HTML document
        let document = Html::parse_document(raw_html);
        let selector = Selector::parse("body").unwrap();
        let body = document.select(&selector).next().unwrap();
        assert!(body.value().name() == "body");
    }

    #[test]
    fn html_markup_removed_from_etym() {
        let raw_html = include_str!("../tests/fixture-viking.html");

        // Test extraction of etymology HTML
        let etym_html = Etymology::extract_etymology_html(&raw_html).unwrap();
        assert!(etym_html.contains("Scandinavian pirate"));
        assert!(etym_html.contains("vikingr"));

        // Test extraction and beautification
        let label = Etymology::extract_word_name(&raw_html).unwrap();
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
        let raw_html = include_str!("../tests/fixture-scrimshaw.html");

        // Test extraction of etymology HTML (inline, not $ref)
        let etym_html = Etymology::extract_etymology_html(&raw_html).unwrap();
        assert!(etym_html.contains("shell or piece of ivory"));
        assert!(etym_html.contains("scrimshon"));

        // Test extraction and beautification
        let label = Etymology::extract_word_name(&raw_html).unwrap();
        assert_eq!(label, "scrimshaw (n.)");

        let etymology = Etymology::beautify(&etym_html).unwrap();
        assert!(etymology.contains("shell or piece of ivory"));
        assert!(etymology.contains("scrimshon"));
        // Should NOT contain cruft from the page
        assert!(!etymology.contains("localStorage"));
        assert!(!etymology.contains("Log in"));
        assert!(!etymology.contains("Remove Ads"));
        // HTML markup should be removed
        assert!(!etymology.contains("<span class=\"foreign notranslate\">"));
        assert!(!etymology.contains("<p>"));
    }
}
