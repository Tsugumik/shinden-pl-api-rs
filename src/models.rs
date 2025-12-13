use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchPageResult {
    pub current_page: u32,
    pub total_pages: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchAnimeItem {
    pub title: String,
    pub url: String,
    pub image_url: String,
    pub anime_type: String,
    pub episode_count: Option<u32>,
    pub status: String,
    pub top_score: Option<f64>,
    pub search_ratings: Option<SearchRatings>,
    pub genres: Vec<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchRatings {
    pub overall: f64,
    pub plot: f64,
    pub graphics: f64,
    pub music: f64,
    pub characters: f64
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnimeDetails {
    pub title: String,
    pub synonyms: Vec<String>,
    pub image_url: String,
    pub description: String,

    pub status: String,
    pub anime_type: String,
    pub start_date: String,
    pub end_date: Option<String>,

    pub episodes_count: Option<u32>,

    pub episode_length: Option<u32>,

    pub studios: Vec<String>,
    pub mpaa: String,

    pub rating_score: f64,
    pub rating_votes: u32,

    pub ratings_breakdown: Option<SearchRatings>,

    pub genres: Vec<String>,
    pub target_groups: Vec<String>,
    pub others_tags: Vec<String>,
    pub character_types: Vec<String>,
    pub place_and_time: Vec<String>,
    pub source_material: String,

    pub episodes: Vec<Episode>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Episode {
    pub title: String,
    pub number: u32,
    pub link: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    pub player: String,
    pub max_res: String,
    pub lang_audio: String,
    pub lang_subs: String,
    pub online_id: String
}
