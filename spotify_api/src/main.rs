use reqwest;
use reqwest::header::{ACCEPT, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct ExternalUrls {
    spotify: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Artist {
    name: String,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct Album {
    name: String,
    artists: Vec<Artist>,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct Track {
    name: String,
    href: String,
    popularity: u32,
    album: Album,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    tracks: Items<Track>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Items<T> {
    items: Vec<T>,
}

fn print_tracks(tracks: Vec<&Track>) {
    for track in tracks {
        println!("Track: {}", track.name);
        println!("Album: {}", track.album.name);

        if let Some(artist) = track.album.artists.first() {
            println!("Artist: {}", artist.name);
        }

        println!("Popularity: {}", track.popularity);
        println!("Spotify: {}", track.external_urls.spotify);
        println!("-----------------------------");
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: cargo run <search_query> <auth_token>");
        return;
    }

    let search_query = &args[1];
    let auth_token = &args[2];

    let url = format!(
        "https://api.spotify.com/v1/search?q={}&type=track",
        search_query
    );

    let client = reqwest::Client::new();

    let response = client
        .get(url)
        .header(AUTHORIZATION, format!("Bearer {}", auth_token))
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => match response.json::<APIResponse>().await {
            Ok(parsed) => print_tracks(parsed.tracks.items.iter().collect()),
            Err(_) => println!("Response doesn't match the struct shape"),
        },

        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Provide valid API token");
        }

        _ => {
            println!("Unexpected response: {}", response.status());
        }
    }
}
