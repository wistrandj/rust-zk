use std::path::PathBuf;

use super::config;
use super::db::dbconfig;

#[derive(Debug)]
struct A {
    version: u32,
    default_location: String
}

pub fn zkset(timeline_file: &PathBuf, args: &Vec<String>) {
    if args.len() < 2 {
        eprintln!("No setting or value given");
    }

    let setting = args.get(0).unwrap();
    let value = args.get(1).unwrap();

    if setting == "location" {
        let default_location = PathBuf::from(value).canonicalize().unwrap();
        if !default_location.is_dir() {
            eprintln!("The default location {} is not a directory", value);
            return;
        }

        let timeline = config::open_timeline(&timeline_file).unwrap();
        dbconfig::set_default_location(&timeline, &default_location);

        let default_location = dbconfig::default_location(&timeline).unwrap();
        let version = dbconfig::version(&timeline).unwrap();
        println!("Version: {}", version);
        println!("Location for open cards: {}", default_location.to_string_lossy());
    }
}

