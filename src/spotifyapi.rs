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
