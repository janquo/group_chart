use config::ConfigErr;
use group_chart::*;
use std::sync::mpsc;
use std::sync::Arc;

fn main() {

    // ##############
    // ----config----

    let raw_args = config::load_args();
    let default_config = config::load();
    let args = match default_config.parse(raw_args) {
        Ok(x) => x,
        Err(ConfigErr::NoArgument) => panic!("-x, -y, -p, -s have to be followed by a value"),
        Err(ConfigErr::U32ParseError) => panic!("use positive integers as collage dimensions"),
        Err(ConfigErr::WrongOption) => panic!("available args: -x, -y, -p, -c, -w, -s"),
        Err(ConfigErr::WrongPeriod) => {
            panic!("available periods: overall | 7day | 1month | 3month | 6month | 12month")
        }
    };

    let collage_size = args.size();

    let mut albums = AlbumsUnscored::new();

    let client = reqwest::Client::new();

    let (transmitter, receiver) = mpsc::channel();

    let key = args.get_key();
    let key = Arc::new(key);

    // ###############################
    // ----getting users scrobbles----

    let threadpool = reader::run_get_chart_for_all_users(&args, &key, transmitter);

    let mut progress = 0;
    for (data, command) in receiver {
        let user = command.get_user();
        let user_data = match data {
            Err(x) => {
                eprintln!(
                    "Couldn't acquire data for user {} because of {}\n trying again in a second...",
                    user, x
                );
                threadpool.execute(move || command.wait_get_chart(1000));
                continue;
            }
            Ok(x) => x,
        };
        if let Some(error_code) = lastfmapi::error_code(&user_data) {
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

        let user_albums = user_albums
            .iter()
            .map(|x| lastfmapi::parse_album(x, String::from(user)))
            .map(|mut x| {
                x.remove_descriptors_from_name();
                x
            })
            .collect::<Vec<Album>>();
        albums.insert(user_albums);

        progress += 1;
        println!("{} users processed", progress);
    }

    threadpool.join();

    // ###########################################################
    // ----getting additional info for albums and choosing top----

    let albums = albums.playcount_sorted();

    let mut top_albums = AlbumsPruned::new(collage_size);

    let db = loop {
        match database::connect(&args.path_write.join("LOLZ.sqlite3")) {
            Err(x) => eprintln!("Error while opening database: {}", x),
            Ok(x) => break x,
        }
    };

    while let Err(x) = database::create_albums_table(&db) {
        eprintln!("Some error during creation albums database table: {}", x);
    }

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
            match album.more_info(&db, &key, &token, &client) {
                Ok(x) => {
                    if !x {
                        if let Err(e) = database::update_album(&db, &album) {
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

        let _ = top_albums.insert_pruning(album);
    }

    // ##########################################
    // ----getting missing info from the user----

    let top_none = top_albums.with_no_score();

    while nones_to_file(&top_none, &args.path_out).is_err() {
        println!("error occurred during attempt to write albums without score to a file\n Do you want to try again? Y/N");
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

    loop {
        match reader::tracks_from_file(&args.path_out) {
            Ok(albums) => {
                albums.into_iter().for_each(|x| {
                    if let Err(e) = database::update_tracks(&db, &x) {
                        eprintln!("error {} while updating tracks in database", e);
                    }
                    top_albums.update_tracks(x);
                });
                break;
            }
            Err(e) => {
                eprintln!("{}", e);
                println!("error occurred during attempt to read scores from file\n Do you want to try again? Y/N");
                match wants_again() {
                    Err(x) => eprintln!("error: {}\n trying again...", x),
                    Ok(x) => {
                        if !x {
                            break;
                        }
                    }
                }
            }
        }
    }

    // ###############################################
    // ----downloading images and preparing output----

    let results = top_albums.with_score();

    if let Err(e) = crate::save_results(&results, &args.path_write.join("results.json")) {
        eprintln!("couldn't save results into a file: {}", e);
    }

    let mut top: Vec<Album> = results
        .into_iter()
        .take(collage_size)
        .collect();

    let mut cover_paths = vec![args.placeholder_img(); collage_size];
    for (album, image_path) in top.iter_mut().zip(cover_paths.iter_mut()) {

        if album.image.is_none() {
            continue;
        }

        let cover_url = album.image.as_ref().unwrap();

        match download_image(cover_url, &args.path_write, &client) {
            Ok(path) => *image_path = path,
            Err(err) => match err {
                DownloadError::OutdatedUrl => {
                    let _ = database::erase_image(&db, &album);
                    match album.more_info(&db, &key, &token, &client) {
                        Ok(x) => {
                            if !x {
                                if let Err(e) = database::update_album(&db, &album) {
                                    eprintln!(
                                                "There was an error during appending new record to database: {}",
                                                e
                                            )
                                }
                            }
                            if let Some(img) = album.image.as_ref() {
                                match download_image(img, &args.path_write, &client) {
                                    Ok(path) => *image_path = path,
                                    Err(err) => {
                                        eprintln!(
                                            "Error {} during downloading image for {}",
                                            err.to_string(),
                                            album
                                        );
                                    }
                                }
                            }
                        }
                        Err(x) => {
                            eprintln!("{} while reading {}", x.to_string(), album);
                        }
                    }
                }
                DownloadError::Reqwest(e) => {
                    eprintln!(
                        "Error {} during downloading image for {}",
                        e.to_string(),
                        album
                    );
                }
            },
        }
    }

    top.iter_mut().fold((), |_, x| println!("{}", x));

    if args.web {
        let s = webpage::albums_to_html(&top, args.save_history);
        if webpage::save_index_html(&s, &args.path_web).is_err() {
            eprint!("{}", s);
        }
    }

    if args.save_history {
        database::create_history_tables(&db).unwrap();
        database::save_results(&db, &top).unwrap();

        let mut history_data = vec![];
        for album in top.iter() {
            history_data.push(database::get_album_history(&db, album).unwrap());
        }

        let script = webpage::charts_js(history_data);
        webpage::save_charts_script(&script, &args.path_web).unwrap();
    }
    drawer::collage(cover_paths, top, args);
}
