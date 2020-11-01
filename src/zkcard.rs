// Usage:
//   $ zk-card --timeline ./timeline.zk card

use std::path::PathBuf;
use rusqlite::{Connection, params};

use super::file;
use super::config;
use super::model::opencard;
use super::model::cardface::CardFace;
use super::model::dbcard;


#[derive(Debug)]
struct A {
    default_location: String
}

fn opencard_folder(timeline: &Connection) -> Option<PathBuf> {
    let mut query_location = timeline.prepare("select default_location from configuration;").unwrap();
    let location = query_location.query_row(params![], |row| {
        Ok(A {
            default_location: row.get(0)?
        })
    });
    let location = PathBuf::from(location.unwrap().default_location);
    if !location.is_dir() {
        eprintln!("The location for open cards {} is missing", location.to_str().unwrap());
        return None;
    }

    return Some(location);
}

pub fn zkcard(timeline_file: PathBuf) {
    let mut timeline: Connection = config::open_timeline(&timeline_file).unwrap();
    let location = opencard_folder(&timeline).unwrap();
    let next_card: CardFace = opencard::next_major_card(&location);
    let next_card_location: PathBuf = next_card.location_in(&location);

    println!("Open a new card in {}", next_card.name());

    file::make_template(&next_card_location);
    file::edit(&next_card_location);
    dbcard::save_cards(&mut timeline, &vec![next_card]);
}

