use std::fs;
use reqwest_cookie_store::CookieStoreMutex;
use std::path::{Path, PathBuf};
use std::fs::{File};
use std::io::{BufReader, BufWriter};
use cookie_store::CookieStore;
use crate::error::ShindenError;


fn resolve_default_path() -> Result<PathBuf, ShindenError> {
    let mut path = dirs::data_local_dir()
        .ok_or_else(|| ShindenError::Config("Cannot determine local data directory".into()))?;

    path.push("shinden_api");

    if !path.exists() {
        fs::create_dir_all(&path)?;
    }

    path.push("cookie.json");

    Ok(path)
}

fn load_store_from_disk(path: &Path) -> Result<CookieStore, ShindenError> {
    if !path.exists() {
        return Ok(CookieStore::default());
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    cookie_store::serde::json::load(reader)
        .map_err(|e| ShindenError::CookieParse(e.to_string()))
}

fn save_store_to_disk(jar: &CookieStoreMutex, path: &Path) -> Result<(), ShindenError> {
    let store = jar.lock().map_err(|_| ShindenError::LockPoisoned)?;

    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    cookie_store::serde::json::save(&*store, &mut writer)
        .map_err(|e| ShindenError::CookieParse(e.to_string()))?;

    Ok(())
}