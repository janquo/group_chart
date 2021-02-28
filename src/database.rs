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

pub fn create_history_tables(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tygodniowa_dates (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date TEXT NOT NULL
        );",
        params![],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tygodniowa (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date INTEGER NOT NULL,
            place INTEGER NOT NULL,
            artist TEXT NOT NULL,
            title TEXT NOT NULL,
            scrobbles INTEGER NOT NULL,
            CONSTRAINT fk_artist_title FOREIGN KEY (artist, title) REFERENCES albums (artist, title),
            CONSTRAINT fk_date FOREIGN KEY (date) REFERENCES tygodniowa_dates (id)
        );",
        params![],
    )?;
    Ok(())
}

pub fn save_results(conn: &Connection, albums: &[Album]) -> rusqlite::Result<()> {
    let date = chrono::Local::now().to_rfc3339();
    let mut stmt_date = conn.prepare(
        "INSERT INTO tygodniowa_dates (date)
            VALUES (?1);",
    )?;
    let mut stmt = conn.prepare_cached(
        "INSERT INTO tygodniowa (date, place, artist, title, scrobbles)
            VALUES (?1,?2,?3,?4,?5);",
    )?;
    let date_id = stmt_date.insert(params![date])?;
    for (i, album) in albums.iter().enumerate() {
        stmt.execute(params![
            date_id,
            i as isize,
            album.artist,
            album.title,
            album.playcount
        ])?;
    }
    Ok(())
}

pub fn connect(path: &Path) -> rusqlite::Result<Connection> {
    Connection::open(path)
}

pub fn update_album(conn: &Connection, album: &Album) -> rusqlite::Result<usize> {
    let mut stmt = conn.prepare_cached(
        "INSERT OR IGNORE INTO albums (artist, title)
            VALUES (?1, ?2);",
    )?;
    stmt.execute(params![album.artist, album.title,])?;
    let mut stmt = conn.prepare_cached(
        "UPDATE albums SET tracks=?1 AND image=?2
            WHERE artist=?3 AND title=?4;",
    )?;
    stmt.execute(params![
        album.tracks.map(|x| x as i32),
        album.image,
        album.artist,
        album.title,
    ])
}

pub fn erase_image(conn: &Connection, album: &Album) -> rusqlite::Result<usize> {
    let mut stmt = conn.prepare_cached(
        "INSERT OR IGNORE INTO albums (artist, title)
            VALUES (?1, ?2);",
    )?;
    stmt.execute(params![album.artist, album.title,])?;
    let mut stmt = conn.prepare_cached(
        "UPDATE albums SET image=?1
            WHERE artist=?2 AND title=?3;",
    )?;
    stmt.execute(params!["", album.artist, album.title,])
}

pub fn get_album(conn: &Connection, album: &Album) -> rusqlite::Result<Album> {
    let mut stmt = conn.prepare_cached(
        "SELECT tracks, image FROM albums
                WHERE artist=?1 AND title=?2;",
    )?;

    stmt.query_row(params![album.artist, album.title], |row| {
        Ok(Album {
            tracks: row.get::<usize, Option<i32>>(0)?.map(|x| x as usize),
            image: row.get(1)?,
            ..album.clone()
        })
    })
}

pub fn get_album_history(
    conn: &Connection,
    album: &Album,
) -> rusqlite::Result<Vec<(usize, usize, usize)>> {
    let mut result: Vec<(usize, usize, usize)> = vec![];
    let mut stmt = conn.prepare_cached(
        "SELECT date, place, scrobbles FROM tygodniowa
            WHERE artist=?1 AND title=?2;",
    )?;

    let data = stmt.query_map(params![album.artist, album.title], |row| {
        Ok((
            row.get::<usize, i32>(0)? as usize,
            row.get::<usize, i32>(1)? as usize,
            row.get::<usize, i32>(2)? as usize,
        ))
    })?;
    for entry in data {
        result.push(entry?);
    }
    Ok(result)
}
