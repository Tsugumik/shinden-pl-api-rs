use crate::{client::ShindenAPI, models::Anime};
use anyhow::Result;
use scraper::{Html, Selector};

impl ShindenAPI {
    pub async fn search_anime(&self, name: &str) -> Result<Vec<Anime>> {
        let search_url = format!(
            "https://shinden.pl/series?search={}",
            name.replace(' ', "+")
        );
        let html = self.get_html(&search_url).await?;

        let doc = Html::parse_document(&html);
        let div_row = Selector::parse(".div-row").unwrap();
        let h3 = Selector::parse("h3").unwrap();
        let a = Selector::parse("a").unwrap();
        let cover = Selector::parse(".cover-col a").unwrap();
        let kind = Selector::parse(".title-kind-col").unwrap();
        let episodes = Selector::parse(".episodes-col").unwrap();
        let rating = Selector::parse(".rate-top").unwrap();

        let mut result = Vec::new();

        for div in doc.select(&div_row) {
            let name_elem = div.select(&h3).next().and_then(|h| h.select(&a).next());
            let name = name_elem
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default();
            let url = name_elem
                .and_then(|el| el.value().attr("href"))
                .unwrap_or("")
                .to_string();
            let img_href = div
                .select(&cover)
                .next()
                .and_then(|el| el.value().attr("href"))
                .unwrap_or("/res/other/placeholders/title/100x100.jpg");

            let full_url = format!("https://shinden.pl{}", url);
            let img_url = format!("https://shinden.pl{}", img_href);
            let anime_type = div
                .select(&kind)
                .next()
                .map(|k| k.text().collect::<String>())
                .unwrap_or_default();
            let ep_count = div
                .select(&episodes)
                .next()
                .map(|e| e.text().collect::<String>())
                .unwrap_or_default()
                .trim()
                .to_string();
            let rate = div
                .select(&rating)
                .next()
                .map(|r| r.text().collect::<String>())
                .unwrap_or_default();

            if !name.is_empty() {
                result.push(Anime {
                    name,
                    url: full_url,
                    image_url: img_url,
                    anime_type,
                    rating: rate,
                    episodes: ep_count,
                    description: String::new(),
                });
            }
        }

        Ok(result)
    }
}


