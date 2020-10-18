use std::path::{PathBuf, Path};
use std::env;

pub struct Config {
    pub card: bool,
    pub folder: PathBuf,    // Folder for open cards
    pub project: PathBuf,   // Path to the ZK configuration file, which is a Sqlite3 database
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

    Config {
        card,
        folder,
        project
    }
}

