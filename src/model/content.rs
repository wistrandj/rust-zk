// Manage the content of cards from the viewpoint of
// sqlite3 timeline or opencards.

use rusqlite::{Connection, params};
use crate::card::{Timestamp, Meta};
use crate::card::Face;
use std::collections::HashSet;
use std::path::Path;
use std::fs;
use crate::hash;

pub fn save_blob(conn: &Connection, file: &Path) {
    // Todo(wistrandj): Read content and calculate hash at the same time.
    let hash = hash::Hash::file(file).unwrap().to_string();
    let content_available = conn.query_row(
        "select count(*) from content where content_sha256 = ?1;",
        params![hash],
        |row| {
            let count: u32 = row.get(0)?;
            Ok(count)
        }
    );


    if let Ok(count_rows) = content_available {
        if count_rows == 0 {
            let blob = fs::read(file).unwrap();
            let success = conn.execute(
                "insert into content(content_sha256, blob) values (?1, ?2);",
                params![hash, blob],
            );

            if let Err(msg) = success {
                panic!("Fail to save blob content to timeline file. Reason: {}", msg);
            }
        }
    } else {
        panic!("Content table missing");
    }
}

fn load_blob(conn: &Connection, file: &Path, sha256: hash::Hash) -> Option<Vec<u8>> {
    let file_exists = file.is_file() || file.is_dir();
    if file_exists {
        panic!("The file exists already");
    }

    let blob = conn.query_row(
        "select blob from content where content_sha256 = ?1",
        params![sha256.to_string()],
        |row| {
            let blob: Vec<u8> = row.get(0)?;
            Ok(blob)
        }
    );

    if let Err(_) = blob {
        return None;
    }

    return Some(blob.unwrap());
}

fn all_timeline_cards(conn: &Connection) -> Vec<Meta> {
    return vec![];
}

fn all_open_cards(folder: &Path) -> Vec<Meta> {
    return vec![];
}

fn timestamp_of_file(file: &Path) -> Timestamp {
    Timestamp { }
}


fn modified_open_cards(folder: &Path, conn: &Connection) -> Vec<Meta> {
    // Note(wistrandj): I am not very proud of this piece of code.
    let opencards: Vec<Meta> = all_open_cards(folder);
    let timelinecards: Vec<Meta> = all_timeline_cards(conn);

    // Error(wistrandj): Expect a named lifetime parameter.
    // type Name = &str;
    // type Content = &str;

    let mut opencards_set: HashSet<(String, String)> = HashSet::new();
    let mut timelinecards_set: HashSet<(String, String)> = HashSet::new();

    for card in opencards {
        let face = card.face.name();
        let sha256 = String::from(card.content_sha256);
        opencards_set.insert((face, sha256));
    }

    for card in timelinecards {
        let face = card.face.name();
        let sha256 = String::from(card.content_sha256);
        timelinecards_set.insert((face, sha256));
    }

    let modified_or_new = opencards_set.difference(&timelinecards_set);
    let mut modified = Vec::new();

    for card in modified_or_new {
        modified.push(Meta {
            face: Face::from_name(&card.0).unwrap(),
            create_time: Timestamp { },
            modify_time: Timestamp { },
            content_sha256: card.1.clone(),
            commit_user: String::new(),
            commit_email: String::new(),
        });
    }

    return modified;
}

