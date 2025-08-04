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

## Getting Started

To use this library in your Rust project, add it as a dependency in your Cargo.toml file:

```toml
[dependencies]
shinden-pl-api = "0.1.5"
```

Alternatively, you can add it directly via the Cargo CLI:
Bash

```sh
cargo add shinden-pl-api
```

### Example
```rust
use anyhow::Result;
use shinden_pl_api::client::ShindenAPI;
use shinden_pl_api::models::{Episode, Player};

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Initialize the API client.
    // This client manages cookies and sets appropriate default headers.
    let client = ShindenAPI::new()?;

    // 2. Log in to the service.
    // Replace with your actual login credentials.
    let _ = client.login("example@example.com", "example"); // Placeholder credentials

    let series_url = "https://shinden.pl/series/12434-hunter-x-hunter-2011";

    println!("Fetching episodes for series: {}", series_url);

    // 3. Fetch the list of episodes for the specified series.
    let episodes: Vec<Episode> = client.get_episodes(series_url).await?;
    println!("Found {} episodes:", episodes.len());

    for ep in &episodes {
        println!(" - {} [{}]", ep.title, ep.link);
    }

    // 4. If the episode list isn't empty, fetch players for the first episode.
    if let Some(first_episode) = episodes.first() {
        println!("\nFetching players for first episode: {}", first_episode.link);
        // `get_players` returns a list of available players on the episode page.
        let players: Vec<Player> = client.get_players(&first_episode.link).await?;
        println!("Found {} players:", players.len());

        // 5. Retrieve the specific player iframe (e.g., the first one from the list).
        // `online_id` is the player identifier you need to pass.
        let player_iframe_html = client.get_player_iframe(&players[0].online_id).await?;

        // Print the full <iframe> tag, which you can then embed in your frontend.
        println!("{}", player_iframe_html);
    }

    Ok(())
}
```

# LICENSE
This project is licensed under the MIT License.
