#![feature(try_trait)]

//#[macro_use]
extern crate image;
extern crate imageproc;
extern crate num_rational;
extern crate reqwest;
extern crate rusttype;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use num_rational::Ratio;
use serde_json::Value;
use std::cmp::Ordering;
use std::collections::{BTreeSet, BinaryHeap};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;

#[derive(Clone)]
pub struct Album {
    title: String,
    artist: String,
    playcount: i64,
    tracks: Option<usize>,
    score: Option<Ratio<i64>>,
    pub image: Option<String>,
    mbid: Option<String>,
}

impl Album {
    fn parse_album(data: &Value) -> Album {
        Album {
            title: String::from(data["name"].as_str().unwrap()),
            artist: String::from(data["artist"]["name"].as_str().unwrap()),
            playcount: data["playcount"].as_str().unwrap().parse().unwrap(),
            tracks: None,
            score: None,
            image: None,
            mbid: data["mbid"].as_str().map(|s| String::from(s)),
        }
    }

    pub fn new(artist: String, title: String) -> Album {
        Album {
            title,
            artist,
            playcount: 0,
            tracks: None,
            score: None,
            image: None,
            mbid: None,
        }
    }
    pub fn more_info(&mut self, key: &str) -> Result<(), reqwest::Error> {
        /* let request_url = if self.mbid.is_none()
            || self.mbid.clone().unwrap_or(String::new()).is_empty() == true
        {
            format!("http://ws.audioscrobbler.com/2.0/?method=album.getinfo&api_key={}&artist={}&album={}&format=json",
                              key, self.artist.replace("&", "%26"), self.title.replace("&", "%26"))
        } else {
            format!("http://ws.audioscrobbler.com/2.0/?method=album.getinfo&api_key={}&mbid={}&format=json",
                                                    key, self.mbid.clone().unwrap())
        };*/
        let request_url = format!("http://ws.audioscrobbler.com/2.0/?method=album.getinfo&api_key={}&artist={}&album={}&format=json",
                          key, self.artist.replace("&", "%26"), self.title.replace("&", "%26"));

        let mut response = reqwest::get(&request_url)?;

        let data = response.json();
        let data: Value = match data {
            Ok(x) => x,
            _ => return Ok(()),
        };

        self.tracks = data["album"]["tracks"]["track"].as_array().map(|x| x.len());
        if self.tracks == Some(0) {
            self.tracks = None;
        }

        self.image = match data["album"]["image"].as_array() {
            None => None,
            Some(x) => x[2]["#text"].as_str().map(|s| String::from(s)),
        };

        self.compute_score();

        Ok(())
    }

    fn compute_score(&mut self) {
        if self.tracks.is_none() || self.tracks == Some(1) || self.tracks == Some(0) {
            return;
        }
        self.score = Some(Ratio::new(self.playcount, self.tracks.unwrap() as i64));
    }

    pub fn incr_playcount(&mut self, other: &Album) {
        self.playcount += other.playcount;
    }

    pub fn playcount(&self) -> i64 {
        self.playcount
    }
    pub fn title(&self) -> &String {
        &self.title
    }
    pub fn artist(&self) -> &String {
        &self.artist
    }

    pub fn score(&self) -> Option<Ratio<i64>> {
        self.score
    }

    pub fn insert(albums: &mut BTreeSet<Album>, user_albums: &Vec<Value>) {
        for album in user_albums.iter().map(|x| Album::parse_album(x)) {
            //eprintln!("adding {} to counter of album {} by user {}", count, name, user);
            //insert returns false if same entry exists in a set
            if albums.contains(&album) {
                let mut old = albums.take(&album).unwrap();
                old.incr_playcount(&album);
                albums.insert(old);
            } else {
                albums.insert(album);
            }
        }
    }

    pub fn sorted_vec(albums: BTreeSet<Album>) -> Vec<Album> {
        let mut res: Vec<Album> = albums.into_iter().collect();
        res.sort_by_key(|album| -album.playcount());
        res
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
    pub fn to_string_semic(&self) -> String {
        format!("{};{};{}", self.artist, self.title, self.playcount)
    }

    pub fn tracks_from_file(albums: &mut BTreeSet<Album>) -> Result<(), Box<dyn Error>> {
        let content = fs::read_to_string("manual_tracks.txt")?;
        for line in content.lines() {
            let mut words = line.split(";");
            let (artist, title, tracks) = (words.next(), words.next(), words.next());
            if tracks == None {
                continue;
            }
            let current = albums.get(&Album::new(
                String::from(artist.unwrap()),
                String::from(title.unwrap()),
            ));
            match current {
                None => continue,
                Some(x) => {
                    if x.score.is_some() {
                        //for example Prince 1999 appears to be 2-tracks on last.fm so should force file version
                        //continue;
                    }
                }
            }
            let mut updated = (*(current.clone().unwrap())).clone();
            updated.tracks = tracks.map(|x| x.parse().unwrap_or(0));
            updated.compute_score();
            albums.replace(updated);
        }
        Ok(())
    }

    pub fn with_no_score(albums: &BTreeSet<Album>) -> Vec<&Album> {
        let mut top_none: Vec<&Album> = albums.iter().filter(|x| x.score.is_none()).collect();
        top_none.sort_by_key(|x| -x.playcount);
        top_none
    }
    pub fn with_score(albums: &BTreeSet<Album>) -> Vec<&Album> {
        let mut top_some: Vec<&Album> = albums.iter().filter(|x| x.score.is_some()).collect();
        top_some.sort_by(|x, y| y.score.unwrap().partial_cmp(&x.score.unwrap()).unwrap());
        top_some
    }

    pub fn get_images(albums: &Vec<&Album>) -> Vec<String> {
        let mut cover_urls: Vec<String> = Vec::new();
        for album in albums.iter() {
            match &album.image {
                Some(x) => cover_urls.push(download_image(&x).unwrap_or(String::from("blank.png"))),
                _ => cover_urls.push(String::from("blank.png")),
            }
        }
        cover_urls
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
            self.artist,
            self.title,
            self.playcount,
            self.score.unwrap_or(Ratio::new(0, 1))
        )
    }
}

