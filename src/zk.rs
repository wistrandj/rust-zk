use std::path::{PathBuf, Path};
use std::fs;
use std::io;
use std::process::Command;
use std::time::SystemTime;
use chrono::{Local, DateTime};
use std::env;
use rusqlite::{Connection};

mod config;

fn list_files_failing(path: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() {
            // println!("DIR {:?}", entry.path());
        } else if filetype.is_symlink() {
            // println!("SYM {:?}", entry.path());
        } else if filetype.is_file() {
            files.push(entry.path());
        }
    }

    return Ok(files);
}

fn list_files(path: &Path) -> Vec<PathBuf> {
    if let Ok(files) = list_files_failing(path) {
        return files;
    } else {
        panic!("Fail to read contents of the zk folder.");
    };
}

fn is_major_card(file: &str) -> bool {
    let mut valid_char;
    for ch in file.chars() {
        valid_char = ch.is_ascii_digit();
        if !valid_char {
            return false;
        }
    }
    return true;
}

fn major_cards(files: &Vec<PathBuf>) -> Vec<u64> {
    let mut major_cards = Vec::new();
    for file in files {
        let name = file.file_name().unwrap().to_str().unwrap();
        if is_major_card(&name) {
            let card_number: u64 = name.parse().unwrap();
            major_cards.push(card_number);
        }
    }
    return major_cards;
}

fn next_major_card(major_cards: &Vec<u64>, path: &PathBuf) -> PathBuf {
    let mut latest_major_card: u64 = 0;
    for card in major_cards {
        if *card > latest_major_card {
            latest_major_card = *card;
        }
    }

    let mut path = path.clone();
    let next_major_card = latest_major_card + 1;
    let next_major_card = next_major_card.to_string();
    path.push(PathBuf::from(next_major_card));
    return path;
}

fn make_template(path: &Path) {
    let now = SystemTime::now();
    let date: DateTime<Local> = DateTime::from(now);
    let date_template = format!("{}\n\n\n", date.format("%Y-%m-%d"));
    fs::write(&path, date_template);
}

fn edit(file: &PathBuf) {
    let mut child = Command::new("/usr/bin/vim")
        .arg(file)
        .arg("-c")
        .arg("$")
        .spawn();

    if let Ok(mut child) = child {
        if let Ok(exit) = child.wait() {
            println!("Note saved");
        } else {
            eprintln!("Fail to wait for vim");
        }
    } else {
        eprintln!("Fail to spawn vim");
    }
}

fn main() {
    let zk_folder = PathBuf::from("/tmp/tx/zk");
    // let zk_folder = PathBuf::from("/tmp/tx");
    println!("Open a note in {:?}", zk_folder);
    let all_cards = list_files(&zk_folder);
    let major_cards = major_cards(&all_cards);
    let config = config::arguments();

    if config.card {
        let next = next_major_card(&major_cards, &zk_folder);
        make_template(&next);
        edit(&next);
    } else {
        println!("No sub-command given");
    }
}
