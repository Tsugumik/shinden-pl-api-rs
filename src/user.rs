use crate::client::ShindenAPI;
use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue};
use scraper::{Html, Selector};
use crate::headers::get_headers_for_type;

impl ShindenAPI {
    pub async fn login(&self, email: &str, password: &str) -> Result<()> {
        let url = "https://shinden.pl/main/login";
        let _get_response_body = self.get_html(url).await?;

        let form = vec![
            ("username".into(), email.into()),
            ("password".into(), password.into()),
            ("remember".into(), "on".into()),
        ];

        let mut post_specific_headers = HeaderMap::new();
        post_specific_headers.insert(reqwest::header::REFERER, HeaderValue::from_static("https://shinden.pl/main/login")); // Często wymagane

        self.post_form(url, &form, Some(post_specific_headers)).await?;

        Ok(())
    }

    pub async fn logout(&self) -> Result<()> {
        let url = "https://shinden.pl/main/logout";
        self.get_html(url).await?;
        Ok(())
    }

    pub async fn get_user_name(&self) -> Result<Option<String>> {
        let html = self.get_html("https://shinden.pl/user").await?;
        let doc = Html::parse_document(&html);
        let title = doc
            .root_element()
            .select(&Selector::parse("title").unwrap())
            .next();
        if let Some(t) = title {
            let text = t.text().collect::<String>();
            let username = text.split("(użytkownik)").next().unwrap_or("").trim();
            return Ok(Some(username.to_string()));
        }
        Ok(None)
    }

    pub async fn get_user_profile_image(&self) -> Result<Option<String>> {
        let html = self.get_html("https://shinden.pl/user").await?;
        let doc = Html::parse_document(&html);
        let img = doc
            .select(&Selector::parse(".info-aside-img").unwrap())
            .next();
        if let Some(img_el) = img {
            if let Some(src) = img_el.value().attr("src") {
                return Ok(Some(format!("https://shinden.pl{}", src)));
            }
        }
        Ok(None)
    }
}
