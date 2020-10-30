use std::path::PathBuf;
use rusqlite::{params};

use super::config;

// Command line tool to create a new timeline database. Usage:
//   $ zk-init --timeline ./path/timeline.zk

pub fn zkinit(timeline_file: PathBuf, _args: &Vec<String>) {
    let timeline = config::open_new_timeline(&timeline_file);
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

