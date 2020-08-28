use clap::{arg, command};
use regex::Regex;
use scraper::{Html, Selector};

// static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub struct Etymology {
    pub word: String,
    pub label: String,
    pub etymology: String,
}

impl Etymology {
    pub fn new(word: &str) -> Etymology {
        let results_html = query_etym_online(word);
        let etymology_html = Etymology::extract_etymology_html(&results_html);
        let etymology = Etymology::beautify(&etymology_html);
        let label = Etymology::extract_word_name(&results_html);
        Etymology {
            word: word.to_owned(),
            label,
            etymology,
        }
    }

    /// Substitute HTML formatting for italics with terminal escape codes.
    /// Does NOT intelligently determine whether terminal is interactive.
    pub fn beautify(etym_html: &str) -> String {
        let re_italics =
            Regex::new(r#"<span class="\w+ notranslate">(?P<word>[^<]+)</span>"#).unwrap();
        // Use manual terminal escape codes for italics
        let e: String = re_italics
            .replace_all(etym_html, "\x1b[0;3m${word}\x1b[23m")
            .to_string();
        let html = Html::parse_fragment(&e);
        // Search for container "div" which was added in `extract_etymology_html` fn.
        let sel = Selector::parse("div").unwrap();
        html.select(&sel).next().unwrap().text().collect::<String>()
    }

    /// From raw HTML results of query, excise just the first definition found.
    pub fn extract_etymology_html(raw_html: &str) -> String {
        // TODO: Should return a Result, in case no hits were found.
        let d = Html::parse_document(raw_html);
        let section_selector = Selector::parse("section").unwrap();
        for x in d.select(&section_selector) {
            if let Some(y) = x.value().attr("class") {
                if y.starts_with("word__def") {
                    // Pad with custom div, so we can easily retrieve the entirety again
                    // in `beautify`.
                    let etym_html = format!("<div>{}</div>", x.inner_html());
                    return etym_html;
                }
            }
        }
        raw_html.to_string()
    }

    /// Extract the entry name, e.g. `Viking (n.)`
    pub fn extract_word_name(raw_html: &str) -> String {
        // TODO: Should return a Result, in case no hits were found.
        let d = Html::parse_document(raw_html);
        let section_selector = Selector::parse("a").unwrap();
        for x in d.select(&section_selector) {
            if let Some(y) = x.value().attr("class") {
                if y.starts_with("word__name") {
                    // Pad with custom div, so we can easily retrieve the entirety again
                    // in `beautify`.
                    let word_name = x.text().collect::<String>();
                    return word_name;
                }
            }
        }
        raw_html.to_string()
    }
}

/// Perform HTTP GET to query EtymOnline.com.
/// Requires a search term. Currently NOT URL-encoded.
/// Returns raw HTML results.
fn query_etym_online(word: &str) -> String {
    // TODO: we should urlescape the word, in case it has spaces
    let url = format!("https://www.etymonline.com/search?q={}", word);
    // Fetch HTML
    match ureq::get(&url).call() {
        Ok(response) => response.into_string().unwrap(),
        Err(ureq::Error::Status(code, _response)) => {
            panic!("Encountered HTTP error {} when looking up {}", code, word);
        }
        Err(_) => {
            panic!("Failed to look up {}", word);
        }
    }
}

fn main() {
    let matches = command!()
        .arg(arg!([word] "Word for which etymology is desired"))
        .arg(arg!(
            -d --debug ... "Turn debugging information on"
        ))
        .get_matches();

    if let Some(word) = matches.value_of("word") {
        let e = Etymology::new(word);
        // Manually bold the search term
        println!("\x1b[1m{}\x1b[22m", e.label);
        let w = textwrap::termwidth();
        let s = textwrap::fill(&e.etymology, w);
        println!("{}", s);
    }
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
        let e = Etymology::new("viking");
        assert!(e.word == "viking");
        assert!(!e.etymology.contains("<span class=\"foreign notranslate\">"));
    }
}
