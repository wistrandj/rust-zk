use std::path::PathBuf;
use super::varg;
use super::db;
use super::model;
use super::db::blob;

// Todo(wistrandj):
// - solve the issue with varg module
// - create subcommand for zkblob --put: save a file by it's sha256
// - create subcommand for zkblob --get: load a file by it's sha256

pub fn zkblob(timeline_file: &PathBuf, args: &varg::Args) {
    let mut action_and_files = args.args.iter();
    let action = action_and_files.next();
    let mut files: Vec<PathBuf> = Vec::new();

    if action.is_none() {
        eprintln!("The blob subcommand expects more arguments. [put/get]");
        return;
    }

    match action.unwrap().as_str() {
        "get" => {
            panic!("Not implemented yet.");
        },
        "put" => {
            for filestr in action_and_files {
                let file = PathBuf::from(filestr);
                files.push(file);
            }
        },
        _ => {
            eprintln!("Invalid blob command [put/get]");
            return;
        }
    }

    let timeline = db::open_new_timeline(&timeline_file).unwrap();

    for file in files {
        blob::save(&timeline, &file);
        println!("Add file {:?}", file);
    }
}
