use super::Album;
use rusqlite::{params, Connection};
use std::path::Path;

pub fn create_albums_table(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS albums (
            artist TEXT,
            title TEXT,
            tracks INTEGER,
            image INTEGER,
            CONSTRAINT pk_albums PRIMARY KEY (artist, title)
        );",
        params![],
    )?;
    Ok(())
}

pub fn connect(path: &Path) -> rusqlite::Result<Connection> {
    Connection::open(path)
}

pub fn update_album(conn: &Connection, album: &Album) -> rusqlite::Result<usize> {
    conn.execute(
        "INSERT OR IGNORE INTO albums (artist, title)
            VALUES (?1, ?2);
        UPDATE albums SET tracks=?3 AND image=?4
            WHERE artist=?1 AND title=?2;",
        params![
            album.artist,
            album.title,
            album.tracks.map(|x| x as i32),
            album.image
        ],
    )
}

pub fn get_album(conn: &Connection, album: &Album) -> rusqlite::Result<Album> {
    let mut stmt = conn
        .prepare(
            "SELECT tracks, image FROM albums
                WHERE artist=?1 AND title=?2",
        )
        .unwrap();

    stmt.query_row(params![album.artist, album.title], |row| {
        Ok(Album {
            tracks: row.get::<usize, Option<i32>>(0)?.map(|x| x as usize),
            image: row.get(1)?,
            ..album.clone()
        })
    })
}
