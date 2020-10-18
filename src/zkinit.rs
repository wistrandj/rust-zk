use std::path::PathBuf;
use std::env;
use rusqlite::{params};
mod config;

// Command line tool to create a new timeline database. Usage:
//
//   $ zk-init --timeline ./path/timeline.zk
//
// The file name can be anything.

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments. Give the config file location");
    }

    let timeline_switch: &String = args.get(1).unwrap();
    let location_argument: &String = args.get(2).unwrap();

    if timeline_switch != "--timeline" {
        eprintln!("The second argument must be --timeline");
        return;
    }

    let location = PathBuf::from(location_argument);

    if location.is_file() || location.is_dir() {
        panic!("The timeline {} exists", location_argument);
    }

    let timeline = config::open_timeline(&location);
    if let Some(mut timeline) = timeline {
        config::setup1(&mut timeline);
        timeline.execute("
            insert into configuration(version, default_location) values (?1, ?2);
        ", params![1, "/tmp"]);
    }
}

