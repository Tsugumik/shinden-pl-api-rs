use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;
use std::sync::Arc;
use reqwest::Client;
use reqwest_cookie_store::CookieStoreMutex;

use crate::headers::{get_headers_for_type, RequestType};
use crate::error::ShindenError;
use crate::utils;

#[derive(Clone)]
pub struct ShindenHttpClient {
    pub http: Client,
    pub jar: Arc<CookieStoreMutex>,
    pub cookie_path: Option<PathBuf>
}

impl ShindenHttpClient {
    pub fn new_default() -> Result<Self, ShindenError> {
        let path = utils::resolve_default_path()?;
        Self::create(Some(path))
    }

    pub fn new_memory() -> Result<Self, ShindenError> {
        Self::create(None)
    }

    pub fn new_custom(path: Option<PathBuf>) -> Result<Self, ShindenError> {
        Self::create(path)
    }

    fn create(path: Option<PathBuf>) -> Result<Self, ShindenError> {
        let store = if let Some(p) = &path {
            utils::load_cookies_from_disk(p)?
        } else {
            cookie_store::CookieStore::default()
        };

        let jar = Arc::new(CookieStoreMutex::new(store));

        let http = Client::builder()
            .cookie_provider(jar.clone())
            .redirect(reqwest::redirect::Policy::default())
            .gzip(true)
            .deflate(true)
            .brotli(true)
            .build()
            .map_err(ShindenError::Network)?;

        Ok(Self {
            http,
            jar,
            cookie_path: path
        })
    }

    pub fn save_cookies(&self) -> Result<(), ShindenError> {
        let path = self.cookie_path.as_ref()
            .ok_or_else(|| ShindenError::Config("Client is configured for in-memory mode".to_string()))?;

        let store = self.jar.lock().map_err(
            |_| ShindenError::LockPoisoned
        )?;

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

    pub async fn get_html(
        &self, url: &str, request_type: RequestType
    ) -> Result<String, ShindenError> {
        let headers = get_headers_for_type(request_type, Some(url))
            .map_err(|e| ShindenError::HeaderConfig(e.to_string()))?;

        let response = self.http.get(url)
            .headers(headers)
            .send()
            .await?
            .error_for_status()?;

        let body = response.text().await?;

        Ok(body)
    }

    pub async fn post_form(
        &self,
        url: &str,
        form: &[(String, String)],
        request_type: RequestType
    ) -> Result<String, ShindenError> {
        let headers = get_headers_for_type(request_type, Some(url))
            .map_err(|e| ShindenError::HeaderConfig(e.to_string()))?;

        let response = self.http.post(url)
            .form(form)
            .headers(headers)
            .send()
            .await?
            .error_for_status()?;

        let body = response.text().await?;

        Ok(body)
    }
}
