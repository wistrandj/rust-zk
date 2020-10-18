use std::path::{PathBuf, Path};
use std::env;
use rusqlite::{Connection, params};

pub struct Config {
    pub card: bool,
    pub default_location: PathBuf,
    pub folder: PathBuf,  // Todo(wistrandj): delete
    pub project: PathBuf, // Todo(wistrandj): delete
}

pub fn arguments() -> Config {
    let args: Vec<String> = env::args().collect();
    let card: bool;

    if args.len() > 1 && args[1] == "card" {
        card = true;
    } else {
        card = false;
    }

    // @Here
    // let folder = PathBuf::from("/Users/wistrandj/timeline/zk");
    // let project = PathBuf::from("/Users/wistrandj/timeline/zk");

    let folder = PathBuf::from("/tmp/tx/zk");
    let project = PathBuf::from("/tmp/tx/zk.db");
    let default_location = PathBuf::from("");

    Config {
        card,
        folder,
        project,
        default_location,
    }
}

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

pub fn set_default_location(conn: &mut Connection, default_location: PathBuf) {
    let mut config = load_configuration(conn).unwrap();
    config.default_location = default_location;
    save_configuration(conn, &config);
}

pub fn save_configuration(conn: &mut Connection, config: &Config) {
    let mut stmt1 = conn.prepare("
        delete from configuration;
    ").unwrap();
    let mut stmt2 = conn.prepare("
        insert into configuration(version, default_location) values (?1, ?2);
    ").unwrap();

    stmt1.query(params![]).unwrap();
    stmt2.query(params![1, config.default_location.to_str()]).unwrap();
}

pub fn load_configuration(conn: &mut Connection) -> Option<Config> {
    let mut stmt = conn.prepare("
        select version, default_location from configuration;
    ").unwrap();

    let config = stmt.query_row(params![], |row| {
        let default_location: String = row.get(0)?;
        Ok(Config {
            card: false,
            default_location: PathBuf::from(default_location),
            folder: PathBuf::from(""),
            project: PathBuf::from(""),
        })
    } );

    return Some(config.unwrap());
}

