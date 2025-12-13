use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use reqwest::Url;
use scraper::{ElementRef, Selector};
use crate::error::ShindenError;

/// Resolves the default path for storing API data (e.g., cookies).
/// Usually points to `~/.local/share/shinden_api/cookies.json` on Linux.
pub fn resolve_default_path() -> Result<PathBuf, ShindenError> {
    let mut path = dirs::data_local_dir()
        .ok_or_else(|| ShindenError::Config("Could not determine local data directory".to_string()))?;

    path.push("shinden_api");

    if !path.exists() {
        fs::create_dir_all(&path)?;
    }

    path.push("cookies.json");
    Ok(path)
}

/// Loads the cookie store from a JSON file at the given path.
/// Returns an empty store if the file does not exist.
pub fn load_cookies_from_disk(path: &Path) -> Result<cookie_store::CookieStore, ShindenError> {
    if !path.exists() {
        return Ok(cookie_store::CookieStore::default())
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    cookie_store::serde::json::load(reader)
        .map_err(|e| ShindenError::CookieParse(e.to_string()))
}

/// Extracts inner text from the first element matching the selector within a parent element.
/// Returns an empty string if the element is not found.
pub fn get_text_from_selector(parent: &ElementRef, sel: &Selector) -> String {
    parent.select(sel)
        .next()
        .map(|el| el.text().collect::<String>().trim().to_string())
        .unwrap_or_default()
}

/// Extracts the value of a specific attribute (e.g., "href", "src") from the first matching element.
/// Returns an empty string if the element or attribute is not found.
pub fn get_attr_from_selector(parent: &ElementRef, sel: &Selector, attr: &str) -> String {
    match parent.select(sel).next() {
        Some(el) => el.value().attr(attr).unwrap_or("").to_string(),
        None => String::new(),
    }
}

/// Parses a string containing a comma-separated decimal (e.g., "7,42") into a f64.
/// Returns `None` if the string is empty or invalid.
pub fn parse_f64_from_comma_str(text: &str) -> Option<f64> {
    if text.is_empty() || text == "-" {
        return None;
    }

    text.replace(",", ".").parse::<f64>().ok()
}

/// Parses a string containing digits (e.g., "148" or "12 episodes") into a u32.
/// Filters out non-digit characters before parsing.
pub fn parse_u32_from_str(text: &str) -> Option<u32> {
    if text.trim() == "?" || text.is_empty() {
        return None;
    }

    let clean: String = text.chars().filter(|c| c.is_ascii_digit()).collect();
    clean.parse::<u32>().ok()
}

/// Extracts text using a selector, looks for a color (':'), and parses the number following it.
/// Useful for strings like "Overall: 7,42".
/// Falls back to parsing the whole string if no colon is found.
pub fn extract_f64_after_colon(parent: &ElementRef, sel: &Selector) -> f64 {
    let text = get_text_from_selector(parent, sel);

    if let Some(idx) = text.find(':') {
        let val_part = &text[idx+1..].trim();
        return parse_f64_from_comma_str(val_part).unwrap_or(0.0);
    }

    parse_f64_from_comma_str(&text).unwrap_or(0.0)
}

/// Extracts a query parameter (e.g., "page") from a relative URL string.
/// Handles relative paths by prepending a dummy host.
pub fn get_u32_param_from_url(href: &str, param_name: &str) -> Option<u32> {
    if href.is_empty() {
        return None;
    }

    let fake_url = match Url::parse(&format!("https://dummy{}", href)) {
        Ok(u) => u,
        Err(_) => return None,
    };

    match fake_url.query_pairs().find(|(k, _)| k == param_name) {
        Some((_, v)) => v.parse::<u32>().ok(),
        None => None,
    }
}
