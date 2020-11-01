use rusqlite::{Connection, params};

pub trait Feature {
    fn enable(&self, conn: &mut Connection);
    fn rollback(&self, conn: &mut Connection);
}

pub fn enable_feature(name: &str, conn: &mut Connection, feature: &dyn Feature) {
    feature.enable(conn);
}

pub fn has_feature_table(name: &str, conn: &Connection) -> bool {
    let row = conn.query_row(
        "select count(*) from sqlite_master where type = 'table' and name = 'feature'",
        params![],
        |row| {
            let count_of_tables: u32 = row.get(0)?;
            let count_of_tables: usize = count_of_tables as usize;
            Ok(count_of_tables)
        }
    );

    return match row {
        Ok(number) => (number == 1),
        Err(_) => { panic!("Fail to read sqlite_master table"); }
    }
}

pub fn has(name: &str, conn: &Connection) -> bool {
    let row = conn.query_row(
        "select count(*) from feature where feature_name = ?1;",
        params![],
        |row| {
            let count_of_tables: u32 = row.get(0)?;
            let count_of_tables: usize = count_of_tables as usize;
            Ok(count_of_tables)
        }
    );

    return match row {
        Ok(number) => (number == 1),
        Err(_) => { panic!("Fail to read features"); }
    }
}

pub fn create_feature_table(conn: &Connection) {
    let success = conn.execute_batch(
        "
        begin;
        create table feature (
            feature_name text not null
        );
        insert into feature(feature_name) values ('feature');
        commit;
        "
    );

    if let Err(msg) = success {
        panic!("Fail to create feature table. Reason: {}", msg);
    }
}

