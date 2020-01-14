use group_chart::*;

fn main() {
    let args = config::load_args();
    let config = config::load();
    let args = config.parse(args).unwrap();
    let key = args.get_key();
    let mut database = reader::load_database(&args.path_write).unwrap();

    let (spotify_id, spotify_secret) = args.get_spotify_auth();

    let auth = spotifyapi::get_access_token(&spotify_id, &spotify_secret).unwrap();

    let singles: Vec<Album> = database
        .iter()
        .filter(|x| x.tracks() <= 2)
        .cloned()
        .collect();
    for mut album in singles.into_iter() {
        let propositions = spotifyapi::search_album(&auth, &album, 2).unwrap();
        if !propositions.is_empty() {
            if propositions[0].tracks() > 1 {
                album = propositions[0].clone();
            } else if propositions.len() > 1 && propositions[1].tracks() > 1 {
                album = propositions[1].clone();
            }
        }
        database.replace(album);
    }
    let without_cover: Vec<Album> = database
        .iter()
        .filter(|x| !x.has_cover())
        .cloned()
        .collect();
    println!("hah {}", without_cover.len());

    let client = reqwest::Client::new();
    for mut album in without_cover.into_iter() {
        if album.tracks() <= 2 {
            continue;
        }
        let spotify_album = spotifyapi::search_album(&auth, &album, 1);
        if let Some(candidate) = spotify_album.unwrap().pop() {
            album = candidate;
        } else {
            let _ = lastfmapi::album_getinfo(&album, &key, &client);
        }
        database.replace(album);
    }

    for album in database {
        Album::add_to_database(&album, &format!("{}new_", &args.path_write)).unwrap();
    }
}
