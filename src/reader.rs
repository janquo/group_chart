use super::*;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::sync::Arc;
use threadpool::ThreadPool;

type Sender = std::sync::mpsc::Sender<(Result<serde_json::Value, reqwest::Error>, Downloader)>;

pub struct Downloader {
    user: String,
    client: reqwest::Client,
    key: Arc<String>,
    period: Arc<String>,
    transmitter: Sender,
    pub try_number: usize,
}

impl Downloader {
    pub fn new(
        user: String,
        key: &Arc<String>,
        period: &Arc<String>,
        transmitter: &Sender,
    ) -> Downloader {
        Downloader {
            user,
            client: reqwest::Client::new(),
            key: Arc::clone(key),
            period: Arc::clone(period),
            transmitter: std::sync::mpsc::Sender::clone(transmitter),
            try_number: 0,
        }
    }

    pub fn delegate_get_chart(mut self) {
        self.try_number += 1;
        let chart = lastfmapi::get_chart(&self.user, &self.key, &self.period, &self.client);
        std::sync::mpsc::Sender::clone(&self.transmitter)
            .send((chart, self))
            .unwrap();
    }

    pub fn wait_get_chart(self, time: u64) {
        super::sleep(time);
        self.delegate_get_chart();
    }

    pub fn get_user(&self) -> &str {
        &self.user
    }
}

pub fn run_get_chart_for_all_users(
    args: &Args,
    key: &Arc<String>,
    transmitter: Sender,
) -> ThreadPool {
    let pool = ThreadPool::new(15); // no idea what number of threads is right

    let users = args.load_users();

    let period = Arc::new(args.period.clone());

    for user in users.into_iter() {
        let download_command = reader::Downloader::new(user, key, &period, &transmitter);
        pool.execute(move || download_command.delegate_get_chart());
    }
    pool
}

pub fn load_database(path: &str) -> io::Result<HashSet<Album>> {
    let mut database: HashSet<Album> = HashSet::with_capacity(15000);

    let content = fs::read_to_string(format!("{}database.txt", path))?;
    for line in content.lines() {
        let album: Album = serde_json::from_str(line)?;
        if !database.insert(album) {
            eprintln!("record doubled in a database");
        }
    }
    Ok(database)
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

pub fn get_users(path: &str) -> Vec<String> {
    let contents = fs::read_to_string(format!("{}users.txt", path))
        .unwrap_or_else(|_| panic!("Something went wrong reading {}users.txt", path));
    contents.lines().map(String::from).collect()
}
