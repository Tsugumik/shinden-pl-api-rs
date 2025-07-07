use crate::{client::ShindenAPI, models::Episode};
use anyhow::{Context, Result};
// Potrzebne do .context()
use scraper::{Html, Selector};

impl ShindenAPI {
    pub async fn get_episodes(&self, link: &str) -> Result<Vec<Episode>> {
        let url = format!("{}/all-episodes", link);

        let html = self.get_html(&url).await?;
        let doc = Html::parse_document(&html);

        let tbody_selector = Selector::parse("tbody").unwrap();
        let tbody_element = doc.select(&tbody_selector)
            .next()
            .context("Could not find tbody element on the page")?;

        let title_selector = Selector::parse(".ep-title").unwrap();
        let button_selector = Selector::parse("a.button.active").unwrap();

        let mut episodes = Vec::new();

        for el in tbody_element.select(&title_selector) {
            episodes.push(Episode {
                title: el.text().collect::<String>(),
                link: String::new(),
            });
        }

        for (i, el) in tbody_element.select(&button_selector).enumerate() {
            if let Some(href) = el.value().attr("href") {
                if let Some(ep) = episodes.get_mut(i) {
                    ep.link = format!("https://shinden.pl{}", href);
                }
            }
        }

        episodes.reverse();
        Ok(episodes)
    }
}
