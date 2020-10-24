use std::path::Path;
use rusqlite::{Connection, params};

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

pub fn setup1(conn: &mut Connection) {
    let mut stmt = conn.prepare("
        select count(*) from sqlite_master where type = 'table' and name = 'configuration';
    ").unwrap();
    let setup1_done = stmt.query_row(params![], |row| {
        let x: u32 = row.get(0)?;
        Ok(x)
    }).unwrap();

    if setup1_done < 1 {
        conn.execute("
            create table configuration (
                version integer,
                default_location text
            );
        ", params![]).unwrap();
    } else {
        eprintln!("The timeline is already created");
    }

    // Q(wistrandj): How to commit change to the database file?
}

pub fn setup2(conn: &mut Connection) {
    let count = conn.query_row(
        "select count(*) from sqlite_master where type = 'table' and name = 'content'",
        params![],
        |row| {
            let count: u32 = row.get(0)?;
            Ok(count)
        }
    ).unwrap();

    if count >= 1 {
        return;
    }

    // Todo(wistrandj): Document the database schema somewhere. Blob inside this
    // table can be null, if the contents have been truncated.
    conn.execute_batch(
        "
        begin;
        create table content(
            content_sha256 text not null,
            blob blob
        );
        create table card(
            card_name text not null, -- e.g. '123', '123a5'
            content_sha256 text not null
        );
        commit;
        "
    ).unwrap();
}

