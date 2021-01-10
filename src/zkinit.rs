use std::path::PathBuf;
use rusqlite::{params};
use super::varg;

use super::model;
use super::model::schema;

// Command line tool to create a new timeline database. Usage:
//   $ zk init ./timeline.zk .
//   $ zk init -t ./timeline.zk .

struct Arguments {
    timeline_file_location: String,
    cards_folder_location: String,
}

fn parse_arguments(args: &varg::Args) -> Option<Arguments> {
    if let Some(subcommand) = &args.subcommand {
        if subcommand != "init" {
            return None;
        }

        let arguments = &args.args;

        if arguments.len() == 0 {
            eprintln!("Not enough arguments. Give at least the name of the timeline file");
            return None;
        }

        let first_argument = arguments.get(0).unwrap();

        if arguments.len() == 1 {
            let timeline_file_location = String::from(first_argument);
            let cards_folder_location = String::from(".");
            return Some(Arguments {
                timeline_file_location,
                cards_folder_location,
            });
        }

        let second_argument = arguments.get(1).unwrap();

        if arguments.len() == 2 {
            let timeline_file_location = String::from(first_argument);
            let cards_folder_location = String::from(second_argument);
            return Some(Arguments {
                timeline_file_location,
                cards_folder_location,
            });
        }

        eprintln!("Too many arguments. Expecting timeline file and card folders location");
        return None;
    }

    if let Some(subcommand) = &args.subcommand {
        eprintln!("subcommand = {}", subcommand);
    }
    if let Some(timeline_file) = &args.timeline_file {
        let timeline_file: String = timeline_file.to_string_lossy().to_string();
        eprintln!("timeline_file = {}", timeline_file);
    }
    eprintln!("Rest of the arguments");
    for arg in args.args.iter() {
        eprintln!("> {}", arg);
    }
    return None;
}

pub fn zkinit(timeline_file: &PathBuf, args: &varg::Args) {
    let args_opt = parse_arguments(args);
    if args_opt.is_none() {
        // eprintln!("");
        return;
    }

    let arguments = args_opt.unwrap();

    let timeline_file_path = PathBuf::from(arguments.timeline_file_location);
    let cards_folder_location = PathBuf::from(&arguments.cards_folder_location);
    let cards_folder_location = cards_folder_location.canonicalize().unwrap();
    let only_update_existing_database = timeline_file_path.is_file();

    if !cards_folder_location.is_dir() {
        eprintln!("The cards folder {} does not exists", arguments.cards_folder_location);
        return;
    }

    if !only_update_existing_database {
        // Create the database
        let mut timeline = model::open_new_timeline(&timeline_file_path).unwrap();
        schema::install_missing_features(&mut timeline);

        let cards_folder_location = cards_folder_location.to_string_lossy().to_string();

        let sql = "insert into configuration(version, default_location) values (?1, ?2)";
        let sql_args = params![1, cards_folder_location];
        let success = timeline.execute(sql, sql_args);
        if let Err(msg) = success {
            eprintln!("Fail: {}", msg);
            eprintln!("Failed to set default location to the initial value (/tmp)", );
            return;
        }

        if let Err(_msg) = timeline.close() {
            eprintln!("Fail to close the database");
            return;
        }
    } else {
        let mut timeline = model::open_new_timeline(&timeline_file_path).unwrap();
        schema::install_missing_features(&mut timeline);
        if let Err(_msg) = timeline.close() {
            eprintln!("Fail to close the database");
            return;
        }
    }
}

