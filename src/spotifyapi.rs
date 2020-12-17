use super::Album;
use std::io::{Error, ErrorKind};

pub fn get_access_token(id: &str, secret: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut response = client
        .post("https://accounts.spotify.com/api/token")
        .basic_auth(id, Some(secret))
        .form(&[("grant_type", "client_credentials")])
        .send()?;
    let json: serde_json::Value = response.json()?;

    json["access_token"].as_str()
        .map(String::from)
        .ok_or(Box::new(Error::new(ErrorKind::Other, "wrong response")))
}

impl Album {
    fn from_value(value: &serde_json::Value) -> Option<Self> {
        let tracks = match value["album_type"].as_str()?{
            "album" | "compilation" => value["total_tracks"].as_u64()?,
            "single" => 1,
            s => panic!("there is another option {}, implement it!", s),
        } as usize;
        let images = value["images"].as_array()?;
        let mut image: Option<String> = None;
        for img in images.iter() {
            image = match img["height"].as_u64()? {
                300 => Some(String::from(img["url"].as_str()?)),
                _ => None,
            };
        }
        Some(Album {
            title: String::from(value["name"].as_str()?),
            artist: String::from(
                value["artists"].as_array().unwrap()[0]["name"]
                    .as_str()?
            ),
            playcount: 0,
            tracks: Some(tracks),
            score: None,
            image,
            best_contributor: (String::from("NaN"), 0),
            no_contributors: 0,
        })
    }
}
pub fn search_album(
    auth_token: &str,
    album: &Album,
    limit: usize,
) -> Result<Vec<Album>, reqwest::Error> {
    let query = format!("album:{} artist:{}", album.title, album.artist);
    let client = reqwest::Client::new();
    let mut response = client
        .get("https://api.spotify.com/v1/search")
        .bearer_auth(auth_token)
        .query(&[
            ("q", &query as &str),
            ("type", "album"),
            ("limit", &limit.to_string()),
        ])
        .send()?;
    let json: serde_json::Value = response.json()?;
    let albums = json["albums"]["items"]
        .as_array()
        .unwrap()
        .iter()
        .map(Album::from_value)
        .filter(Option::is_some)
        .map(Option::unwrap);
    Ok(albums.collect())
}

pub fn get_non_single(auth_token: &str, album: &Album) -> Result<Option<Album>, reqwest::Error> {
    let mut albums = search_album(auth_token, album, 2)?;
    let mut result = None;
    if !albums.is_empty() {
        if albums[0].tracks() > 1 {
            result = Some(albums.remove(0));
        } else if albums.len() > 1 && albums[1].tracks() > 1 {
            result = Some(albums.remove(1));
        }
    }
    Ok(result)
}
