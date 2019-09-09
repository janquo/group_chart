use super::*;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::sync::Arc;

type Sender = std::sync::mpsc::Sender<(Result<serde_json::Value, reqwest::Error>, Downloader)>;

pub struct Downloader {
    user: String,
    client: reqwest::Client,
    key: Arc<String>,
    period: Arc<String>,
    transmitter: Sender,
}

impl Downloader {
    pub fn new(
        user: String,
        key: &Arc<String>,
        period: &Arc<String>,
        transmitter: &Sender,
    ) -> Downloader {
        Downloader {
            user: user,
            client: reqwest::Client::new(),
            key: Arc::clone(key),
            period: Arc::clone(period),
            transmitter: std::sync::mpsc::Sender::clone(transmitter),
        }
    }

    pub fn delegate_get_chart(self) -> std::thread::JoinHandle<()> {
        std::thread::spawn(move || {
            let chart = get_chart(&self.user, &self.key, &self.period, &self.client);
            std::sync::mpsc::Sender::clone(&self.transmitter)
                .send((chart, self))
                .unwrap();
        })
    }

    pub fn wait_get_chart(self, time: u64) -> std::thread::JoinHandle<()> {
        super::sleep(time);
        self.delegate_get_chart()
    }

    pub fn get_user(&self) -> &str {
        &self.user
    }
}

pub fn run_get_char_for_all_users(
    args: &Args,
    key: &Arc<String>,
    transmitter: Sender,
) -> Vec<std::thread::JoinHandle<()>> {
    let mut handles = vec![];

    let users = args.load_users();

    let period = Arc::new(args.period.clone());

    for user in users.into_iter() {
        let download_command = reader::Downloader::new(user, key, &period, &transmitter);
        handles.push(download_command.delegate_get_chart());
    }
    handles
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
