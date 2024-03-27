use reqwest;

pub fn search_artist() {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get("https://api.spotify.com/v1/search")
        .send()
        .unwrap();
        //.text();

    match response.status() {
        reqwest::StatusCode::OK => {
            println!("Success! {:?}", response);
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Need to grab a new token");
        }
        _ => {
            panic!("Uh oh! Something unexpected happened.");
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_search_artist() {
        search_artist();
    }
}
