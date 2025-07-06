use crate::{client::ShindenAPI, models::Episode};
use anyhow::Result;
use scraper::{Html, Selector};

pub async fn get_episodes(client: &ShindenAPI, link: &str) -> Result<Vec<Episode>> {
    let url = format!("{}/all-episodes", link);
    let html = client.get_html(&url).await?;
    let doc = Html::parse_document(&html);

    let title_selector = Selector::parse(".ep-title").unwrap();
    let button_selector = Selector::parse("a.button.active").unwrap();

    let mut episodes = Vec::new();

    for el in doc.select(&title_selector) {
        episodes.push(Episode {
            title: el.text().collect::<String>(),
            link: String::new(),
        });
    }

    for (i, el) in doc.select(&button_selector).enumerate() {
        if let Some(href) = el.value().attr("href") {
            if let Some(ep) = episodes.get_mut(i) {
                ep.link = format!("https://shinden.pl{}", href);
            }
        }
    }

    episodes.reverse();
    Ok(episodes)
}
