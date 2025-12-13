use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use reqwest::Url;
use scraper::{ElementRef, Selector};
use crate::error::ShindenError;

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

pub fn load_cookies_from_disk(path: &Path) -> Result<cookie_store::CookieStore, ShindenError> {
    if !path.exists() {
        return Ok(cookie_store::CookieStore::default())
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    cookie_store::serde::json::load(reader)
        .map_err(|e| ShindenError::CookieParse(e.to_string()))
}

pub fn get_text_from_selector(parent: &ElementRef, sel: &Selector) -> String {
    parent.select(sel)
        .next()
        .map(|el| el.text().collect::<String>().trim().to_string())
        .unwrap_or_default()
}

pub fn get_attr_from_selector(parent: &ElementRef, sel: &Selector, attr: &str) -> String {
    match parent.select(sel).next() {
        Some(el) => el.value().attr(attr).unwrap_or("").to_string(),
        None => String::new(),
    }
}

pub fn parse_f64_from_comma_str(text: &str) -> Option<f64> {
    if text.is_empty() || text == "-" {
        return None;
    }

    text.replace(",", ".").parse::<f64>().ok()
}

pub fn parse_u32_from_str(text: &str) -> Option<u32> {
    if text.trim() == "?" || text.is_empty() {
        return None;
    }

    let clean: String = text.chars().filter(|c| c.is_ascii_digit()).collect();
    clean.parse::<u32>().ok()
}

pub fn extract_f64_after_colon(parent: &ElementRef, sel: &Selector) -> f64 {
    let text = get_text_from_selector(parent, sel);

    if let Some(idx) = text.find(':') {
        let val_part = &text[idx+1..].trim();
        return parse_f64_from_comma_str(val_part).unwrap_or(0.0);
    }

    parse_f64_from_comma_str(&text).unwrap_or(0.0)
}

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
