

use group_chart::communicator::*;
use group_chart::drawer;
use group_chart::reader;
use std::collections::HashMap;
use std::{thread, time};

fn main() {
    let top_number = 25;

    let users = reader::get_users();
    let users: Vec<&str> = users.lines().collect();

    let key = reader::get_key();

    let mut global_chart: HashMap<(String, String), i64> = HashMap::new();

    for user in users {
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

        for (name, artist, count) in user_albums.iter().map(|x| parse_album(x)) {
            eprintln!("adding {} to counter of album {} by user {}", count, name, user);
            let count_glob = global_chart.entry((name, artist)).or_insert(0);
            *count_glob += count;
        }
        let half_second = time::Duration::from_millis(500);
        thread::sleep(half_second);
    }

    let mut global_chart_vec: Vec<((String, String), i64)> = Vec::new();

    for album in global_chart {
        global_chart_vec.push(album);
    }

    global_chart_vec.sort_by_key(|(_, count)| -(*count));
    global_chart_vec.truncate(top_number);

    let mut cover_urls : Vec<String> = Vec::new();
    for album in global_chart_vec {
        println!("{} - {}: {}", (album.0).1, (album.0).0, album.1);
        let data = get_album_inf(&(album.0).0, &(album.0).1, &key).unwrap();
        match get_img_url(&data) {
            Ok(x) => cover_urls.push(download_image(&x).unwrap_or(String::from("blank.png"))),
            _ => cover_urls.push(String::from("blank.png")),
        }
    }
    drawer::collage(cover_urls);
}