pub fn parse_args(args: Vec<String>) -> (u32, u32, String, bool) {
    let (mut x, mut y, mut period, mut captions) = (5u32, 5u32, String::from("7day"), true);
    let mut args = args.into_iter();
    let mut arg = args.next();
    while arg.is_some() {
        match arg.unwrap().as_str() {
            "-x" => x = args.next().unwrap().parse().unwrap(),
            "-y" => y = args.next().unwrap().parse().unwrap(),
            "-p" => period = args.next().unwrap(),
            "-c" => captions = false,
            _ => (),
        }
        arg = args.next();
    }
    (x, y, period, captions)
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

pub fn get_chart(user: &str, key: &str, period: &str) -> Result<Value, reqwest::Error> {
    let request_url = format!("http://ws.audioscrobbler.com/2.0/?method=user.gettopalbums&user={}&api_key={}&period={}&format=json",
                              user, key, period);
    let mut response = reqwest::get(&request_url)?;

    let answer: Value = response.json()?;

    Ok(answer)
}

///returns false if album shouldn't be considered
pub fn top_scores_update(
    album: &Album,
    top_number: usize,
    scores: &mut BinaryHeap<Ratio<i64>>,
) -> bool {
    let smallest = -scores.peek().unwrap_or(&Ratio::new(-100000, 1));

    match album.score() {
        Some(score) => {
            if score < smallest && scores.len() < top_number {
                scores.push(-score);
                true
            } else if score >= smallest {
                scores.push(-score);
                if scores.len() > top_number {
                    scores.pop();
                }
                true
            } else {
                false
            }
        }
        None => true,
    }
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

pub fn nones_to_file(nones: &Vec<&Album>) -> Result<(), std::io::Error> {
    let mut file = File::create("nones.txt")?;
    file.write_all(
        nones
            .iter()
            .map(|x| x.to_string_semic())
            .collect::<Vec<String>>()
            .join("\n")
            .as_bytes(),
    )?;
    Ok(())
}

pub fn sleep(x: u64) {
    std::thread::sleep(std::time::Duration::from_millis(x));
}

pub fn wants_again() -> Result<bool, std::io::Error> {
    let mut answer = String::new();
    std::io::stdin().read_line(&mut answer)?;
    if answer.chars().next().unwrap_or('Y') == 'N' {
        Ok(false)
    } else {
        Ok(true)
    }
}

pub mod drawer {

    pub fn collage(
        images: Vec<String>,
        albums: Vec<&super::Album>,
        x: u32,
        y: u32,
        captions: bool,
    ) {
        use image::GenericImage;

        let mut img = image::ImageBuffer::new(174 * x, 174 * y);

        for ((i, image), album) in (0..(x * y)).zip(images.iter()).zip(albums.iter()) {
            let img2 = image::open(image).unwrap();
            let mut img2 = img2.to_rgba();
            if captions {
                draw_description(&mut img2, album.artist(), album.title());
            }
            img.copy_from(&img2, 174 * (i % x), 174 * (i / x));
        }
        img.save("test.png").unwrap();
    }

    fn draw_description(
        img: &mut image::ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>>,
        author: &String,
        title: &String,
    ) {
        use rusttype::{FontCollection, Scale};

        let font = Vec::from(include_bytes!("comic.ttf") as &[u8]);
        let font = FontCollection::from_bytes(font)
            .unwrap()
            .into_font()
            .unwrap();

        /*let font_shadow = Vec::from(include_bytes!("comicbd.ttf") as &[u8]);
        let font_shadow = FontCollection::from_bytes(font_shadow)
            .unwrap()
            .into_font()
            .unwrap();*/

        let height = 8.5;
        let scale = Scale {
            x: height * 1.5,
            y: height,
        };
        let mut draw = |fnt, txt, x, y, col| {
            imageproc::drawing::draw_text_mut(
                img,
                image::Rgba([col, col, col, col]),
                x,
                y,
                scale,
                fnt,
                txt,
            )
        };
        let mut with_shadow = |txt, y| {
            draw(&font, txt, 0, y, 0u8);
            draw(&font, txt, 0, y + 2u32, 0u8);
            draw(&font, txt, 2, y + 0u32, 0u8);
            draw(&font, txt, 2, y + 2u32, 0u8);
            draw(&font, txt, 1, y + 1u32, 240u8);
        };

        with_shadow(author, 0);
        with_shadow(title, 9);
    }
}
