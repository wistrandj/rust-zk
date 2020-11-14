use rusqlite::{Connection, params};
use crate::model::cardface::Face;

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

pub fn save_cards(conn: &mut Connection, cards: &Vec<Face>) {
    let mut do_rollback = false;
    let mut commit = conn.savepoint().unwrap();

    {
        let mut stmt = commit.prepare(
            "insert into card(card_name) values (?1) on conflict do nothing"
        ).unwrap();

        for card in cards {
            let name = card.name();
            let success = stmt.execute(params![name]);

            if let Err(msg) = success {
                eprintln!("Fail to save card {}. Reason: {}", name, msg);
                do_rollback = true;
                break;
            }
        }
    }

    if do_rollback {
        commit.rollback();
    } else {
        commit.commit();
    }
}

