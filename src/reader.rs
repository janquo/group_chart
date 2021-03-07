use super::*;
use std::fs;
use std::io;
use std::sync::Arc;
use serde_json::Value;
use std::error::Error;
use std::time::Duration;
use futures::stream::futures_unordered::FuturesUnordered;

pub struct Downloader {
    user: String,
    client: reqwest::Client,
    key: Arc<String>,
    period: Period,
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
        }
    }

    async fn get_chart(&self) -> Result<Value, Box<dyn Error>> {
        lastfmapi::get_chart(&self.user, &self.key, self.period, &self.client).await
    }

    /// tries to get lastfm data for the user repetitively
    /// in case of error json response received, it sleeps **blocking** because if API calls rate is
    ///     exceeded, no other tasks should be performed
    pub async fn repeat_get_chart(self) -> Option<Value> {
        let mut try_number : i32 = 0;
        loop {
            try_number += 1;

            let data = self.get_chart().await;

            let user_data = match data {
                Err(x) => {
                    eprintln!(
                        "Couldn't acquire data for user {} because of {}\n trying again in a second...",
                        self.user.as_str(), x
                    );
                    //async_std::task::sleep(Duration::from_secs(1)).await;
                    super::sleep(1000);
                    continue;
                }
                Ok(x) => x,
            };

            if let Some(error_code) = lastfmapi::error_code(&user_data) {
                eprintln!("Error code {} while reading user {}", error_code, self.user.as_str());
                if error_code == 29 || (error_code == 8 && try_number < 5) {
                    eprintln!("waiting...");
                    super::sleep(1000);
                } else {
                    eprintln!("escaping");
                    return None;
                }
                continue;
            }

            return Some(user_data);
        }
    }

    pub fn get_user(&self) -> &str {
        &self.user
    }
}

pub fn run_get_chart_for_all_users(
    args: &Args,
    key: &Arc<String>,
) -> FuturesUnordered<async_std::task::JoinHandle<Option<Value>>> {
    let users = args.load_users();

    users.into_iter()
        .map(|user| reader::Downloader::new(user, key, args.period))
        .map(reader::Downloader::repeat_get_chart)
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
