use group_chart::*;
use std::collections::BTreeSet;
use std::{thread, time};

fn main() {
    let top_number = 25;

    let users = get_users();
    let users: Vec<&str> = users.lines().collect();

    let key = get_key();

    let mut albums: BTreeSet<Album> = BTreeSet::new();

    let no_users = users.len();

    for (progress, user) in users.iter().enumerate() {
        let user_data = get_chart(user, &key);

        match user_data {
            Err(x) => {
                eprintln!("Error reading user {}. Continue to next user. {}", user, x);
                continue;
            }
            _ => (),
        }

        let user_data = user_data.unwrap();

        if !user_data["topalbums"]["album"].is_array() {
            eprintln!("User {} has no scrobbles. Continue to next user.", user);
            continue;
        };

        let user_albums: &Vec<serde_json::Value> =
            user_data["topalbums"]["album"].as_array().unwrap();

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

        println!("{}/{}", progress + 1, no_users);
        thread::sleep(time::Duration::from_millis(125)); // don't overuse server
    }

    let mut albums: Vec<Album> = albums.into_iter().collect();

    albums.sort_by_key(|album| -album.get_count());

    let mut cover_urls: Vec<String> = Vec::new();
    let mut min_so_far = 100000f64;
    let mut top_albums = BTreeSet::new();

    for mut album in albums.into_iter() {
        eprintln!("{}", top_albums.len());
        album.more_info(&key);
        if (album.get_count() as f64 / 2f64) < min_so_far && top_albums.len() >= top_number {
            break;
        }
        match album.get_score() {
            Some(score) => min_so_far = f64::min(min_so_far, score),
            None => (),
        }
        top_albums.insert(album);
    }
    Album::tracks_from_file(&mut top_albums);

    let mut top_none: Vec<&Album> = top_albums
        .iter()
        .filter(|x| x.get_score().is_none())
        .collect();
    top_none.sort_by_key(|x| -x.get_count());
    let top_none = top_none.drain(..);
    
    eprintln!("none {}", top_none.len());
    let mut top_some: Vec<&Album> = top_albums
        .iter()
        .filter(|x| x.get_score().is_some())
        .collect();
    top_some.sort_by(|x, y| {
        y.get_score()
            .unwrap()
            .partial_cmp(&x.get_score().unwrap())
            .unwrap()
    });
    let mut top_some = top_some.drain(0..top_number);
    eprintln!("some {}", top_some.len());

    for _ in 0..top_number {
        let album = top_some.next().unwrap();

        println!("{}", album);

        match &album.image {
            Some(x) => cover_urls.push(download_image(&x).unwrap_or(String::from("blank.png"))),
            _ => cover_urls.push(String::from("blank.png")),
        }
    }
    drawer::collage(cover_urls);

    nones_to_file(&(top_none.collect()));
}
