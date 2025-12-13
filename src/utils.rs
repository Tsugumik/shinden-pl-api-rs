use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use scraper::{ElementRef, Selector};
use scraper::node::Text;
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
