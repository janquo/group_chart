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
use std::collections::{BTreeSet, BinaryHeap, HashSet};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Write;

pub mod drawer;

#[derive(Clone)]
pub struct Album {
    title: String,
    artist: String,
    playcount: i64,
    tracks: Option<usize>,
    score: Option<Ratio<i64>>,
    pub image: Option<String>,
    mbid: Option<String>,
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
            mbid: data["mbid"].as_str().map(|s| String::from(s)),
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
            mbid: None,
            best_contributor: (String::from("NaN"), 0),
            no_contributors: 0,
        }
    }
    pub fn more_info(
        &mut self,
        database: &HashSet<Album>,
        key: &str,
    ) -> Result<bool, reqwest::Error> {
        /* let request_url = if self.mbid.is_none()
            || self.mbid.clone().unwrap_or(String::new()).is_empty() == true
        {
            format!("http://ws.audioscrobbler.com/2.0/?method=album.getinfo&api_key={}&artist={}&album={}&format=json",
                              key, self.artist.replace("&", "%26"), self.title.replace("&", "%26"))
        } else {
            format!("http://ws.audioscrobbler.com/2.0/?method=album.getinfo&api_key={}&mbid={}&format=json",
                                                    key, self.mbid.clone().unwrap())
        };*/
        if let Some(album) = database.get(&self) {
            self.tracks = album.tracks;
            self.image = album.image.clone();

            self.compute_score();

            Ok(true)
        } else {
            let request_url = format!("http://ws.audioscrobbler.com/2.0/?method=album.getinfo&api_key={}&artist={}&album={}&format=json",
                                      key, self.artist.replace("&", "%26"), self.title.replace("&", "%26"));

            let mut response = reqwest::get(&request_url)?;

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
                Some(x) => x[3]["#text"].as_str().map(|s| String::from(s)),
            };
            self.compute_score();

            Ok(false)
        }
    }

    fn compute_score(&mut self) {
        if self.tracks.is_none() || self.tracks == Some(1) || self.tracks == Some(0) {
            return;
        }
        self.score = Some(Ratio::new(self.playcount, self.tracks.unwrap() as i64));
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

    pub fn insert(albums: &mut BTreeSet<Album>, user_albums: &Vec<Value>, user: &str) {
        for album in user_albums
            .iter()
            .map(|x| Album::parse_album(x, String::from(user)))
        {
            //eprintln!("adding {} to counter of album {} by user {}", count, name, user);
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
            r#"<div class="card mb-3">
                <div class="row no-gutters">
                <div class="col-md-4">
                <img alt="alt" class="card-img" src="{}">
                </div>
                <div class="col-md-8">
                <div class="card-body">
                <h5 class="card-title">{} - {}</h5>
                <p class="card-text">Scrobbles: {}</p>
                <p class="card-text">Score: {:.2}</p>
                <p class="card-text">Contributors number: {}</p>
                <p class="card-text">Top: {} - {} scrobbles</p>
                </div>
                </div>
                </div>
                </div>"#,
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
    pub fn tracks_from_file(albums: &mut BTreeSet<Album>, path_out: &String, path_write: &String) -> Result<(), Box<dyn Error>> {
        let content = fs::read_to_string(format!("{}nones.txt", path_out))?;
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
            let mut updated = (*(current.as_ref().unwrap())).clone();
            updated.tracks = tracks.map(|x| x.parse().unwrap_or(0));
            updated.compute_score();
            Album::add_to_database(&updated, path_write)?;
            albums.replace(updated);
        }
        Ok(())
    }

    pub fn load_database(path : &String) -> Result<HashSet<Album>, Box<dyn Error>> {
        let mut database: HashSet<Album> = HashSet::with_capacity(15000);

        let content = fs::read_to_string(format!("{}database.txt", path))?;
        for line in content.lines() {
            let mut words = line.split(";");
            let (artist, title, tracks, image) =
                (words.next(), words.next(), words.next(), words.next());
            if artist == None || title == None {
                continue;
            }
            let album = Album {
                title: String::from(title.unwrap()),
                artist: String::from(artist.unwrap()),
                playcount: 0,
                tracks: tracks.unwrap().parse().ok(),
                score: None,
                image: image.map(String::from),
                mbid: None,
                best_contributor: (String::from(""), 0),
                no_contributors: 0,
            };
            if !database.insert(album) {
                eprintln!("record doubled in a database {} - {}", artist.unwrap(), title.unwrap());
            }
        }
        Ok(database)
    }

    pub fn add_to_database(album: &Album, path : &String) -> std::io::Result<()> {
        if album.tracks != None && album.tracks != Some(0) {
            let mut file = std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(format!("{}database.txt", path))?;

            file.write_all(album.to_database_format().as_bytes())?;
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

    pub fn get_images(albums: &Vec<&Album>, path: &String) -> Vec<String> {
        let mut cover_urls: Vec<String> = Vec::new();
        for album in albums.iter() {
            match &album.image {
                Some(x) => {
                    cover_urls.push(download_image(&x, path).unwrap_or(format!("{}blank.png", path)))
                }
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
            "{} - {}: {} ({})",
            self.artist,
            self.title,
            self.score.unwrap_or(Ratio::new(0, 1)),
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
impl Args {
    fn new() -> Args {
        Args{ x: 5u32, y: 5u32, period: String::from("7day"), captions: false, nick: None, web: false, path_read: String::from("./data/"), path_write: String::from("./data/"), path_out: String::from(""), path_web: String::from("./docs/")}
    }
}
pub fn load_config() -> Args {
    let mut args = Args::new();
    let lines = fs::read_to_string("config.ini")
        .expect("Something went wrong reading the config.ini file");
    let lines = lines.lines();
    for line in lines.into_iter() {
        let mut words = line.split("=");
        let key = words.next().unwrap();
        let value = words.next().unwrap();
        match key {
            "x" => args.x = value.parse().unwrap(),
            "y" => args.y = value.parse().unwrap(),
            "period" => args.period = String::from(value),
            "captions" => args.captions = value.parse().unwrap(),
            "web" => args.web = value.parse().unwrap(),
            "user" => args.nick = if value == "" {None} else {Some(String::from(value))},
            "read_path" => args.path_read = String::from(value),
            "write_path" => args.path_write  = String::from(value),
            "out_path" => args.path_out = String::from(value),
            "web_path" => args.path_web = String::from(value),
            _ => panic!("check your config file"),
        }
    }
    args
}

pub fn parse_args(args: Vec<String>, mut res: Args) -> Result<Args, i32> {
    let mut args = args.into_iter();
    args.next();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-x" => res.x = args.next().ok_or(1)?.parse().ok().ok_or(2)?,
            "-y" => res.y = args.next().ok_or(1)?.parse().ok().ok_or(2)?,
            "-p" => res.period = args.next().ok_or(1)?,
            "-c" => res.captions = true,
            "-w" => res.web = true,
            "-s" => res.nick = Some(args.next().ok_or(1)?),
            _ => return Err(3),
        }
    }
    if !vec!["7day", "1month", "3month", "6month", "1year", "overall"].contains(&res.period.as_str()) {
        return Err(4);
    }
    Ok(res)
}

pub fn get_users(path: &String) -> Vec<String> {
    let contents = fs::read_to_string(format!("{}users.txt", path))
        .expect(&format!("Something went wrong reading {}users.txt", path));
    contents.lines().map(String::from).collect()
}

pub fn get_key(path: &String) -> String {
    let contents = fs::read_to_string(format!("{}key.txt", path))
        .expect(&format!("Something went wrong reading {}key.txt", path));
    contents
}

pub fn get_chart(user: &str, key: &str, period: &str) -> Result<Value, reqwest::Error> {
    let request_url = format!("http://ws.audioscrobbler.com/2.0/?method=user.gettopalbums&user={}&api_key={}&period={}&limit=1000&format=json",
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
pub fn albums_to_html(albums: &Vec<&Album>) -> String {
    let mut doc = String::from(
    r##"<!doctype html>
    <html lang="en">
    <head>
        <!-- Required meta tags -->
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=no">

        <!-- Bootstrap CSS -->
        <link rel="stylesheet" href="https://stackpath.bootstrapcdn.com/bootstrap/4.3.1/css/bootstrap.min.css" integrity="sha384-ggOyR0iXCbMQv3Xipma34MD+dH/1fQ784/j6cY/iJTQUOhcWr7x9JvoRxT2MZw1T" crossorigin="anonymous">

        <title>#tygodniowa</title>
    </head>
    <body>
    <div class="card-columns">"##);
    for album in albums {
        doc.push_str(&album.to_html_card());
    }
    doc.push_str(r##"</div>

    <!-- Optional JavaScript -->
    <!-- jQuery first, then Popper.js, then Bootstrap JS -->
    <script crossorigin="anonymous" integrity="sha384-q8i/X+965DzO0rT7abK41JStQIAqVgRVzpbzo5smXKp4YfRvH+8abtTE1Pi6jizo" src="https://code.jquery.com/jquery-3.3.1.slim.min.js"></script>
    <script crossorigin="anonymous" integrity="sha384-UO2eT0CpHqdSJQ6hJty5KVphtPhzWj9WO1clHTMGa3JDZwrnQq4sF86dIHNDz0W1" src="https://cdnjs.cloudflare.com/ajax/libs/popper.js/1.14.7/umd/popper.min.js"></script>
    <script crossorigin="anonymous" integrity="sha384-JjSmVgyd0p3pXB1rRibZUAYoIIy6OrQ6VrjIEaFf/nJGzIxFDsf4x0xIM+B07jRM" src="https://stackpath.bootstrapcdn.com/bootstrap/4.3.1/js/bootstrap.min.js"></script>
    </body>
    </html>"##);
    doc
}
pub fn save_index_html(s: &String, path: &String) -> std::io::Result<()> {
    let mut file = File::create(format!("{}index.html", path))?;
    file.write_all(s.as_bytes())?;
    Ok(())
}
pub fn download_image(target: &str, path: &String) -> Result<String, reqwest::Error> {
    let mut response = reqwest::get(target)?;
    let mut result;

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

pub fn nones_to_file(nones: &Vec<&Album>, path: &String) -> Result<(), std::io::Error> {
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

pub fn wants_again() -> Result<bool, std::io::Error> {
    let mut answer = String::new();
    std::io::stdin().read_line(&mut answer)?;
    if answer.chars().next().unwrap_or('Y') == 'N' {
        Ok(false)
    } else {
        Ok(true)
    }
}
