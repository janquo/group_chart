use super::*;
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

pub fn tracks_from_file(
    path_out: &Path,
) -> io::Result<Vec<Album>> {
    let content = fs::read_to_string(path_out.join("nones.txt"))?;

    let mut res = vec![];

    for line in content.lines() {
        let mut words = line.split(';');

        let (artist, title, tracks) = (words.next(), words.next(), words.next());

        if tracks == None {
            continue;
        }

        let mut album = Album::new(
            String::from(artist.unwrap()),
            String::from(title.unwrap()),
        );

        album.tracks = tracks.map(|x| x.parse().unwrap_or(0));

        res.push(album);
    }

    Ok(res)
}

pub fn get_users(path: &std::path::Path) -> Vec<String> {
    let users_path = path.join("users.txt");
    let contents = fs::read_to_string(&users_path)
        .unwrap_or_else(|_| panic!("Something went wrong reading {}", users_path.display()));
    contents.lines().map(String::from).collect()
}
