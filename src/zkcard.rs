// Usage:
//   $ zk-card --timeline ./timeline.zk card

use std::path::PathBuf;
use rusqlite::Connection;

use super::file;
use super::config;
use super::model::opencard;
use super::db::dbcard;

pub fn zkcard(timeline_file: &PathBuf) {
    let mut timeline: Connection = config::open_timeline(&timeline_file).unwrap();
    let opencards = opencard::CardFolder::from_timeline(&timeline);
    let cards = opencards.cards();
    let next = opencard::next_available(&cards);
    let next_location: PathBuf = next.location_in(&opencards.folder);
    eprintln!("Open a new card in {}", next.name());
    file::make_template(&next_location);
    file::edit(&next_location);
    dbcard::save_cards(&mut timeline, &vec![next]);
}

