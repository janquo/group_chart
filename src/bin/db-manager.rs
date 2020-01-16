use group_chart::*;

fn main() {
    let args = config::load_args();
    let config = config::load();
    let args = config.parse(args).unwrap();
    let key = args.get_key();
    let mut database = reader::load_database(&args.path_write).unwrap();

    let (spotify_id, spotify_secret) = args.get_spotify_auth();

    let auth = spotifyapi::get_access_token(&spotify_id, &spotify_secret).unwrap();

    let incomplete = database
        .iter()
        .filter(|x| x.tracks() <= 2 || !x.has_cover())
        .cloned();

    let mut without_duplicates = std::collections::HashSet::<Album>::new();

    for mut album in incomplete.into_iter() {
        if let Some(other) = without_duplicates.take(&album) {
            album.merge_info(other);
        }
        without_duplicates.insert(album);
    }

    let client = reqwest::Client::new();

    for mut album in without_duplicates.into_iter() {
        if let Err(e) = album.apis_info(&key, &auth, &client) {
            println!("couldn't repair an album because of {}", e.to_string());
        }
        else {
            database.replace(album).unwrap();
        }
    }

    for album in database {
        Album::add_to_database(&album, &format!("{}new_", &args.path_write)).unwrap();
    }
}
