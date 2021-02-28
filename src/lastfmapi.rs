use super::Album;
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

pub async fn get_chart(
    user: &str,
    key: &str,
    period: &str,
    client: &reqwest::Client,
) -> Result<Value, Box<dyn std::error::Error>> {
    let request_url = format!("http://ws.audioscrobbler.com/2.0/?method=user.gettopalbums&user={}&api_key={}&period={}&limit=1000&format=json",
                              user, key, period);
    let response = client.get(&request_url).send().await?;

    let answer: Value = response.json().await?;
    Ok(answer)
}

pub async fn album_getinfo(
    album: &Album,
    key: &str,
    client: &reqwest::Client,
) -> Result<Album, Box<dyn std::error::Error>> {
    let request_url = format!("http://ws.audioscrobbler.com/2.0/?method=album.getinfo&api_key={}&artist={}&album={}&format=json",
                              key, album.artist.replace("&", "%26"), album.title.replace("&", "%26"));

    let response = client.get(&request_url).send().await?;

    let data: Value = response.json().await?;

    let tracks = data["album"]["tracks"]["track"].as_array().map(Vec::len);

    let image = match data["album"]["image"].as_array() {
        Some(x) => x[3]["#text"].as_str().map(String::from),
        None => None,
    };

    let mut result = Album {
        tracks,
        image,
        ..album.clone()
    };

    result.compute_score();

    Ok(result)
}

pub fn error_code(response: &Value) -> Option<i64> {
    if response["error"] != serde_json::Value::Null {
        response["error"].as_i64().unwrap().into()
    } else {
        None
    }
}
