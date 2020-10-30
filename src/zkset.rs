use std::env;
use std::path::PathBuf;
use rusqlite::{params};

mod varg;
mod config;

#[derive(Debug)]
struct A {
    version: u32,
    default_location: String
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let timeline_file = varg::get_timeline(&mut args);
    if let Some(timeline_file) = timeline_file {
        if args.len() < 3 {
            eprintln!("Invalid number of arguments. Need four.");
            return;
        }

        if !timeline_file.is_file() {
            eprintln!("Timeline {} is missing", timeline_file.to_str().unwrap());
            return;
        }

        let setting_name: &String = args.get(1).unwrap();
        let setting_value: &String = args.get(2).unwrap();

        if setting_name == "location" {
            let default_location = PathBuf::from(setting_value).canonicalize().unwrap();
            if !default_location.is_dir() {
                eprintln!("The default location {} is not a directory", setting_value);
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

}

