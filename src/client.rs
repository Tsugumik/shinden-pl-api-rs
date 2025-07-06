use reqwest::{Client, header::HeaderMap};
use cookie_store::CookieStore;
use reqwest_cookie_store::CookieStoreMutex;
use std::fs::{File, create_dir_all};
use std::io::{BufReader};
use std::path::PathBuf;
use std::sync::Arc;
use anyhow::{Result, Context, anyhow};

#[derive(Clone)]
pub struct ShindenAPI {
    pub client: Client,
    jar: Arc<CookieStoreMutex>,
    cookie_path: PathBuf,
}

impl ShindenAPI {
    pub fn new() -> Result<Self> {
        let mut cookie_path = dirs::data_local_dir()
            .context("Could not find user local data directory")?;
        cookie_path.push("shinden_api");
        create_dir_all(&cookie_path)?;
        cookie_path.push("cookies.json");

        let store = if cookie_path.exists() {
            let file = File::open(&cookie_path)?;
            let reader = BufReader::new(file);
            CookieStore::load_json(reader).map_err(|e| anyhow!(e.to_string()))?
        } else {
            CookieStore::default()
        };

        let jar = Arc::new(CookieStoreMutex::new(store));

        let client = Client::builder()
            .cookie_provider(jar.clone())
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
            .build()?;

        Ok(Self { client, jar, cookie_path })
    }

    pub async fn get_html(&self, url: &str) -> Result<String> {
        let response = self.client.get(url).send().await?;
        let body = response.text().await?;
        self.save_cookies()?;
        Ok(body)
    }

    pub async fn post_form(&self, url: &str, form: &[(String, String)], headers: Option<HeaderMap>) -> Result<String> {
        let mut req = self.client.post(url).form(form);
        if let Some(h) = headers {
            req = req.headers(h);
        }
        let response = req.send().await?;
        let body = response.text().await?;
        self.save_cookies()?;
        Ok(body)
    }

    fn save_cookies(&self) -> Result<()> {
        if let Some(parent) = self.cookie_path.parent() {
            create_dir_all(parent)?;
        }
        let mut file = File::create(&self.cookie_path)?;
        let store = self.jar.lock().unwrap();
        store.save_json(&mut file).map_err(|e| anyhow!(e.to_string()))?;
        Ok(())
    }
}
