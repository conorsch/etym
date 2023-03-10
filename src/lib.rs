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
        let d = Html::parse_document(raw_html);
        let section_selector = Selector::parse("a")
            .map_err(|_| anyhow::anyhow!("Failed to parse 'a' element for word entry"))?;
        for x in d.select(&section_selector) {
            if let Some(y) = x.value().attr("class") {
                if y.starts_with("word__name") {
                    // Pad with custom div, so we can easily retrieve the entirety again
                    // in `beautify`.
                    let word_name = x.text().collect::<String>();
                    return Ok(word_name);
                }
            }
        }
        anyhow::bail!("Failed to find word name within HTML")
    }
}

/// Perform HTTP GET to query EtymOnline.com.
/// Requires a search term. Currently NOT URL-encoded.
/// Returns raw HTML results.
fn query_etym_online(word: &str) -> Result<String> {
    // TODO: we should urlescape the word, in case it has spaces
    let url = format!("https://www.etymonline.com/search?q={}", word);
    // Fetch HTML
    ureq::get(&url)
        .call()?
        .into_string()
        .map_err(|_| anyhow::anyhow!(format!("Failed to query EtymOnline; network error?")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_import_fixture() {
        let raw_html = include_str!("../tests/fixture-viking.html");
        assert!(raw_html != "foo");
    }

    #[test]
    fn can_parse_html() {
        let raw_html = include_str!("../tests/fixture-viking.html");
        let document = Html::parse_document(raw_html);
        let selector = Selector::parse("div#root").unwrap();
        let root_div = document.select(&selector).next().unwrap();
        assert!(root_div.value().name() == "div");
        assert!(root_div.value().id() == Some("root"));
        let root_text = root_div.text().collect::<String>();
        assert!(root_text.starts_with("Adver"));
    }

    #[test]
    fn html_markup_removed_from_etym() {
        let raw_html = include_str!("../tests/fixture-viking.html");
        let _h = Etymology::extract_etymology_html(&raw_html);
        let e = Etymology::new("viking").unwrap();
        assert!(e.word == "viking");
        assert!(!e.etymology.contains("<span class=\"foreign notranslate\">"));
    }
}
