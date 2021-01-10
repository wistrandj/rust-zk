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

