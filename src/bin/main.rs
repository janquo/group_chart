use config::ConfigErr;
use group_chart::*;
use num_rational::Ratio;
use std::collections::{BTreeSet, BinaryHeap, HashSet};
use std::sync::mpsc;
use std::sync::Arc;

fn main() {
    let args = config::load_args();
    let config = config::load();
    let args = match config.parse(args) {
        Ok(x) => x,
        Err(ConfigErr::NoArgument) => panic!("-x, -y, -p, -s have to be followed by a value"),
        Err(ConfigErr::U32ParseError) => panic!("use positive integers as collage dimensions"),
        Err(ConfigErr::WrongOption) => panic!("available args: -x, -y, -p, -c, -w, -s"),
        Err(ConfigErr::WrongPeriod) => {
            panic!("available periods: overall | 7day | 1month | 3month | 6month | 12month")
        }
    };

    let collage_size = args.size();

    let mut albums: BTreeSet<Album> = BTreeSet::new();

    let client = reqwest::Client::new();

    let (transmitter, receiver) = mpsc::channel();

    let key = args.get_key();
    let key = Arc::new(key);

    let threadpool = reader::run_get_chart_for_all_users(&args, &key, transmitter);

    let mut progress = 0;
    for (data, command) in receiver {
        let user = command.get_user();
        let user_data = match data {
            Err(x) => {
                eprintln!(
                    "Couldn't aquire data for user {} because of {}\n trying again in a second...",
                    user, x
                );
                threadpool.execute(move || command.wait_get_chart(1000));
                continue;
            }
            Ok(x) => x,
        };
        if user_data["error"] != serde_json::Value::Null {
            let error_code = user_data["error"].as_i64().unwrap();
            eprintln!("Error code {} while reading user {}", error_code, user);
            if error_code == 29 || (error_code == 8 && command.try_number < 5) {
                eprintln!("waiting...");
                threadpool.execute(move || command.wait_get_chart(1000));
            } else {
                eprintln!("escaping");
                progress += 1;
                println!("{} users processed", progress);
            }
            continue;
        }

        let user_albums = match user_data["topalbums"]["album"].as_array() {
            None => {
                eprintln!("User {} has no scrobbles. Continue to next user.", user);
                return;
            }
            Some(x) => x,
        };

        let mut user_albums = user_albums
            .iter()
            .map(|x| lastfmapi::parse_album(x, String::from(user)))
            .collect::<Vec<Album>>();
        for album in user_albums.iter_mut() {
            album.remove_descriptors_from_name();
        }
        Album::insert(&mut albums, user_albums);

        progress += 1;
        println!("{} users processed", progress);
    }

    threadpool.join();

    let albums = Album::rev_sorted_vec(albums);

    let mut top_albums = BTreeSet::new();
    let mut negative_scores_max_heap: BinaryHeap<Ratio<i64>> = BinaryHeap::new();

    let database: HashSet<Album> = match reader::load_database(&args.path_write) {
        Err(x) => {
            eprintln!("couldn't read database.txt file with error {}", x);
            HashSet::with_capacity(0)
        }
        Ok(x) => x,
    };

    let albums_no = albums.len();

    let (id, secret) = args.get_spotify_auth();

    let token = match spotifyapi::get_access_token(&id, &secret) {
        Ok(token) => token,
        Err(err) => {
            eprintln!("Couldn't acquire spotify auth token {}", err);
            String::from("")
        }
    };

    for (i, mut album) in albums.into_iter().enumerate() {
        eprintln!("{}/{}", i, albums_no);

        loop {
            match album.more_info(&database, &key, &token, &client) {
                Ok(x) => {
                    if !x {
                        if let Err(e) = Album::add_to_database(&album, &args.path_write) {
                            eprintln!(
                                "There was an error during appending new record to database: {}",
                                e
                            )
                        }
                    }
                    break;
                }
                Err(x) => {
                    eprintln!("{} while reading {}", x.to_string(), album);
                    if format!("{}", album).contains('#') {
                        break;
                    }
                    sleep(1000);
                }
            }
        }

        let smallest = -negative_scores_max_heap
            .peek()
            .unwrap_or(&Ratio::new(-100_000, 1));

        //some prunning
        if top_albums.len() >= collage_size && Ratio::new(album.playcount(), 3) < smallest {
            break;
        }

        //if a score belongs to top or there is no score then insert
        if is_top_and_update_top(&album, collage_size, &mut negative_scores_max_heap) {
            top_albums.insert(album);
        }
    }

    let top_none = Album::with_no_score(&top_albums);

    while nones_to_file(&top_none, &args.path_out).is_err() {
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

    println!("update nones file and enter anything to proceed");
    std::io::stdin().read_line(&mut String::new()).unwrap_or(0);

    while Album::tracks_from_file(&mut top_albums, &args.path_out, &args.path_write).is_err() {
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
        .take(collage_size)
        .collect();

    let cover_urls = Album::get_images(&top, &args.path_write);

    top.iter_mut().fold((), |_, x| println!("{}", x));

    if args.web {
        let s = albums_to_html(&top);
        if save_index_html(&s, &args.path_web).is_err() {
            eprint!("{}", s);
        }
    }

    drawer::collage(cover_urls, top, args);
}
