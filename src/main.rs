use std::collections::HashMap;
use group_chart::communicator;
use group_chart::drawer;
use group_chart::reader;

fn main() {
    let users = reader::get_users();
    let users : Vec<&str> = users.lines().collect();

    let key = reader::get_key();

    let mut global_chart : HashMap<String, i64> = HashMap::new();

    for user in users {
        let user_data = communicator::get_chart(user, &key);

        match user_data {
            Err(x) => panic!(x),
            _ => (),
        }
        let user_data = user_data.unwrap();

        if !user_data["weeklyalbumchart"]["album"].is_array() {panic!("no array")};

        let user_albums : &Vec<serde_json::Value> = user_data["weeklyalbumchart"]["album"].as_array().unwrap();
        for (name, count) in user_albums.iter().map(|x| communicator::parse_album(x)) {
            let count_glob = global_chart.entry(name).or_insert(0);
            *count_glob += count;
        }
    }

    let mut global_chart_vec : Vec<(String, i64)> = Vec::new();
    for album in global_chart {
        global_chart_vec.push(album);
    }

    global_chart_vec.sort_by_key(|(_, count)| -(*count));
    global_chart_vec.truncate(25);
    for album in global_chart_vec {
        println!("{} {}", album.0, album.1);
    }

}
