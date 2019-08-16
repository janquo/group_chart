use super::*;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::sync::Arc;

pub struct APIError { }
type Sender = std::sync::mpsc::Sender<Result<(Vec<serde_json::Value>, String), (APIError, Downloader)>>;

pub struct Downloader {
    user: String,
    client: reqwest::Client,
    key: Arc<String>,
    period: Arc<String>,
    transmitter: Sender,
}

impl Downloader {
    pub fn new(user: String, key: &Arc<String>, period: &Arc<String>, transmitter: &Sender) -> Downloader {
        Downloader {
            user: user,
            client: reqwest::Client::new(),
            key: Arc::clone(key),
            period: Arc::clone(period),
            transmitter: std::sync::mpsc::Sender::clone(transmitter)
        }
    }

    pub fn get_user_data(self) -> std::thread::JoinHandle<()> {
        unimplemented!();

    }

    pub fn wait_get_user_data(self, time: u64) -> std::thread::JoinHandle<()> {
        super::sleep(time);
        self.get_user_data()
    }
}

pub fn load_database(path: &String) -> io::Result<HashSet<Album>> {
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
            best_contributor: (String::from(""), 0),
            no_contributors: 0,
        };
        if !database.insert(album) {
            eprintln!("record doubled in a database");
        }
    }
    Ok(database)
}
