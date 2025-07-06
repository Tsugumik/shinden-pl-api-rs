use crate::client::ShindenAPI;
use scraper::{Html, Selector};
use anyhow::Result;

pub async fn login(client: &ShindenAPI, email: &str, password: &str) -> Result<()> {
    let url = "https://shinden.pl/main/login";
    client.get_html(url).await?;
    let form = vec![
        ("username".into(), email.into()),
        ("password".into(), password.into()),
        ("remember".into(), "on".into()),
    ];
    client.post_form(url, &form, None).await?;
    Ok(())
}

pub async fn get_user_name(client: &ShindenAPI) -> Result<Option<String>> {
    let html = client.get_html("https://shinden.pl/user").await?;
    let doc = Html::parse_document(&html);
    let title = doc.root_element().select(&Selector::parse("title").unwrap()).next();
    if let Some(t) = title {
        let text = t.text().collect::<String>();
        let username = text.split("(użytkownik)").next().unwrap_or("").trim();
        return Ok(Some(username.to_string()));
    }
    Ok(None)
}

pub async fn get_user_profile_image(client: &ShindenAPI) -> Result<Option<String>> {
    let html = client.get_html("https://shinden.pl/user").await?;
    let doc = Html::parse_document(&html);
    let img = doc.select(&Selector::parse(".info-aside-img").unwrap()).next();
    if let Some(img_el) = img {
        if let Some(src) = img_el.value().attr("src") {
            return Ok(Some(format!("https://shinden.pl{}", src)));
        }
    }
    Ok(None)
}