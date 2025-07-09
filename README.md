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

# LICENSE
This project is licensed under the MIT License.
