extern crate chrono;
extern crate image;
extern crate imageproc;
extern crate num_rational;
extern crate regex;
extern crate reqwest;
extern crate rusttype;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate threadpool;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate derive_error;

use num_rational::Ratio;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{BTreeSet, BinaryHeap};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};

pub mod config;
pub mod database;
pub mod drawer;
pub mod lastfmapi;
pub mod reader;
pub mod spotifyapi;
pub mod webpage;

#[derive(Debug, Error)]
pub enum DownloadError {
    OutdatedUrl,
    Reqwest(reqwest::Error),
}

pub struct Args {
    pub x: u32,
    pub y: u32,
    pub period: String,
    pub captions: bool,
    pub nick: Option<String>,
    pub web: bool,
    pub path_write: PathBuf,
    pub path_read: PathBuf,
    pub path_out: PathBuf,
    pub path_web: PathBuf,
    pub save_history: bool,
}
_
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Album {
    title: String,
    artist: String,
    playcount: i64,
    tracks: Option<usize>,

    #[serde(skip)]
    score: Option<Ratio<i64>>,
    pub image: Option<String>,
    best_contributor: (String, i64),
    no_contributors: i64,
}

impl Album {
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
    pub fn apis_info(
        &mut self,
        key: &str,
        token: &str,
        client: &reqwest::Client,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        if self.score.is_none() || !self.has_cover() {
            if let Some(album) = spotifyapi::get_non_single(token, &self)? {
                self.merge_info(album);
            }
            if self.tracks.is_none() || !self.has_cover() {
                let album = lastfmapi::album_getinfo(self, &key, client)?;
                self.merge_info(album);
            }
            Ok(false)
        } else {
            Ok(true)
        }
    }

