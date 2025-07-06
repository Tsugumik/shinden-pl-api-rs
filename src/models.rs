use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Anime {
    pub name: String,
    pub url: String,
    pub image_url: String,
    pub anime_type: String,
    pub rating: String,
    pub episodes: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Episode {
    pub title: String,
    pub link: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    pub player: String,
    pub max_res: String,
    pub lang_audio: String,
    pub lang_subs: String,
    pub online_id: String,
}
