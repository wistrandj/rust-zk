use rusqlite::{Connection, params};
use crate::feature as feature;
use std::collections::HashSet;


pub fn install_missing_features(conn: &mut Connection) {
    if !feature::has_feature_table("feature", &conn) {
        feature::create_feature_table(&conn);
    }
    feature::enable_feature("setup1", conn, &Setup1 {});
    feature::enable_feature("setup2", conn, &Setup2 {});
    feature::enable_feature("tag", conn, &Tag {});
}

struct Setup1 {}
impl feature::Feature for Setup1 {
    fn enable(&self, conn: &mut Connection) {
        let success = conn.execute(
            "
            create table configuration (
                version integer,
                default_location text
            );
            ",
            params![]
        );

        if let Err(msg) = success {
            panic!("Fail to create configuration table. Reason: {}", msg);
        }
    }

    fn rollback(&self, conn: &mut Connection) {
        let success = conn.execute(
            "delete table configuration;",
            params![]
        );

        if let Err(msg) = success {
            panic!("Fail to delete configuration table. Reason: {}", msg);
        }
    }
}


struct Setup2 {}
impl feature::Feature for Setup2 {
    fn enable(&self, conn: &mut Connection) {
        let success = conn.execute_batch(
            "
            create table content(
                content_sha256 text not null,
                blob blob
            );
            create table card(
                card_name text primary key, -- '123', '123a1'
                content_sha256 text
            );
            "
        );

        if let Err(msg) = success {
            panic!("Fail to create card and content tables. Reason: {}", msg);
        }
    }
    fn rollback(&self, conn: &mut Connection) {
        let success = conn.execute_batch(
            "
            delete table configuration;
            delete table card;
            "
        );

        if let Err(msg) = success {
            panic!("Fail to delete card and content tables. Reason: {}", msg);
        }
    }
}

pub struct TagFeature<'a> {
    connection: &'a Connection
}

impl<'a> TagFeature<'a> {
    pub fn new(connection: &'a Connection) -> TagFeature<'a> {
        TagFeature { connection }
    }

    pub fn all_tags(&self) -> Result<Vec<String>, &'static str> {
        let mut tags_set: HashSet<String> = HashSet::new();
        let mut tags_vec: Vec<String> = Vec::new();

        let sql = "select tag_name from tag order by 1;";
        let args = params![];
        let mut stmt = self.connection.prepare(sql);
        if let Ok(mut stmt) = stmt {
            let tags_result = stmt.query_map(args,
                |row| {
                    let tag_name: String = row.get(0)?;
                    return Ok(tag_name);
                }
            ).unwrap();

            for tag in tags_result {
                tags_set.insert(tag.unwrap());
            }
            for tag in tags_set.drain() {
                tags_vec.push(tag);
            }
            tags_vec.sort();
            return Ok(tags_vec);
        } else {
            return Err("Fail to prepare a query");
        }
    }

    /// Given a list of cards, find all tags associated to them.
    pub fn find_tags_of_cards(&self, major_card_numbers: &Vec<usize>) -> Result<Vec<String>, &'static str> {
        let mut found_tags = HashSet::new();

        let sql = "select tag_name from tag where major_card_number = ?1";
        for &major_card in major_card_numbers {
            let major_card: u32 = major_card as u32;
            let args = params![major_card];
            let mut stmt = self.connection.prepare(sql).unwrap();
            let result = stmt.query_map(args, |row| {
                let tag_name: String = row.get(0)?;
                return Ok(tag_name);
            });

            for tag_name_row in result.unwrap() {
                let tag_name: String = tag_name_row.unwrap();
                found_tags.insert(tag_name);
            }
        }

