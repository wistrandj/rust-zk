use std::path::{PathBuf, Path};
use std::fs;
use std::io;
use crate::card::Face;
use rusqlite::{Connection, params};
use crate::card;

pub struct CardFolder {
    pub folder: PathBuf
}

impl CardFolder {
    pub fn new(folder: PathBuf) -> CardFolder {
        if folder.is_dir() {
            return CardFolder { folder };
        } else {
            // Note(wistrandj): This is not the way to handle errors.
            panic!("The default card folder is missing");
        }
    }

    pub fn from_timeline(timeline: &Connection) -> CardFolder {
        let row = timeline.query_row(
            "select default_location from configuration",
            params![],
            |row| {
                let folder: String = row.get(0)?;
                let folder = PathBuf::from(folder);
                return Ok(folder);
            });

        if let Ok(path) = row {
            let folder = PathBuf::from(path);
            return Self::new(folder);
        } else {
            panic!("Bug: The default card folder is not configured");
        }
    }

    pub fn cards(&self) -> Vec<card::Face> {
        let cardnames = list_file_names(&self.folder);

        if let Ok(cardnames) = cardnames {
            let cards: Vec<Face> = cardnames.iter()
                .map(|it| card::Face::from_name(it))
                .filter(|it| it.is_some())
                .map(|it| it.unwrap())
                .collect();
            return cards;
        } else {
            panic!("No cards");
        }
    }
}

pub fn next_available(cards: &Vec<card::Face>) -> Face {
    let latest_card = cards.iter().max();
    let latest_number: usize = if let Some(card) = latest_card { card.major_number() } else { 0 };
    return Face::from_number(latest_number + 1);
}

pub fn next_major_card(folder: &PathBuf) -> Face {
    let cards = list_cards(folder);
    let latest_card = cards.iter().max();
    let latest_number: usize = if let Some(card) = latest_card { card.major_number() } else { 0 };
    return Face::from_number(latest_number + 1);
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

fn list_cards(path: &Path) -> Vec<Face> {
    if let Ok(files) = list_file_names(path) {
        return files.iter()
            .map(|file| Face::from_name(file))
            .filter(|card| !card.is_none())
            .map(|card| card.unwrap())
            .collect();
    } else {
        return vec![];
    }
}

