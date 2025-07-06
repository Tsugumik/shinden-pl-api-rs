use std::sync::Arc;
use reqwest::{Client, header::HeaderMap};
use cookie_store::CookieStore;
use reqwest_cookie_store::CookieStoreMutex;
use anyhow::Result;

pub struct ShindenAPI {
    pub client: Client,
}

impl ShindenAPI {
    pub fn new() -> Result<Self> {
        let store = CookieStore::default();
        let jar = Arc::new(CookieStoreMutex::new(store));

        let client = Client::builder()
            .cookie_provider(jar)
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
            .build()?;

        Ok(Self { client })
    }

    pub async fn get_html(&self, url: &str) -> Result<String> {
        let response = self.client.get(url).send().await?;
        Ok(response.text().await?)
    }

    pub async fn post_form(&self, url: &str, form: &[(String, String)], headers: Option<HeaderMap>) -> Result<String> {
        let mut req = self.client.post(url).form(form);

        if let Some(headers) = headers {
            req = req.headers(headers);
        }

        let response = req.send().await?;
        Ok(response.text().await?)
    }
}