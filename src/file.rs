use std::path::{Path, PathBuf};
use std::fs;
use chrono::{Local, DateTime};
use std::process::Command;
use std::time::SystemTime;


pub fn make_template(path: &Path) {
    let now = SystemTime::now();
    let date: DateTime<Local> = DateTime::from(now);
    let date_template = format!("{}\n\n\n", date.format("%Y-%m-%d"));
    let success = fs::write(&path, date_template);

    if let Err(_) = success {
        eprintln!("Failed to write template {}", path.to_str().unwrap());
    }
}

pub fn edit(file: &PathBuf) {
    let child = Command::new("/usr/bin/vim")
        .arg(file)
        .arg("-c")
        .arg("$")
        .spawn();

    if let Ok(mut child) = child {
        if let Ok(_exit) = child.wait() {
            println!("Note saved");
        } else {
            eprintln!("Fail to wait for vim");
        }
    } else {
        eprintln!("Fail to spawn vim");
    }
}

