use group_chart::*;
use num_rational::Ratio;
use std::collections::BTreeSet;
use std::{thread, time};

fn main() {
    let x_collage = 6; let y_collage = 6;
    let top_number = (x_collage * y_collage) as usize;

    let users = get_users();
    let users: Vec<&str> = users.lines().collect();

    let key = get_key();

    let mut albums: BTreeSet<Album> = BTreeSet::new();

    let no_users = users.len();

    for (progress, user) in users.iter().enumerate() {
        let mut user_data = serde_json::Value::Null;
        while true {
            let mut user_data1 = get_chart(user, &key, "7days");

            match user_data1 {
                Err(x) => {
                    eprintln!("Error reading user {}. Continue to next user. {:?}", user, x);
                    continue;
                }
                _ => (),
            }

            let mut user_data1 = user_data1.unwrap();

            if user_data1["error"] == serde_json::Value::Null {
                user_data = user_data1.clone();
                break;
            }
            else {
                let error_code = user_data1["error"].as_i64().unwrap();
                eprintln!("Error code {} while reading user {}", error_code, user);
                if error_code == 29 {
                    eprintln!("waiting...");
                    thread::sleep(time::Duration::from_millis(2000));
                }
                else {
                    eprintln!("escaping");
                    break;
                }
            }
        }

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
    let mut top_albums = BTreeSet::new();
    let mut scores: BTreeSet<Ratio<i64>> = BTreeSet::new(); //BTreeSet doesnt hold multiple copies of the same score, however, it is not so bad

    let albums_no = albums.len();
    for (i, mut album) in albums.into_iter().enumerate() {
        eprintln!("{}/{}", i, albums_no);
        thread::sleep(time::Duration::from_millis(125));
        album.more_info(&key);
        let smallest = scores
            .iter()
            .next()
            .cloned()
            .unwrap_or(Ratio::new(100000, 1));
        if top_albums.len() >= top_number {
            if Ratio::new(album.get_count(), 2) < smallest {
                break;
            }
        }
        match album.get_score() {
            Some(score) => {
                if score < smallest && scores.len() < top_number {
                    scores.insert(score);
                } else if score >= smallest {
                    scores.insert(score);
                    if scores.len() > top_number {
                        scores = scores.into_iter().rev().take(top_number).collect();
                    }
                } else {
                    continue;
                }
            }
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

    nones_to_file(&(top_none.collect()));

    println!("update manual tracks file and enter anything to proceed");
    std::io::stdin().read_line(&mut String::new());

    Album::tracks_from_file(&mut top_albums);

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
    drawer::collage(cover_urls, x_collage, y_collage);
}
