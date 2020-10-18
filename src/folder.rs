use std::path::{PathBuf, Path};
use std::fs;
use std::io;

pub fn next_major_card(folder: &PathBuf) -> PathBuf {
    let all_cards = list_files(&folder);
    let major_cards = major_cards(&all_cards);
    let next_card = get_next_major_card(&major_cards, &folder);

    return next_card;
}

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

fn get_next_major_card(major_cards: &Vec<u64>, path: &PathBuf) -> PathBuf {
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

