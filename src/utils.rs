use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
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
