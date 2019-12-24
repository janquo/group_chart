use super::Album;
use reqwest;
use serde_json::Value;

pub fn parse_album(data: &Value, user: String) -> Album {
    Album {
        title: String::from(data["name"].as_str().unwrap()),
        artist: String::from(data["artist"]["name"].as_str().unwrap()),
        playcount: data["playcount"].as_str().unwrap().parse().unwrap(),
        tracks: None,
        score: None,
        image: None,
        best_contributor: (user, data["playcount"].as_str().unwrap().parse().unwrap()),
        no_contributors: 1,
    }
}

pub fn get_chart(
    user: &str,
    key: &str,
    period: &str,
    client: &reqwest::Client,
) -> Result<Value, reqwest::Error> {
    let request_url = format!("http://ws.audioscrobbler.com/2.0/?method=user.gettopalbums&user={}&api_key={}&period={}&limit=1000&format=json",
                              user, key, period);
    let mut response = client.get(&request_url).send()?;

    let answer: Value = response.json()?;
    Ok(answer)
}

pub fn album_getinfo(
    album: &mut Album,
    key: &str,
    client: &reqwest::Client,
) -> Result<bool, reqwest::Error> {
    let request_url = format!("http://ws.audioscrobbler.com/2.0/?method=album.getinfo&api_key={}&artist={}&album={}&format=json",
                              key, album.artist.replace("&", "%26"), album.title.replace("&", "%26"));

    let mut response = client.get(&request_url).send()?;

    let data = response.json();
    let data: Value = match data {
        Ok(x) => x,
        _ => return Ok(true), //not an ideal solution, but shouldn't happen
    };

    album.tracks = data["album"]["tracks"]["track"].as_array().map(|x| x.len());
    if album.tracks == Some(0) {
        album.tracks = None;
    }

    album.image = match data["album"]["image"].as_array() {
        None => None,
        Some(x) => x[3]["#text"].as_str().map(String::from),
    };
    album.compute_score();

    if album.tracks == None {
        return Ok(true);
    }

    Ok(false)
}
