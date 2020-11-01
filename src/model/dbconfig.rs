use rusqlite::{Connection, params};
use std::path::{PathBuf, Path};

pub fn default_location(timeline: &Connection) -> Option<PathBuf> {
    let row = timeline.query_row(
        "select default_location from configuration;",
        params![],
        |row| {
            let path: String = row.get(0)?;
            Ok(PathBuf::from(path))
        }
    );

    return match row {
        Ok(path) => Some(path),
        Err(msg) => {
            eprintln!("Fail to get the default location. Reason: {}", msg);
            None
        }
    }
}

pub fn set_default_location(timeline: &Connection, location: &Path) {
    let success = timeline.execute(
        "update configuration set default_location = ?1;",
        params![location.to_str()]
    );

    if let Err(msg) = success {
        eprintln!("Fail to set default location. Reason: {}", msg);
    }
}

pub fn version(timeline: &Connection) -> Option<usize> {
    let row = timeline.query_row(
        "select version from configuration;",
        params![],
        |row| {
            let version: i32 = row.get(0)?;
            let version: usize = version as usize;
            // let version: usize = version.parse().unwrap();
            Ok(version)
        }
    );

    return match row {
        Ok(version) => Some(version),
        Err(msg) => {
            eprintln!("Fail to get the version. Reason: {}", msg);
            None
        }
    }
}
