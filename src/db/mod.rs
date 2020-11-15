pub mod dbcard;
pub mod dbconfig;
pub mod blob;


use std::path::Path;
use rusqlite::Connection;

pub fn open_new_timeline(file: &Path) -> Option<Connection> {
    return Some(Connection::open(file).unwrap());
}

pub fn open_timeline(file: &Path) -> Option<Connection> {
    // Safety(wistrandj): Ensure that the file does not exists.
    if !file.is_file() {
        eprintln!("The file {} is not a timeline", file.to_str().unwrap());
        return None;
    }

    let sqlite_connection = Connection::open(file);

    if let Ok(sqlite_connection) = sqlite_connection {
        return Some(sqlite_connection);
    } else {
        return None;
    }
}

