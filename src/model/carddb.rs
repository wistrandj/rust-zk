use crate::card::Face;
use crate::hash;
use rusqlite::{Connection, params};
use std::path::{PathBuf, Path};

fn cards(conn: &mut Connection) -> Vec<Face> {
    let mut stmt = conn.prepare("select card_name from card;").unwrap();
    let cards = stmt.query_map(
        params![],
        |row| {
            let name: String = row.get(0)?;
            let card = Face::from_name(name.as_str()).unwrap();
            Ok(card)
        });
    let cards: Vec<Face> = cards
        .unwrap()
        .map(|maybe_card| maybe_card.unwrap())
        .collect();
    return cards;
}


pub fn save_card_and_hash(conn: &Connection, card: &Face, hash: &hash::Hash) {
    let mut stmt = conn.prepare("insert or replace into card(card_name, content_sha256) values (?1, ?2);").unwrap();
    let name: String = card.name();
    let hash: String = hash.to_string();
    let success = stmt.execute(params![name, hash]);
    if let Err(msg) = success {
        eprintln!("Fail to save card and the hash");
    }
}

pub fn default_location(conn: &Connection) -> Option<PathBuf> {
    let row = conn.query_row(
        "select default_location from configuration;",
        params![],
        |row| {
            let path: String = row.get(0)?;
            Ok(PathBuf::from(path))
        }
    );

    return match row {
        Ok(path) => Some(path),
        Err(msg) => {
            eprintln!("Fail to get the default location. Reason: {}", msg);
            None
        }
    }
}

pub fn set_default_location(conn: &Connection, location: &Path) {
    let success = conn.execute(
        "update configuration set default_location = ?1;",
        params![location.to_str()]
    );

    if let Err(msg) = success {
        eprintln!("Fail to set default location. Reason: {}", msg);
    }
}

pub fn version(conn: &Connection) -> Option<usize> {
    let row = conn.query_row(
        "select version from configuration;",
        params![],
        |row| {
            let version: i32 = row.get(0)?;
            let version: usize = version as usize;
            // let version: usize = version.parse().unwrap();
            Ok(version)
        }
    );

    return match row {
        Ok(version) => Some(version),
        Err(msg) => {
            eprintln!("Fail to get the version. Reason: {}", msg);
            None
        }
    }
}
