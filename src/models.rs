use serde::{Deserialize, Serialize};

/// Represents a single page of search results.
/// Contains pagination info and list of found anime.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchPageResult {
    /// The current page number (1-based).
    pub current_page: u32,
    /// The total number of available pages.
    pub total_pages: u32,
    /// The list of anime items found on the current page.
    pub anime_list: Vec<SearchAnimeItem>
}

/// Represents a single anime entry displayed on the search results list.
/// This structure contains summary data available without entering the details page.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchAnimeItem {
    /// The main title of the anime.
    pub title: String,
    /// The full URL to the anime details page (e.g., `https://shinden.pl/series/...`).
    pub url: String,
    /// The full URL to the cover image.
    pub image_url: String,
    /// The type of the anime (e.g., "TV", "OVA", "Movie").
    pub anime_type: String,
    /// The number of episodes. `None` if unknown or currently airing.
    pub episodes_count: Option<u32>,
    /// The emission status (e.g., "Zakończone" - Completed, "Emitowane" - Airing).
    pub status: String,
    /// The "TOP" score displayed prominently on the right side of the list row.
    pub top_score: Option<f64>,
    /// Detailed ratings breakdown (Overall, Plot, Graphics, etc.).
    pub search_ratings: Option<SearchRatings>,
    /// List of genres/tags associated with the title (e.g., "Action", "Fantasy").
    pub genres: Vec<String>
}

/// Detailed ratings breakdown found in the search list view.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchRatings {
    /// Overall rating.
    pub overall: f64,
    /// Plot/Story rating.
    pub plot: f64,
    /// Graphics/Animation rating.
    pub graphics: f64,
    /// Music/Sound rating.
    pub music: f64,
    /// Characters rating.
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
    pub other_tags: Vec<String>,
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
