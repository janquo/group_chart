#![feature(try_trait)]

//#[macro_use]
extern crate image;
extern crate imageproc;
extern crate reqwest;
extern crate rusttype;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use serde_json::Value;
use std::cmp::Ordering;
use std::fs;
use std::fs::File;

pub struct Album {
    title: String,
    artist: String,
    playcount: i64,
    tracks: Option<usize>,
    score: Option<f64>,
    pub image: Option<String>,
}

impl Album {
    pub fn parse_album(data: &Value) -> Album {
        Album {
            title: String::from(data["name"].as_str().unwrap()),
            artist: String::from(data["artist"]["name"].as_str().unwrap()),
            playcount: data["playcount"].as_str().unwrap().parse().unwrap(),
            tracks: None,
            score: None,
            image: None,
        }
    }

    pub fn more_info(&mut self, key: &str) -> Result<(), std::option::NoneError> {
        let request_url = format!("http://ws.audioscrobbler.com/2.0/?method=album.getinfo&api_key={}&artist={}&album={}&format=json",
                              key, self.artist, self.title);
        let mut response = reqwest::get(&request_url).expect("no response from api");

        let data = response.json();
        let data : Value = match data {
            Ok(x) => x,
            _ => return Ok(()),
        };

        self.tracks = data["album"]["tracks"]["track"].as_array().map(|x| x.len());
        if self.tracks == Some(0) {
            self.tracks = None;
        }

        self.image = data["album"]["image"].as_array()?[2]["#text"]
            .as_str()
            .map(|s| String::from(s));

        self.compute_score();

        Ok(())
    }

    fn compute_score(&mut self) {
        if self.tracks.is_none() {
            return;
        }
        self.score = Some(self.playcount as f64 / (self.tracks.unwrap() as f64));
    }

    pub fn incr_playcount(&mut self, other: &Album) {
        self.playcount += other.playcount;
    }

    pub fn get_count(&self) -> i64 {
        self.playcount
    }

    pub fn get_score(&self) -> Option<f64> {
        self.score
    }

    pub fn compare_decr(&self, other: &Album) -> Ordering {
        if self.score.is_none() || other.score.is_none() {
            return other.playcount.cmp(&self.playcount);
        }
        other
            .score
            .unwrap()
            .partial_cmp(&self.score.unwrap())
            .unwrap()
    }
}
impl PartialEq for Album {
    fn eq(&self, other: &Album) -> bool {
        self.title == other.title && self.artist == other.artist
    }
}
impl Eq for Album {}
impl Ord for Album {
    fn cmp(&self, other: &Album) -> Ordering {
        self.title.cmp(&other.title)
    }
}
impl PartialOrd for Album {
    fn partial_cmp(&self, other: &Album) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::fmt::Display for Album {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} - {}, with score: {} ({})",
            self.artist, self.title, self.playcount, self.score.unwrap_or(0f64)
        )
    }
}

pub fn get_users() -> String {
    let contents =
        fs::read_to_string("users.txt").expect("Something went wrong reading the users.txt file");
    contents
}
pub fn get_key() -> String {
    let contents =
        fs::read_to_string("key.txt").expect("Something went wrong reading the key.txt file");
    contents
}

pub fn get_chart(user: &str, key: &str) -> Result<Value, reqwest::Error> {
    let request_url = format!("http://ws.audioscrobbler.com/2.0/?method=user.gettopalbums&user={}&api_key={}&period=7day&format=json",
                              user, key);
    let mut response = reqwest::get(&request_url)?;

    let answer: Value = response.json()?;

    Ok(answer)
}

pub fn download_image(target: &str) -> Result<String, reqwest::Error> {
    let mut response = reqwest::get(target)?;
    let mut result;

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");
        let fname = format!("./images/{}", fname);
        let fname = fname.as_str();
        result = String::from(fname);
        File::create(fname).unwrap()
    };
    std::io::copy(&mut response, &mut dest).unwrap();
    Ok(result)
}

pub mod drawer {

    pub fn collage(images: Vec<String>) {
        use image::GenericImage;

        let mut img = image::ImageBuffer::new(870, 870);

        for i in 0..25 as usize {
            let img2 = image::open(images[i].clone()).unwrap();

            img.copy_from(&img2, 174 * (i % 5) as u32, 174 * (i / 5) as u32);
        }
        //draw_description(&mut img);
        img.save("test.png").unwrap();
    }

    fn draw_description(img: &mut image::ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>>) {
        use rusttype::{FontCollection, Scale};

        let font = Vec::from(include_bytes!("comic.ttf") as &[u8]);
        let font = FontCollection::from_bytes(font)
            .unwrap()
            .into_font()
            .unwrap();

        let height = 12.4;
        let scale = Scale {
            x: height * 2.0,
            y: height,
        };
        imageproc::drawing::draw_text_mut(
            img,
            image::Rgba([0u8, 0u8, 255u8, 255u8]),
            0,
            0,
            scale,
            &font,
            "Hello, world!",
        );
    }
}
