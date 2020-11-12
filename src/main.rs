mod file;
mod varg;
mod config;
use varg::Args;

mod db;
mod zkinit;
mod zkset;
mod zkcard;
mod model;
mod schema;
mod hash;
mod zkblob;

fn main() {
    let args = Args::from_user_args();

    if let Some(subcommand) = &args.subcommand {
        if let Some(timeline_file) = &args.timeline_file {
            match subcommand.as_str() {
                "init" => {
                    zkinit::zkinit(timeline_file);
                },
                "set" => {
                    zkset::zkset(timeline_file, &args.args);
                },
                "card" => {
                    zkcard::zkcard(timeline_file);
                },
                "blob" => {
                    zkblob::zkblob(timeline_file, &args);
                },
                _ => {
                    eprintln!("Invalid or missing subcommand");
                },
            }
        } else {
            eprintln!("The timeline file argument is missing");
        }
    } else {
        eprintln!("No subcommand given");
    }
}
