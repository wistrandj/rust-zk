use std::path::PathBuf;
use std::env;
use rusqlite::{params};

mod varg;
mod config;

// Command line tool to create a new timeline database. Usage:
//
//   $ zk-init --timeline ./path/timeline.zk
//
// The file name can be anything.

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let timeline_file = varg::get_timeline(&mut args);

    if let Some(location) = timeline_file {
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
}


