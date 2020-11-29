// Manage the content of cards from the viewpoint of
// sqlite3 timeline or opencards.

use rusqlite::{Connection, params};
use crate::card::{Timestamp, Meta};
use crate::card::Face;
use std::collections::HashSet;
use std::path::Path;
use std::fs;
use crate::hash;

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

