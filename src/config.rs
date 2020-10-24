use std::path::Path;
use rusqlite::{Connection, params};

pub fn open_timeline(file: &Path) -> Option<Connection> {
    return Some(Connection::open(file).unwrap());
}

pub fn setup1(conn: &mut Connection) {
    // Safety(wistrandj): Ensure that the file does not exists.

    conn.execute("
        create table configuration (
            version integer,
            default_location text
        );
    ", params![]).unwrap();

    // Q(wistrandj): How to commit change to the database file?
}

