mod file;
mod folder;
mod varg;
mod config;
use varg::Args;

mod zkinit;
mod zkset;
mod zkcard;
mod model;

fn main() {
    let args = Args::from_user_args();

    if let Some(subcommand) = args.subcommand {
        if let Some(timeline_file) = args.timeline_file {
            if subcommand == "init".to_string() {
                zkinit::zkinit(timeline_file);
            }
            else if subcommand == "set".to_string() {
                zkset::zkset(timeline_file, &args.args);
            }
            else if subcommand == "card".to_string() {
                zkcard::zkcard(timeline_file);
            }
        }
    }
}