        let mut found_tags_vec = Vec::new();
        for tag_name in found_tags.drain() {
            found_tags_vec.push(tag_name);
        }
        found_tags_vec.sort();
        return Ok(found_tags_vec);
    }

    pub fn find_all_cards_for_given_tags(&self, tag_names: &Vec<String>) -> Result<Vec<usize>, &'static str> {
        let mut major_cards_set: HashSet<usize> = HashSet::new();

        let sql = "select major_card_number from tag where tag_name = ?1";
        let mut stmt = self.connection.prepare(sql).unwrap();

        for tag_name in tag_names {
            let args = params![tag_name];
            let rows = stmt.query_map(args, |row| {
                let major_card_number: u32 = row.get(0)?;
                return Ok(major_card_number as usize);
            }).unwrap();

            for row in rows {
                let major_card: usize = row.unwrap();
                major_cards_set.insert(major_card);
            }
        }

        let mut major_cards_vec = Vec::new();
        for major_card in major_cards_set.drain() {
            major_cards_vec.push(major_card);
        }
        major_cards_vec.sort();
        return Ok(major_cards_vec);
    }

    pub fn tag_exists(&self, tag_name: &str) -> Result<bool, &'static str> {
        let sql = "select count(*) from tag where tag_name = ?1";
        let args = params![tag_name];
        let success = self.connection.query_row(sql, args, |row| {
            let count: u32 = row.get(0)?;
            return Ok(count);
        });
        match success {
            Ok(count) => Ok(count > 0),
            Err(_) => Err("Fail to check if the given tag exists in a database")
        }
    }

    pub fn tag_is_set(&self, tag_name: &str, major_card_number: usize) -> Result<bool, &'static str> {
        let sql = "select count(*) from tag where tag_name = ?1 and major_card_number = ?2";
        let args = params![tag_name, major_card_number as u32];
        let tag_exists = self.connection.query_row(sql, args,
            |row| {
                let matching_rows: u32 = row.get(0)?;
                println!("{}", format!("> tags for {} are count of {}", major_card_number, matching_rows));
                return Ok(matching_rows > 0);
            });
        match tag_exists {
            Ok(tag_exists) => Ok(tag_exists),
            Err(_) => Err("Fail to find out if a tag exists on a card")
        }
    }

    /// Set a tag to a given card. Do nothing if it is set already.
    pub fn set_tag_to_card(&self, tag_name: &str, major_card_number: usize) -> Result<(), &'static str> {
        let sql = "insert into tag(tag_name, major_card_number) values (?1, ?2);";
        let args = params![tag_name, major_card_number as u32];
        let success = self.connection.execute(sql, args);
        match success {
            Ok(_) => Ok(()),
            Err(_) => Err("Fail to set a card to a card"),
        }
    }

    /// Unset a tag of the given card.
    pub fn unset_tag_of_card(&self, tag_name: &str, major_card_number: usize) -> Result<(), &'static str>  {
        let sql = "delete from tag where tag_name = ?1 and major_card_number = ?2;";
        let args = params![tag_name, major_card_number as u32];
        let success = self.connection.execute(sql, args);
        match success {
            Ok(_) => Ok(()),
            Err(_) => Err("Fail to unset a tag"),
        }
    }

    pub fn unset_tag_from_all_cards(&self, tag_name: &str) -> Result<(), &'static str> {
        let sql = "delete from tag where tag_name = ?1";
        let args = params![tag_name];
        let success = self.connection.execute(sql, args);
        println!("tag {} deleted from all cards", tag_name);
        match success {
            Ok(_) => Ok(()),
            Err(_) => Err("Fail to unset tag from all cards"),
        }
    }

    /// Latest batch id number or zero if no batches available.
    pub fn latest_batch_id_or_zero(&self) -> Result<usize, &'static str> {
        let sql = "select coalesce(max(batch_id), 0) as latest_batch_id from tag_history;";
        let args = params![];
        let success = self.connection.query_row(
            sql,
            args,
            |row| {
                let latest_batch_id: u32 = row.get(0)?;
                return Ok(latest_batch_id as usize);
        });
        match success {
            Ok(latest_batch_id) => Ok(latest_batch_id),
            Err(_) => Err("Fail to get the latest batch id"),
        }
    }

    /// Insert a tag into a history as a part of a batch.
    pub fn insert_tag_to_a_batch_history(&self, batch_id: usize, tag_name: &str, major_card_number: usize) -> Result<(), &'static str> {
        let sql = "insert into tag_history(batch_id, tag_name, major_card_number) values (?1, ?2, ?3);";
        let args = params![batch_id as u32, tag_name, major_card_number as u32];
        let success = self.connection.execute(sql, args);
        match success {
            Ok(_) => Ok(()),
            Err(_) => Err("Fail to set a card to a card"),
        }
    }

    /// Delete tags from cards in a given batch.
    pub fn delete_card_tags_in_a_batch(&self, batch_id: usize) -> Result<(), &'static str> {
        let sql = "delete from tag where (tag_name, major_card_number) in (select tag_name, card from tag_history where batch_id = ?1)";
        let args = params![batch_id as u32];
        let success = self.connection.execute(sql, args);
        match success {
            Ok(_) => Ok(()),
            Err(_) => Err("Fail to delete card tags"),
        }
    }

    /// Delete the batch from tag history.
    pub fn delete_batch_from_history(&self, batch_id: usize) -> Result<(), &'static str> {
        let sql = "delete from tag_history where batch_id = ?1";
        let args = params![batch_id as u32];
        let success = self.connection.execute(sql, args);
        match success {
            Ok(_) => Ok(()),
            Err(_) => Err("Fail to delete batch from history"),
        }
    }
}

pub struct Tag {}

impl feature::Feature for Tag {
    fn enable(&self, conn: &mut Connection) {
        let success = conn.execute_batch(
            "
            create table tag (
                tag_name text not null,
                major_card_number integer not null,
                unique(tag_name, major_card_number)
            );
            create table tag_history (
                batch_id integer not null,
                tag_name text not null,
                major_card_number integer not null
            );
            "
        );

        if let Err(msg) = success {
            panic!("Fail to create tag tables. Reason: {}", msg);
        }
    }
    fn rollback(&self, conn: &mut Connection) {
        let success = conn.execute_batch(
            "
            delete table tag;
            delete table tag_history;
            "
        );

        if let Err(msg) = success {
            panic!("Fail to delete tag tables. Reason: {}", msg);
        }
    }
}
