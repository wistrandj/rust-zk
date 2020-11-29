// Usage:
//   $ zk-card --timeline ./timeline.zk card

use std::path::PathBuf;
use rusqlite::Connection;

use super::file;
use super::model;
use super::model::cardfolder;
use super::model::carddb;
use super::hash;

pub fn zkcard(timeline_file: &PathBuf) {
    let mut timeline: Connection = model::open_timeline(&timeline_file).unwrap();
    let opencards = cardfolder::CardFolder::from_timeline(&timeline);
    let cards = opencards.cards();
    let next = cardfolder::next_available(&cards);
    let next_location: PathBuf = next.location_in(&opencards.folder);
    eprintln!("Open a new card in {}", next.name());
    file::make_template(&next_location);
    file::edit(&next_location);
    let hash: hash::Hash = model::blob::save(&timeline, &next_location);
    carddb::save_card_and_hash(&mut timeline, &next, &hash);
}

