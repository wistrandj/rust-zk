mod file;
mod varg;
use varg::Args;

mod control;
mod model;
mod zkinit;
mod zkset;
mod zkcard;
mod card;
mod feature;
mod hash;
mod zkblob;
mod zktag;

fn main() {
    let args = Args::from_user_args();

    if let Some(subcommand) = &args.subcommand {
        if let Some(timeline_file) = &args.timeline_file {
            match subcommand.as_str() {
                "init" => {
                    zkinit::zkinit(timeline_file, &args);
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
                "tag" => {
                    zktag::zktag(timeline_file, &args);
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
