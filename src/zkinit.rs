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

    // if location.is_file() || location.is_dir() {
    //     eprintln!("The timeline {} exists", location_argument);
    //     return;
    // }

    let timeline = config::open_new_timeline(&location);
    if let Some(mut timeline) = timeline {
        config::setup1(&mut timeline);
        let success = timeline.execute("
            insert into configuration(version, default_location) values (?1, ?2);
        ", params![1, "/tmp"]);

        if let Err(_) = success {
            eprintln!("Failed to set default location to the initial value (/tmp)", );
            return;
        }

        // Make sure that features are updated too
        config::setup2(&mut timeline);
    }

}


