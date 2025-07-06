use crate::headers::{get_headers_for_type, RequestType};
use anyhow::{Context, Result};
use cookie_store::serde;
use cookie_store::CookieStore;
use reqwest::Client;
use reqwest_cookie_store::CookieStoreMutex;
use std::fs::{create_dir_all, File};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Clone)]

pub struct ShindenAPI {
    pub client: Client,
    jar: Arc<CookieStoreMutex>,
    cookie_path: PathBuf,
}

impl ShindenAPI {
    pub fn new() -> Result<Self> {
        let mut cookie_path =
            dirs::data_local_dir().context("Could not find user local data directory")?;
        cookie_path.push("shinden_api");
        create_dir_all(&cookie_path)?;
        cookie_path.push("cookies.json");

        let store = load_cookies(&cookie_path)?;
        let jar = Arc::new(CookieStoreMutex::new(store));

        let client = Client::builder()
            .cookie_provider(jar.clone())
            .redirect(reqwest::redirect::Policy::default())
            .gzip(true)
            .deflate(true)
            .brotli(true)
            .build()?;

        Ok(Self {
            client,
            jar,
            cookie_path,
        })
    }

    pub async fn get_html(&self, url: &str) -> Result<String> {
        let headers = get_headers_for_type(RequestType::Frontend, Some(url))?;
        let response = self.client.get(url)
            .headers(headers)
            .send()
            .await?
            .error_for_status()?;

        let body = response.text().await?;

        save_cookies(&self.jar.lock().unwrap(), &self.cookie_path)?;
        Ok(body)
    }

    pub async fn post_form(&self, url: &str, form: &[(String, String)], custom_headers: Option<reqwest::header::HeaderMap>) -> Result<String> {
        let mut headers = get_headers_for_type(RequestType::Login, Some(url))?;

        if let Some(ch) = custom_headers {
            headers.extend(ch);
        }

        let response = self.client.post(url)
            .form(form)
            .headers(headers)
            .send()
            .await?
            .error_for_status()?;

        let body = response.text().await?;
        save_cookies(&self.jar.lock().unwrap(), &self.cookie_path)?;
        Ok(body)
    }
}

fn load_cookies(path: &std::path::Path) -> Result<CookieStore> {
    if path.exists() {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        serde::json::load(reader).map_err(|e| anyhow::anyhow!(e.to_string()))
    } else {
        Ok(CookieStore::default())
    }
}

fn save_cookies(store: &CookieStore, path: &std::path::Path) -> Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    serde::json::save(store, &mut writer).map_err(|e| anyhow::anyhow!(e.to_string()))?;

    Ok(())
}
