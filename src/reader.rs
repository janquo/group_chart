use std::io;
use std::collections::HashSet;
use std::fs;
use super::*;

pub fn load_database(path : &String) -> io::Result<HashSet<Album>> {
    let mut database: HashSet<Album> = HashSet::with_capacity(15000);

    let content = fs::read_to_string(format!("{}database.txt", path))?;
    for line in content.lines() {
        let mut words = line.split(";");
        let (artist, title, tracks, image) =
            (words.next(), words.next(), words.next(), words.next());
        if artist == None || title == None {
            continue;
        }
        let album = Album {
            title: String::from(title.unwrap()),
            artist: String::from(artist.unwrap()),
            playcount: 0,
            tracks: tracks.unwrap().parse().ok(),
            score: None,
            image: image.map(String::from),
            best_contributor: (String::from(""), 0),
            no_contributors: 0,
        };
        if !database.insert(album) {
            eprintln!("record doubled in a database");
        }
    }
    Ok(database)
}
