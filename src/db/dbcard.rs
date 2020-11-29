use crate::hash;
use rusqlite::{Connection, params};
use crate::card::Face;

fn cards(conn: &mut Connection) -> Vec<Face> {
    let mut stmt = conn.prepare("select card_name from card;").unwrap();
    let cards = stmt.query_map(
        params![],
        |row| {
            let name: String = row.get(0)?;
            let card = Face::from_name(name.as_str()).unwrap();
            Ok(card)
        });
    let cards: Vec<Face> = cards
        .unwrap()
        .map(|maybe_card| maybe_card.unwrap())
        .collect();
    return cards;
}


pub fn save_card_and_hash(conn: &Connection, card: &Face, hash: &hash::Hash) {
    let mut stmt = conn.prepare("insert or replace into card(card_name, content_sha256) values (?1, ?2);").unwrap();
    let name: String = card.name();
    let hash: String = hash.to_string();
    let success = stmt.execute(params![name, hash]);
    if let Err(msg) = success {
        eprintln!("Fail to save card and the hash");
    }
}

