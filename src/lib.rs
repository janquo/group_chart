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
use std::collections::{BTreeSet, BinaryHeap, HashSet};
use std::fs;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io;
use std::io::Write;

pub mod config;
pub mod drawer;
pub mod reader;

pub struct Args {
    pub x: u32,
    pub y: u32,
    pub period: String,
    pub captions: bool,
    pub nick: Option<String>,
    pub web: bool,
    pub path_write: String,
    pub path_read: String,
    pub path_out: String,
    pub path_web: String,
}

#[derive(Clone)]
pub struct Album {
    title: String,
    artist: String,
    playcount: i64,
    tracks: Option<usize>,
    score: Option<Ratio<i64>>,
    pub image: Option<String>,
    best_contributor: (String, i64),
    no_contributors: i64,
}

impl Album {
    fn parse_album(data: &Value, user: String) -> Album {
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

    pub fn new(artist: String, title: String) -> Album {
        Album {
            title,
            artist,
            playcount: 0,
            tracks: None,
            score: None,
            image: None,
            best_contributor: (String::from("NaN"), 0),
            no_contributors: 0,
        }
    }
    pub fn more_info(
        &mut self,
        database: &HashSet<Album>,
        key: &str,
        client: &reqwest::Client,
    ) -> Result<bool, reqwest::Error> {
        if let Some(album) = database.get(&self) {
            self.tracks = album.tracks;
            self.image = album.image.clone();

            self.compute_score();

            Ok(true)
        } else {
            let request_url = format!("http://ws.audioscrobbler.com/2.0/?method=album.getinfo&api_key={}&artist={}&album={}&format=json",
                                      key, self.artist.replace("&", "%26"), self.title.replace("&", "%26"));

            let mut response = client.get(&request_url).send()?;

            let data = response.json();
            let data: Value = match data {
                Ok(x) => x,
                _ => return Ok(true), //not an ideal solution, but shouldn't happen
            };

            self.tracks = data["album"]["tracks"]["track"].as_array().map(|x| x.len());
            if self.tracks == Some(0) {
                self.tracks = None;
            }

            self.image = match data["album"]["image"].as_array() {
                None => None,
                Some(x) => x[3]["#text"].as_str().map(String::from),
            };
            self.compute_score();

            if self.tracks == None {
                return Ok(true);
            }

            Ok(false)
        }
    }

    fn compute_score(&mut self) {
        if self.tracks.is_none() || self.tracks == Some(0) {
            return;
        }
        self.score = Some(Ratio::new(
            if self.tracks == Some(1) {
                0
            } else {
                self.playcount
            },
            self.tracks.unwrap() as i64,
        ));
    }

    pub fn merge(&mut self, other: &Album) {
        self.no_contributors += 1;
        self.playcount += other.playcount;
        if self.best_contributor.1 < other.best_contributor.1 {
            self.best_contributor = other.best_contributor.clone();
        }
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

    pub fn insert(albums: &mut BTreeSet<Album>, user_albums: &[Value], user: &str) {
        for album in user_albums
            .iter()
            .map(|x| Album::parse_album(x, String::from(user)))
        {
            //insert returns false if same entry exists in a set
            if albums.contains(&album) {
                let mut old = albums.take(&album).unwrap();
                old.merge(&album);
                albums.insert(old);
            } else {
                albums.insert(album);
            }
        }
    }

    pub fn rev_sorted_vec(albums: BTreeSet<Album>) -> Vec<Album> {
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
    pub fn to_database_format(&self) -> String {
        format!(
            "{};{};{};{}\n",
            self.artist,
            self.title,
            self.tracks.unwrap_or(0),
            match &self.image {
                Some(x) => &x[..],
                None => "blank.png",
            }
        )
    }
    pub fn to_html_card(&self) -> String {
        format!(
            include_str!("../data/html_card"),
            match &self.image {
                Some(x) => &x[..],
                None => "blank.png",
            },
            self.artist,
            self.title,
            self.playcount,
            match &self.score {
                Some(x) => (*x.numer() as f64) / (*x.denom() as f64),
                None => 0.0,
            },
            self.no_contributors,
            self.best_contributor.0,
            self.best_contributor.1,
        )
    }

    pub fn tracks_from_file(
        albums: &mut BTreeSet<Album>,
        path_out: &str,
        path_write: &str,
    ) -> io::Result<()> {
        let content = fs::read_to_string(format!("{}nones.txt", path_out))?;
        for line in content.lines() {
            let mut words = line.split(';');
            let (artist, title, tracks) = (words.next(), words.next(), words.next());
            if tracks == None {
                continue;
            }
            let current = albums.get(&Album::new(
                String::from(artist.unwrap()),
                String::from(title.unwrap()),
            ));
            if current.is_none() {
                continue;
            }
            let mut updated = (*(current.as_ref().unwrap())).clone();
            updated.tracks = tracks.map(|x| x.parse().unwrap_or(0));
            updated.compute_score();
            Album::add_to_database(&updated, path_write)?;
            albums.replace(updated);
        }
        Ok(())
    }

    pub fn add_to_database(album: &Album, path: &str) -> io::Result<()> {
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(format!("{}database.txt", path))?;

        file.write_all(album.to_database_format().as_bytes())?;

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

    pub fn get_images(albums: &[&Album], path: &str) -> Vec<String> {
        let mut cover_urls: Vec<String> = Vec::new();
        let client = reqwest::Client::new();
        for album in albums.iter() {
            match &album.image {
                Some(x) => cover_urls.push(
                    download_image(&x, path, &client)
                        .unwrap_or_else(|_| format!("{}blank.png", path)),
                ),
                _ => cover_urls.push(format!("{}blank.png", path)),
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
        match self.title.cmp(&other.title) {
            Ordering::Equal => self.artist.cmp(&other.artist),
            x => x,
        }
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
            "{} - {}: {} ({})",
            self.artist,
            self.title,
            self.score.unwrap_or_else(|| Ratio::new(0, 1)),
            self.playcount
        )
    }
}

impl Hash for Album {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.title.hash(state);
        self.artist.hash(state);
    }
}

pub fn get_users(path: &str) -> Vec<String> {
    let contents = fs::read_to_string(format!("{}users.txt", path))
        .unwrap_or_else(|_| panic!("Something went wrong reading {}users.txt", path));
    contents.lines().map(String::from).collect()
}

pub fn get_key(path: &str) -> String {
    fs::read_to_string(format!("{}key.txt", path))
        .unwrap_or_else(|_| panic!("Something went wrong reading {}key.txt", path))
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

///returns false if album shouldn't be considered
pub fn is_top_and_update_top(
    album: &Album,
    top_number: usize,
    scores: &mut BinaryHeap<Ratio<i64>>,
) -> bool {
    let smallest = -scores.peek().unwrap_or(&Ratio::new(-100_000, 1));

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
pub fn albums_to_html(albums: &[&Album]) -> String {
    let mut doc = String::from(include_str!("../data/html_header"));
    for album in albums {
        doc.push_str(&album.to_html_card());
    }
    doc.push_str(include_str!("../data/html_footer"));
    doc
}
pub fn save_index_html(s: &str, path: &str) -> io::Result<()> {
    let mut file = File::create(format!("{}index.html", path))?;
    file.write_all(s.as_bytes())?;
    Ok(())
}
pub fn download_image(
    target: &str,
    path: &str,
    client: &reqwest::Client,
) -> Result<String, reqwest::Error> {
    let mut response = client.get(target).send()?;
    let result;

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");
        let fname = format!("{}images/{}", path, fname);
        let fname = fname.as_str();
        result = String::from(fname);
        File::create(fname).unwrap()
    };
    std::io::copy(&mut response, &mut dest).unwrap();
    Ok(result)
}

pub fn nones_to_file(nones: &[&Album], path: &str) -> io::Result<()> {
    let mut file = File::create(format!("{}nones.txt", path))?;
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

pub fn wants_again() -> io::Result<bool> {
    let mut answer = String::new();
    std::io::stdin().read_line(&mut answer)?;
    if answer.chars().next().unwrap_or('Y') == 'N' {
        Ok(false)
    } else {
        Ok(true)
    }
}
