use rusqlite::{Connection, params};
use crate::card::Face;
use std::path::Path;
use std::fs;
use crate::hash;

pub fn save(conn: &Connection, file: &Path) -> hash::Hash {
    // Todo(wistrandj): Read content and calculate hash at the same time.
    let hash = hash::Hash::file(file).unwrap();
    let hash_str = hash.to_string();
    let content_available = conn.query_row(
        "select count(*) from content where content_sha256 = ?1;",
        params![hash_str],
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
                params![hash_str, blob],
            );

            if let Err(msg) = success {
                panic!("Fail to save blob content to timeline file. Reason: {}", msg);
            }
        }
    } else {
        panic!("Content table missing");
    }

    return hash;
}

pub fn load(conn: &Connection, sha256: hash::Hash) -> Option<Vec<u8>> {
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
