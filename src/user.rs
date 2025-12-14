use scraper::{Html, Selector};
use crate::client::ShindenHttpClient;
use crate::error::ShindenError;
use crate::headers::RequestType;

pub async fn get_username(client: &ShindenHttpClient) -> Result<Option<String>, ShindenError> {
    let html = client.get_html(
        "https://shinden.pl/user", RequestType::Frontend
    ).await?;

    let doc = Html::parse_document(&html);

    let selector = Selector::parse("title")
        .map_err(|e| ShindenError::HtmlParsing(e.to_string()))?;

    let title_element = doc.select(&selector).next();

    if let Some(t) = title_element {
        let text = t.text().collect::<String>();

        let username = text.split("(użytkownik)")
            .next()
            .unwrap_or("")
            .trim();

        if username.is_empty() {
            return Ok(None);
        }

        return Ok(Some(username.to_string()));

    }

    Ok(None)
}

pub async fn get_profile_image(client: &ShindenHttpClient) -> Result<Option<String>, ShindenError> {
    let html = client.get_html(
        "https://shinden.pl/user", RequestType::Frontend
    ).await?;

    let doc = Html::parse_document(&html);

    let selector = Selector::parse(".info-aside-img")
        .map_err(|e| ShindenError::HtmlParsing(e.to_string()))?;

    let img = doc.select(&selector).next();

    if let Some(img_el) = img {
        if let Some(src) = img_el.value().attr("src") {
            return Ok(Some(format!("https://shinden.pl{}", src)));
        }
    }

    Ok(None)
}
