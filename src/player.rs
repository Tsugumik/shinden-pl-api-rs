use crate::{client::ShindenAPI, models::Player};
use anyhow::Result;
use scraper::{Html, Selector};
use serde_json::Value;

impl ShindenAPI {
    pub async fn get_players(&self, episode_link: &str) -> Result<Vec<Player>> {
        let html = self.get_html(episode_link).await?;
        let doc = Html::parse_document(&html);
        let buttons_selector = Selector::parse(".ep-buttons a").unwrap();

        let mut result = Vec::new();
        for el in doc.select(&buttons_selector) {
            if let Some(data) = el.value().attr("data-episode") {
                if let Ok(json) = serde_json::from_str::<Value>(data) {
                    let player = Player {
                        player: json["player"].as_str().unwrap_or("").to_string(),
                        max_res: json["max_res"].as_str().unwrap_or("").to_string(),
                        lang_audio: json["lang_audio"].as_str().unwrap_or("").to_string(),
                        lang_subs: json["lang_subs"].as_str().unwrap_or("").to_string(),
                        online_id: json["online_id"].as_str().unwrap_or("").to_string(),
                    };
                    result.push(player);
                }
            }
        }

        Ok(result)
    }
}

