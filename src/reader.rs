use super::*;
use std::fs;
use std::io;
use std::sync::Arc;
use serde_json::Value;
use futures::stream::futures_unordered::FuturesUnordered;

pub struct Downloader {
    user: String,
    client: reqwest::Client,
    key: Arc<String>,
    period: Period,
    pub result: Result<Value, reqwest::Error>,
    n_tries: usize,
}

impl Downloader {
    pub fn new(
        user: String,
        key: &Arc<String>,
        period: Period,
    ) -> Downloader {
        Downloader {
            user,
            client: reqwest::Client::new(),
            key: Arc::clone(key),
            period,
            result: Ok(Value::Null),
            n_tries: 0,
        }
    }

    pub async fn get_chart(mut self) -> Self {
        self.n_tries += 1;
        self.result = lastfmapi::get_chart(&self.user, &self.key, self.period, &self.client).await;
        self
    }

    pub fn user(&self) -> &str {
        &self.user
    }
    pub fn n_tries(&self) -> usize {
        self.n_tries
    }
}

pub fn run_get_chart_for_all_users(
    args: &Args,
    key: &Arc<String>,
) -> FuturesUnordered<async_std::task::JoinHandle<Downloader>> {
    let users = args.load_users();

    users.into_iter()
        .map(|user| reader::Downloader::new(user, key, args.period))
        .map(Downloader::get_chart)
        .map(async_std::task::spawn)
        .collect()
}

pub fn tracks_from_file(
    albums: &mut BTreeSet<Album>,
    path_out: &Path,
    db: &Connection,
) -> io::Result<()> {
    let content = fs::read_to_string(path_out.join("nones.txt"))?;
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
        database::update_album(&db, &updated).unwrap();
        albums.replace(updated);
    }
    Ok(())
}

pub fn get_users(path: &std::path::Path) -> Vec<String> {
    let users_path = path.join("users.txt");
    let contents = fs::read_to_string(&users_path)
        .unwrap_or_else(|_| panic!("Something went wrong reading {}", users_path.display()));
    contents.lines().map(String::from).collect()
}
