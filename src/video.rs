use crate::client::ShindenAPI;
use anyhow::Result;
use scraper::{Html, Selector};
use std::time::Duration;
use tokio::time::sleep;


impl ShindenAPI {
    pub async fn get_player_iframe(&self, online_id: &str) -> Result<String> {
        let url1 = format!(
            "https://api4.shinden.pl/xhr/{}/player_load?auth=X2d1ZXN0XzowLDUsMjEwMDAwMDAsMjU1LDQxNzQyOTM2NDQ%3D",
            online_id
        );
        let url2 = format!(
            "https://api4.shinden.pl/xhr/{}/player_show?auth=X2d1ZXN0XzowLDUsMjEwMDAwMDAsMjU1LDQxNzQyOTM2NDQ%3D&width=0&height=-1",
            online_id
        );

        let _ = self.get_html(&url1).await?;
        sleep(Duration::from_secs(5)).await;
        let html = self.get_html(&url2).await?;

        let doc = Html::parse_document(&html);
        let iframe = doc.select(&Selector::parse("iframe").unwrap()).next();

        Ok(iframe.map(|i| i.html()).unwrap_or_default())
    }
}

