use std::path::{PathBuf, Path};
use std::fs;
use std::io;
use super::cardface::CardFace;

pub fn next_major_card(folder: &PathBuf) -> PathBuf {
    let cards = list_cards(folder);
    let latest_card = cards.iter().max();
    let latest_number: usize = if let Some(card) = latest_card { card.major_number() } else { 0 };
    let card = CardFace::from_number(latest_number + 1);
    return card.location_in(folder);
}

fn list_file_names(path: &Path) -> io::Result<Vec<String>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() || filetype.is_symlink() {
            continue;
        } else if filetype.is_file() {
            let path = entry.path();
            let filename = path.file_name().unwrap().to_str().unwrap();
            files.push(String::from(filename));
        }
    }

    return Ok(files);
}

fn list_cards(path: &Path) -> Vec<CardFace> {
    if let Ok(files) = list_file_names(path) {
        return files.iter()
            .map(|file| CardFace::from_name(file))
            .filter(|card| !card.is_none())
            .map(|card| card.unwrap())
            .collect();
    } else {
        return vec![];
    }
}

