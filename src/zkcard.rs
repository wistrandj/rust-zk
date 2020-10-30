// Usage:
//   $ zk-card --timeline ./timeline.zk card

use std::path::PathBuf;
use rusqlite::params;
use std::env;

mod varg;
mod file;
mod config;
mod folder;

#[derive(Debug)]
struct A {
    default_location: String
}


fn main() {
    let mut args: Vec<String> = env::args().collect();
    let timeline_file = varg::get_timeline(&mut args);

    if let Some(timeline_file) = timeline_file {
        if args.len() < 2 {
            eprintln!("Too little arguments.");
        }

        if !timeline_file.is_file() {
            eprintln!("The given timeline is not a file");
            return;
        }

        let card_switch: &String = args.get(1).unwrap();

        if card_switch == "card" {
            let timeline = config::open_timeline(&timeline_file).unwrap();
            let mut query_location = timeline.prepare("select default_location from configuration;").unwrap();
            let location = query_location.query_row(params![], |row| {
                Ok(A {
                    default_location: row.get(0)?
                })
            });
            let location = PathBuf::from(location.unwrap().default_location);
            if !location.is_dir() {
                eprintln!("The location for open cards {} is missing", location.to_str().unwrap());
                return;
            }

            let next_card = folder::next_major_card(&location);
            println!("Open a new card in {}", next_card.to_str().unwrap());

            file::make_template(&next_card);
            file::edit(&next_card);
        }
    }
}
