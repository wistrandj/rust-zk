use std::path::PathBuf;
use rusqlite::params;

use super::config;

#[derive(Debug)]
struct A {
    version: u32,
    default_location: String
}

pub fn zkset(timeline_file: PathBuf, args: &Vec<String>) {
    if args.len() < 2 {
        eprintln!("No setting or value given");
    }

    println!("{:?}", args);

    let setting = args.get(0).unwrap();
    let value = args.get(1).unwrap();

    if setting == "location" {
        let default_location = PathBuf::from(value).canonicalize().unwrap();
        if !default_location.is_dir() {
            eprintln!("The default location {} is not a directory", value);
            return;
        }

        let timeline = config::open_timeline(&timeline_file).unwrap();
        let success = timeline.execute("
            update configuration set default_location = ?1;
        ", params![default_location.to_str().unwrap()]);

        if let Ok(_) = success {
            eprintln!("OK");
        } else {
            eprintln!("Failed to set default location to {}", default_location.to_str().unwrap());
        }

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

