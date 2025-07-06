use reqwest::{Client, header::HeaderMap};
use cookie_store::CookieStore;
use reqwest_cookie_store::CookieStoreMutex;
use std::fs::{File, create_dir_all};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::sync::Arc;
use anyhow::{Context, Result};
use cookie_store::serde;
use reqwest::header::{HeaderValue, ACCEPT, USER_AGENT, ACCEPT_ENCODING, CONNECTION, CONTENT_TYPE};

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

        let mut default_headers = HeaderMap::new();
        default_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded")); // To jest ważne dla POST
        default_headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate, br"));
        default_headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
        default_headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36 Edg/111.0.1661.51"));
        default_headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"));

        let client = Client::builder()
            .cookie_provider(jar.clone())
            .redirect(reqwest::redirect::Policy::default())
            .default_headers(default_headers)
            .build()?;

        Ok(Self {
            client,
            jar,
            cookie_path,
        })
    }

    pub async fn get_html(&self, url: &str) -> Result<String> {
        let response = self.client.get(url).send().await?;
        let body = response.text().await?;
        save_cookies(&self.jar.lock().unwrap(), &self.cookie_path)?;
        Ok(body)
    }

    pub async fn post_form(
        &self,
        url: &str,
        form: &[(String, String)],
        headers: Option<HeaderMap>,
    ) -> Result<String> {
        let mut req = self.client.post(url).form(form);

        if let Some(h) = headers {
            req = req.headers(h);
        }

        let response = req.send().await?;
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
