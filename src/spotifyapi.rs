use reqwest;
use serde_json;

pub fn get_access_token(id: &str, secret: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let mut response = client
        .post("https://accounts.spotify.com/api/token")
        .basic_auth(id, Some(secret))
        .form(&[("grant_type", "client_credentials")])
        .send()?;
    let json: serde_json::Value = response.json()?;
    Ok(String::from(json["access_token"].as_str().unwrap()))
}

pub fn search_album(auth_token: &str, query: &str) {
    let client = reqwest::Client::new();
    let mut response = client
        .get("https://api.spotify.com/v1/search")
        .bearer_auth(auth_token)
        .query(&[("q", query), ("type", "album"), ("limit", "3")])
        .send()
        .unwrap();
    let json: serde_json::Value = response.json().unwrap();
    println!("{}", json);
}
