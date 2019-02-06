use group_chart::*;
use num_rational::Ratio;
use std::collections::{BTreeSet, BinaryHeap};

fn main() {
    let x_images = 6;
    let y_images = 6;
    let period = "7day"; //overall | 7day | 1month | 3month | 6month | 12month
    let top_number =  (x_images * y_images) as usize;

    let users = get_users();
    let users: Vec<&str> = users.lines().collect();

    let key = get_key();

    let mut albums: BTreeSet<Album> = BTreeSet::new();


    let no_users = users.len();

    for (progress, user) in users.iter().enumerate() {
        let user_data: serde_json::Value = loop {
            let user_data1 = get_chart(user, &key, period);

            let user_data1 = match user_data1 {
                Err(x) => {
                    eprintln!(
                        "Couldn't aquire data for user {} with error {:?}\n trying again in a second...",
                        user, x
                    );
                    sleep(1000);
                    continue;
                }
                Ok(x) => x,
            };

            if user_data1["error"] == serde_json::Value::Null {
                break user_data1;
            } else {
                let error_code = user_data1["error"].as_i64().unwrap();
                eprintln!("Error code {} while reading user {}", error_code, user);
                if error_code == 29 {
                    eprintln!("waiting...");
                    sleep(2000);
                } else {
                    eprintln!("escaping");
                    break serde_json::Value::Null;
                }
            }
        };

        let user_albums = match user_data["topalbums"]["album"].as_array() {
            None => {
                eprintln!("User {} has no scrobbles. Continue to next user.", user);
                continue;
            }
            Some(x) => x,
        };

        Album::insert(&mut albums, user_albums);

        println!("{}/{}", progress + 1, no_users);
        sleep(125); // don't overuse server
    }

    //descending sorted Vec
    let albums = Album::sorted_vec(albums);

    let mut top_albums = BTreeSet::new();
    let mut scores: BinaryHeap<Ratio<i64>> = BinaryHeap::new(); // max_heap of negative scores

    let albums_no = albums.len();
    for (i, mut album) in albums.into_iter().enumerate() {
        eprintln!("{}/{}", i, albums_no);
        sleep(125);

        loop {
            match album.more_info(&key) {
                Ok(_) => break,
                Err(x) => {
                    eprintln!("error {:?} while reading {}", x, album);
                    sleep(1000);
                }
            }
        }

        let smallest = -scores.peek().unwrap_or(&Ratio::new(-100000, 1));

        //some prunning
        if top_albums.len() >= top_number && Ratio::new(album.get_count(), 2) < smallest {
                break;
        }

        //if a score belongs to top or there is no score then insert
        if top_scores_update(&album, top_number, &mut scores) {
            top_albums.insert(album);
        }
    }
    match Album::tracks_from_file(&mut top_albums) {
        Err(x) => eprintln!(
            "{:?}\nerror ocurred during reading manual_tracks.txt, proceeding...",
            x
        ),
        _ => (),
    }

    let top_none = Album::with_no_score(&top_albums);

    while nones_to_file(&top_none).is_err() {
        eprintln!("error ocurred durring attempt to write albums without score to a file\n Do you want to try again? Y/N");
        match wants_again() {
            Err(x) => eprintln!("error: {}\n trying again...", x),
            Ok(x) => {
                if !x {
                    break;
                }
            }
        }
    }

    println!("update manual tracks file and enter anything to proceed");
    std::io::stdin().read_line(&mut String::new()).unwrap_or(0);

    while Album::tracks_from_file(&mut top_albums).is_err() {
        eprintln!("error ocurred durring attempt to read scores from file\n Do you want to try again? Y/N");
        match wants_again() {
            Err(x) => eprintln!("error: {}\n trying again...", x),
            Ok(x) => {
                if !x {
                    break;
                }
            }
        }
    }

    let mut top: Vec<&Album> = Album::with_score(&top_albums)
        .into_iter()
        .take(top_number)
        .collect();

    let cover_urls = Album::get_images(&top);
    top.iter_mut().fold((), |_, x| println!("{}", x));

    drawer::collage(cover_urls, x_images, y_images);
}
