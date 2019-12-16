use group_chart::*;

fn main() {
    let args = config::load_args();
    let config = config::load();
    let args = config.parse(args).unwrap();
    let _key = args.get_key();
    let mut database = reader::load_database(&args.path_write).unwrap();

    let (spotify_id, spotify_secret) = args.get_spotify_auth();

    let auth = spotifyapi::get_access_token(&spotify_id, &spotify_secret).unwrap();

    let _singles: Vec<Album> = database
        .iter()
        .filter(|x| x.tracks() <= 2)
        .cloned()
        .collect();
    let without_cover: Vec<Album> = database
        .iter()
        .filter(|x| !x.has_cover())
        .cloned()
        .collect();
    println!("hah {}", without_cover.len());
    for mut album in without_cover.into_iter() {
        if album.tracks() <= 2 {
            continue;
        }
        spotifyapi::search_album(&auth, &album, 2);
        println!("provide cover for {}", album);
        let mut answer = String::new();
        std::io::stdin().read_line(&mut answer).unwrap();
        album.image = Some(answer);
        database.replace(album);
    }

    for album in database {
        Album::add_to_database(&album, &format!("{}new_", &args.path_write)).unwrap();
    }
}
