use std::path::PathBuf;
use rusqlite::{params};

use super::config;
use super::schema;

// Command line tool to create a new timeline database. Usage:
//   $ zk-init --timeline ./path/timeline.zk

pub fn zkinit(timeline_file: &PathBuf) {
    let timeline = config::open_new_timeline(timeline_file);
    if let Some(mut timeline) = timeline {
        schema::install_missing_features(&mut timeline);

        // Note(wistrandj): Set default location to a dummy value. Expect the user to change it
        // immediately
        let success = timeline.execute("
            insert into configuration(version, default_location) values (?1, ?2);
        ", params![1, "/tmp"]);

        if let Err(_) = success {
            eprintln!("Failed to set default location to the initial value (/tmp)", );
            return;
        }

        let success = timeline.close();
        if let Ok(_) = success {
            eprintln!("Succesfully closed the timeline database");
        } else if let Err(result) = success {
            let (_conn, msg) = result;
            eprintln!("Failed to close the timeline database. Reason: {}", msg);
            return;
        }
    }
}

