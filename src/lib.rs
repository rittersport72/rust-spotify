pub mod api;

use crate::api::APIResponse;
use reqwest;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};

pub struct Spotify {
    auth_token: String,
    client: reqwest::blocking::Client,
}

impl Spotify {
    pub fn new(auth_token: &str) -> Self {
        Self {
            auth_token: auth_token.to_string(),
            client: reqwest::blocking::Client::new(),
        }
    }

    pub fn search_artist(&self) {
        let url = format!(
            "https://api.spotify.com/v1/search?q={query}&type=track,artist",
            query = "Roxette"
        );

        let response = self
            .client
            .get(url)
            .header(AUTHORIZATION, format!("Bearer {}", self.auth_token))
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .send()
            .unwrap();
        //.text();

        match response.status() {
            reqwest::StatusCode::OK => {
                println!("Success! {:?}", response);
                // on success, parse our JSON to an APIResponse
                match response.json::<APIResponse>() {
                    Ok(parsed) => println!("Success! {:?}", parsed),
                    Err(_) => println!("Hm, the response didn't match the shape we expected."),
                };
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                println!("Need to grab a new token");
            }
            _ => {
                panic!("Uh oh! Something unexpected happened.");
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_search_artist() {
        let spot = Spotify::new("AUTH_TOKEN");

        spot.search_artist();
    }
}
