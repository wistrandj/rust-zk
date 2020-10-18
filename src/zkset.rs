use std::env;
use std::path::PathBuf;
use rusqlite::{params};
mod config;

#[derive(Debug)]
struct A {
    version: u32,
    default_location: String
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        eprintln!("Invalid number of arguments. Need four.");
        return;
    }

    let timeline_switch: &String = args.get(1).unwrap();
    let location_argument: &String = args.get(2).unwrap();
    let location = PathBuf::from(location_argument);
    let setting_name: &String = args.get(3).unwrap();
    let setting_value: &String = args.get(4).unwrap();

    if timeline_switch != "--timeline" {
        eprintln!("Timeline switch (--timeline) not given as first argument");
        return;
    }

    if !location.is_file() {
        eprintln!("Timeline {} is missing", location_argument);
        return;
    }

    if setting_name == "location" {
        let default_location = PathBuf::from(setting_value).canonicalize().unwrap();
        if !default_location.is_dir() {
            eprintln!("The default location {} is not a directory", setting_value);
            return;
        }

        let timeline = config::open_timeline(&location).unwrap();
        timeline.execute("
            update configuration set default_location = ?1;
        ", params![default_location.to_str().unwrap()]);
        eprintln!("OK, set to {:?}", default_location.to_str());

        let mut stmt = timeline.prepare("select version, default_location from configuration;").unwrap();
        let row = stmt.query_row(params![], |row| {
            Ok(A {
                version: row.get(0)?,
                default_location: row.get(1)?
            })
        });

        let row = row.unwrap();
        println!("Version: {}\nDefault location: {}", row.version, row.default_location);
    }

}

