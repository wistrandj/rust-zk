use std::path::{Path, PathBuf};
use rusqlite::{Connection, params, Savepoint};
use std::env;
use std::fs;
use std::ffi::OsStr;

mod config;

#[derive(Debug)]
struct A {
    default_location: String,
}

#[derive(Debug)]
struct Config {
    timeline: PathBuf,
    add_all: bool,
    add_cards: Option<Vec<PathBuf>>,
}

impl Config {
    fn default() -> Result<Config, &'static str> {
        let args: Vec<String> = std::env::args().collect();
        let add_cards = Config::_user_args_cards(&args);
        let add_all = Config::_user_args_add_all(&args) || add_cards.is_none();
        let timeline = Config::_user_args_timeline(&args);

        if timeline.is_none() {
            return Err("No timeline given");
        }

        let timeline = timeline.unwrap();

        Ok(Config {
            timeline,
            add_all: false,
            add_cards: None,
        })
    }

    fn _user_args_add_all(args: &Vec<String>) -> bool{
        for arg in args {
            if arg == "--all" {
                return true;
            }
        }
        return false;
    }

    fn _user_args_cards(args: &Vec<String>) -> Option<Vec<PathBuf>> {
        // Warning(wistrandj): This seem to fail. It does not return anything.
        let mut cards: Vec<PathBuf> = Vec::new();
        let mut skip_next = false;

        for arg in args {
            if skip_next {
                skip_next = false;
                continue;
            }
            if !skip_next {
                if arg == "--timeline" {
                    skip_next = false
                } else if arg == "--all" {
                    return None;
                } else {
                    let card = PathBuf::from(arg);
                    cards.push(card);
                }
            }
        }

        return Some(cards);
    }

    fn _user_args_timeline(args: &Vec<String>) -> Option<PathBuf> {
        let mut idx: Option<usize> = None;

        for (i, arg) in args.iter().enumerate() {
            if arg == "--timeline" || arg == "-t" {
                idx = Some(i + 1);
                break;
            }
        }

        if let Some(idx) = idx {
            let timeline_arg: &String = args.get(idx).unwrap();
            let timeline = PathBuf::from(timeline_arg);
            if !timeline.is_file() {
                return None;
            }
            return Some(timeline);
        }
        return None;
    }
}

fn inside_of_default_location(conn: &Savepoint, file: &Path) -> bool {
    let dir = PathBuf::from(file)
        .canonicalize().unwrap();
    let dir = dir.parent().unwrap();
    let dir: String = dir.to_string_lossy().to_string();

    let location = conn.query_row(
        "select default_location from configuration;",
        params![],
        |row| {
            Ok(A{
                default_location: row.get(0)?
            })
        });

    let location: String = location.unwrap().default_location;

    return location.as_str() == dir;
}

fn add_card(conn: &Savepoint, file: &Path) -> Result<(), ()> {
    let filename: &OsStr = file.file_name().unwrap();
    let filename: String = filename.to_string_lossy().to_string();
    let content: Vec<u8> = fs::read(file).unwrap();

    if inside_of_default_location(&conn, file) {
        // Todo(wistrandj): Calculate sha256 of the content
        let content_sha256 = "xxx";
        let success = conn.execute(
            "insert into content(content_sha256, blob) values (?1, ?2);",
            params![content_sha256, content],
        );

        if let Err(_) = success {
            return Err(());
        }

        let success = conn.execute(
            "insert into card(card_name, content_sha256) values (?1, ?2);",
            params![filename, content_sha256],
        );

        if let Ok(_) = success {
            return Ok(());
        }
    } else {
        eprintln!("The file is not in the default location");
        return Err(());
    }

    return Err(());
}

fn main() {
    let config = Config::default();
    println!("Config :::::::::: {:?}", config);

    // @Here
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Too little arguments! The first argument must be --timeline.");
        return;
    }

    if args.len() < 3 {
        eprintln!("No cards given");
        return;
    }

    let timeline_switch: &String = args.get(1).unwrap();
    let timeline_location: &String = args.get(2).unwrap();
    // let first_card: &String = args.get(2).unwrap();

    let timeline_location = PathBuf::from(timeline_location);
    let mut timeline = config::open_timeline(&timeline_location).unwrap();

    let batch = timeline.savepoint().unwrap();

    let cards_argument = &args[3..];
    for card in cards_argument {
        let card = PathBuf::from(card);
        if !card.is_file() {
            eprintln!("Card missing at {}", card.to_str().unwrap());
        }
        add_card(&batch, card.as_path());
    }

    batch.commit().unwrap();
}
