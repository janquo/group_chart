use super::Album;
use rusqlite::{params, Connection};
use std::path::PathBuf;

pub fn create_albums_table(path: PathBuf) -> rusqlite::Result<()> {
    let conn = Connection::open(path)?;

    conn.execute(
        "CREATE TABLE albums (
            artist TEXT PRIMARY KEY,
            title TEXT PRIMARY KEY,
            tracks INTEGER,
            image INTEGER
        ) IF NOT EXISTS",
        params![],
    )?;
    Ok(())
}

pub fn connect(path: PathBuf) -> rusqlite::Result<Connection> {
    Connection::open(path)
}

pub fn add(conn: &Connection, album: &Album) -> rusqlite::Result<usize> {
    conn.execute(
        "INSERT INTO albums (artist, title, tracks, image)
            VALUES (?1, ?2, ?3)",
        params![
            album.artist,
            album.title,
            album.tracks.map(|x| x as i32),
            album.image
        ],
    )
}

#[cfg(test)]
mod tests {}