    pub fn more_info(
        &mut self,
        db: &Connection,
        key: &str,
        token: &str,
        client: &reqwest::Client,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        match database::get_album(&db, &self) {
            Ok(album) => {
                self.tracks = album.tracks;
                self.image = album.image;
                self.compute_score();
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => (),
            Err(err) => eprintln!(
                "error occurred during reading album from the database: {:?}",
                err
            ),
        }
        self.apis_info(key, token, client)
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

    pub fn merge(&mut self, other: Album) {
        self.no_contributors += other.no_contributors;
        self.playcount += other.playcount;
        if self.best_contributor.1 < other.best_contributor.1 {
            self.best_contributor = other.best_contributor;
        }
    }

    pub fn merge_info(&mut self, other: Album) {
        if let Some(x) = other.tracks {
            match self.tracks {
                Some(y) if x <= y => (),
                None | Some(_) => {
                    self.tracks = other.tracks;
                    self.compute_score();
                }
            };
        }
        if !self.has_cover() && other.has_cover() {
            self.image = other.image;
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
    pub fn to_string_semicolon(&self) -> String {
        format!("{};{};{}", self.artist, self.title, self.playcount)
    }

    pub fn get_images(albums: &[&Album], path: &Path) -> Vec<Option<PathBuf>> {
        let mut cover_paths = Vec::new();
        let client = reqwest::Client::new();
        for album in albums.iter() {
            match &album.image {
                Some(x) => cover_paths.push(download_image(&x, path, &client).ok()),
                _ => cover_paths.push(Some(path.join("blank.png"))),
            }
        }
        cover_paths
    }

    pub fn has_cover(&self) -> bool {
        match &self.image {
            None => false,
            Some(x) => !x.is_empty() && x != "blank.png",
        }
    }

    pub fn tracks(&self) -> usize {
        match &self.tracks {
            None => 0,
            Some(n) => *n,
        }
    }

    pub fn remove_descriptors_from_name(&mut self) {
        lazy_static! {
            static ref REGEX: regex::Regex = regex::Regex::new(
                r"\s?([\[\(][^\]\)]*|: |- )([Rr]emaster|[Dd]eluxe|([0-9]*th)? [Aa]nniversary)[^\[\(]*[\]\)]?\s?"
            )
            .unwrap();
        }
        #[cfg(debug_assertions)]
        {
            if REGEX.replace_all(&self.title, "") == "" {
                println!("why is this nothing? {}", self);
            }
        }
        self.title = String::from(REGEX.replace_all(&self.title, ""));
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

pub struct AlbumsUnscored {
    data: BTreeSet<Album>,
}

impl AlbumsUnscored {
    pub fn new() -> Self {
        Self {
            data: BTreeSet::new(),
        }
    }

    pub fn insert(&mut self, user_albums: Vec<Album>) {
        for mut album in user_albums.into_iter() {

            if self.data.contains(&album) {
                let mut old = self.data.take(&album).unwrap();
                old.merge(album);
                album = old;
            }

            self.data.insert(album);
        }
    }

    pub fn playcount_sorted(self) -> Vec<Album> {
        let mut res: Vec<Album> = self.data.into_iter().collect();
        res.sort_by_key(|album| -album.playcount());
        res
    }
}

/// aims to keep top `pruning size` scored albums
/// albums will only be pruned if by the time of insert it is sure it won't be in the top
pub struct AlbumsPruned {
    data: BTreeSet<Album>,
    min_scores: BinaryHeap<Ratio<i64>>, // it's max heap so it keeps negative scores
    pruning_size: usize,
}

impl AlbumsPruned {
    pub fn new(pruning_size: usize) -> Self {
        Self {
            data: BTreeSet::new(),
            min_scores: BinaryHeap::new(),
            pruning_size,
        }
    }

    /// returns false if album was pruned
    pub fn insert_pruning(&mut self, album: Album) -> bool {
        let smallest = -*(self.min_scores
            .peek()
            .unwrap_or(&Ratio::new(i64::MIN, 1)));

        if self.data.len() >= self.pruning_size && Ratio::new(album.playcount(), 3) < smallest {
            return false;
        }

        match album.score() {
            Some(score) => {
                if score < smallest && self.min_scores.len() < self.pruning_size {
                    self.min_scores.push(-score);
                    true
                } else if score >= smallest {
                    self.min_scores.push(-score);
                    if self.min_scores.len() > self.pruning_size {
                        self.min_scores.pop();
                    }
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    pub fn with_no_score(&self) -> Vec<&Album> {
        let mut top_none: Vec<&Album> = self.data.iter()
            .filter(|x| x.score().is_none())
            .collect();
        top_none.sort_by_key(|x| -x.playcount());
        top_none
    }

    pub fn with_score(self) -> Vec<Album> {
        let mut top_some: Vec<Album> = self.data.into_iter().filter(|x| x.score.is_some()).collect();
        top_some.sort_by(|x, y| y.score.unwrap().partial_cmp(&x.score.unwrap()).unwrap());
        top_some
    }

    pub fn update_tracks(&mut self, album: Album) {
        if let Some(mut old) = self.data.take(&album) {
            old.tracks = album.tracks;
            old.compute_score();
            self.data.insert(old);
        }
    }
}

pub fn albums_to_html(albums: &[Album]) -> String {
    let mut doc = String::from(include_str!("../data/html_header"));
    for album in albums {
        doc.push_str(&album.to_html_card());
    }
    doc.push_str(include_str!("../data/html_footer"));
    doc
}

pub fn download_image(
    target: &str,
    path: &Path,
    client: &reqwest::Client,
) -> Result<PathBuf, DownloadError> {
    let mut response = client.get(target).send()?;
    if !response.status().is_success() {
        return Err(DownloadError::OutdatedUrl);
    }
    let result;

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");
        let fname = path.join("images").join(fname);
        result = PathBuf::from(&fname);
        File::create(fname).unwrap()
    };
    std::io::copy(&mut response, &mut dest).unwrap();
    Ok(result)
}

pub fn nones_to_file(nones: &[&Album], path: &Path) -> io::Result<()> {
    let mut file = File::create(path.join("nones.txt"))?;
    file.write_all(
        nones
            .iter()
            .map(|x| x.to_string_semicolon())
            .collect::<Vec<String>>()
            .join("\n")
            .as_bytes(),
    )?;
    Ok(())
}

pub fn save_results(albums: &Vec<Album>, path: &Path) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(
        serde_json::to_string(albums)?.as_bytes()
    )?;
    Ok(())
}

pub fn results_from_file(path: &Path) -> io::Result<Vec<Album>> {
    let content = std::fs::read_to_string(path)?;
    let mut results : Vec<Album> = serde_json::from_str(&content)?;
    for album in results.iter_mut() {
        album.compute_score();
    }
    Ok(results)
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

#[cfg(test)]
mod tests {
    use super::Album;

    #[test]
    fn remove_descriptors_from_name_test() {
        let examples = vec![
            ("Innuendo (2011 Remaster)", "Innuendo"),
            ("Judgement (Remastered)", "Judgement"),
            ("Time And A Word [Expanded & Remastered]", "Time And A Word"),
            ("Past Masters (Vols. 1 & 2 / Remastered)", "Past Masters"),
            ("The Hybrid - Deluxe Edition", "The Hybrid"),
            ("In Utero - 20th Anniversary Super Deluxe", "In Utero"),
            (
                "Beating a Dead Horse: Deluxe Ultra-Limited Exclusive Undead Edition",
                "Beating a Dead Horse",
            ),
            ("David Live (2005 Mix, Remastered Version)", "David Live"),
            (
                "The Dark Side Of The Moon [Remastered] (Remastered Version)",
                "The Dark Side Of The Moon",
            ),
            (
                "Paul's Boutique (20th Anniversary Remastered Edition)",
                "Paul's Boutique",
            ),
        ];
        for (raw, processed) in examples.into_iter() {
            let mut album = super::Album::new(String::new(), String::from(raw));
            album.remove_descriptors_from_name();
            assert_eq!(album.title, processed);
        }
    }

    //noinspection ALL
    #[test]
    fn serialize_album() {
        let album = Album {
            title: String::from("Black Celebration"),
            artist: String::from("Depeche Mode"),
            playcount: 57,
            tracks: Some(22),
            score: None,
            image: Some(String::from("blank.png")),
            best_contributor: (String::from("janquo"), 43),
            no_contributors: 2,
        };

        assert_eq!(
            &serde_json::from_str::<Album>(&serde_json::to_string(&album).unwrap()).unwrap(),
            &album
        );
    }
}
