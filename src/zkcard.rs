use std::path::PathBuf;
use rusqlite::params;
use std::env;

mod file;
mod config;
mod folder;

#[derive(Debug)]
struct A {
    default_location: String
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Too little arguments.");
    }

    let timeline_switch: &String = args.get(1).unwrap();
    let timeline: &String = args.get(2).unwrap();
    let card_switch: &String = args.get(3).unwrap();

    if timeline_switch != "--timeline" {
        eprintln!("The first argument must be --timeline followed by the path");
        return;
    }

    let timeline = PathBuf::from(timeline);
    if !timeline.is_file() {
        eprintln!("The timeline timeline is not in timeline format!");
        return;
    }

    if card_switch == "card" {
        let timeline = config::open_timeline(&timeline).unwrap();
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
