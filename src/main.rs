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

    let no_users = users.len();
    let mut progress = 0;

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
            //eprintln!("adding {} to counter of album {} by user {}", count, name, user);
            let count_glob = global_chart.entry((name, artist)).or_insert(0);
            *count_glob += count;
        }
        let quarter_second = time::Duration::from_millis(250);
        progress += 1;
        println!("{}/{}", progress, no_users);
        thread::sleep(quarter_second);
    }

    let mut global_chart_vec: Vec<((String, String), i64)> = Vec::new();

    for album in global_chart {
        global_chart_vec.push(album);
    }

    global_chart_vec.sort_by_key(|(_, count)| -(*count));
    //global_chart_vec.truncate(top_number);

    let mut cover_urls: Vec<String> = Vec::new();
    let mut min_so_far = 1000f64;
    let mut data_vec = Vec::new();
    for album in global_chart_vec.iter() {
        let data = get_album_inf(&(album.0).0, &(album.0).1, &key).unwrap();
        let tracks_no = get_album_tracks_no(&data);
        if tracks_no == 0 {
            eprintln!("zero tracków na {}", (album.0).0);
            continue;
        }
        let count_waged = (album.1 as f64) / (tracks_no as f64);
        if min_so_far > count_waged {
            min_so_far = count_waged
        } else if data_vec.len() >= top_number {
            break;
        }
        data_vec.push((data.clone(), count_waged));
    }
    let mut chart_zipped: Vec<(&((String, String), i64), &(serde_json::Value, f64))> =
        global_chart_vec.iter().zip(data_vec.iter()).collect();

    chart_zipped.sort_by(|(_, (_, x)), (_, (_, y))| y.partial_cmp(x).unwrap());
    chart_zipped.truncate(top_number);

    for (album, (data, points)) in chart_zipped {
        println!("{} - {}: {}", (album.0).1, (album.0).0, album.1);

        match get_img_url(&data) {
            Ok(x) => cover_urls.push(download_image(&x).unwrap_or(String::from("blank.png"))),
            _ => cover_urls.push(String::from("blank.png")),
        }
    }
    drawer::collage(cover_urls);
}
