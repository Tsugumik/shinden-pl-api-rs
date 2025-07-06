# Shinden.pl API Client (Rust)

A simple and blazingly fast API client for the Shinden.pl service, written in Rust. 
This project enables programmatic interaction with the service, 
such as logging in, fetching episode lists, searching for anime, and retrieving video player links, 
all while leveraging Rust's performance and safety guarantees.

## Features
- User Authentication: Handles session management and user login.
- Episode Listing: Extracts titles and links to individual episode pages.
- Player Retrieval: Obtains data about available video players for a given episode (e.g., Mega.nz, CDA.pl iframes).
- Cookie Management: Persistently stores and manages session cookies.

## Usage
Here's an example of how to use the library to log in, fetch an episode list for a series, and extract a player link for the first episode.
```rust
use anyhow::Result;
use shinden_pl_api::client::ShindenAPI;
use shinden_pl_api::episodes::get_episodes;
use shinden_pl_api::models::{Episode, Player};
use shinden_pl_api::player::get_players;
use shinden_pl_api::user::login;
use shinden_pl_api::video::get_player_iframe;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Initialize the API client.
    // This client manages cookies and sets appropriate default headers.
    let client = ShindenAPI::new()?;

    // 2. Log in to the service.
    // Replace with your actual login credentials.
    let _ = login(&client, "example@example.com", "example"); // Placeholder credentials

    let series_url = "https://shinden.pl/series/12434-hunter-x-hunter-2011";

    println!("Fetching episodes for series: {}", series_url);

    // 3. Fetch the list of episodes for the specified series.
    let episodes: Vec<Episode> = get_episodes(&client, series_url).await?;
    println!("Found {} episodes:", episodes.len());

    for ep in &episodes {
        println!(" - {} [{}]", ep.title, ep.link);
    }

    // 4. If the episode list isn't empty, fetch players for the first episode.
    if let Some(first_episode) = episodes.first() {
        println!("\nFetching players for first episode: {}", first_episode.link);
        // `get_players` returns a list of available players on the episode page.
        let players: Vec<Player> = get_players(&client, &first_episode.link).await?;
        println!("Found {} players:", players.len());

        // 5. Retrieve the specific player iframe (e.g., the first one from the list).
        // `online_id` is the player identifier you need to pass.
        let player_iframe_html = get_player_iframe(&client, &players[0].online_id).await?;

        // Print the full <iframe> tag, which you can then embed in your frontend.
        println!("{}", player_iframe_html);
    }

    Ok(())
}
```

Search anime example.
```rust
println!("\n--- Searching for anime ---");
let search_query = "Naruto"; // You can change this query

println!("Searching for: '{}'", search_query);
let search_results: Vec<Anime> = search_anime(&client, search_query).await?;

if search_results.is_empty() {
    println!("No anime found for '{}'", search_query);
} else {
    println!("Found {} results for '{}':", search_results.len(), search_query);
    for anime in search_results {
        println!("  - Name: {}", anime.name);
        println!("    URL: {}", anime.url);
        println!("    Image: {}", anime.image_url);
        println!("    Type: {}", anime.anime_type);
        println!("    Episodes: {}", anime.episodes);
        println!("    Rating: {}", anime.rating);
        println!("    ---");
    }
}
```

# LICENSE
This project is licensed under the MIT License.