use std::path::PathBuf;

#[derive(Debug)]
pub struct Args {
    pub subcommand: Option<String>,
    pub timeline_file: Option<PathBuf>,
    pub args: Vec<String>
}

impl Args {
    pub fn from_user_args() -> Args {
        let mut args = std::env::args().collect::<Vec<String>>();
        let _executable = args.remove(0);
        return Args::from_vec(args);
    }

    fn from_vec(mut args: Vec<String>) -> Args {
        let timeline_file = get_timeline(&mut args);
        let subcommand = Args::subcommand(&mut args);

        Args {
            subcommand,
            timeline_file,
            args,
        }
    }

    fn subcommand(args: &mut Vec<String>) -> Option<String> {
        // Assume that the --timeline switch and it's parameter are removed.
        if args.len() == 0 {
            return None;
        }

        let all_commands = [
            "init", "card", "add", "set", "blob"
        ];

        let subcommand = args.get(0).unwrap();
        if all_commands.contains(&subcommand.as_str()) {
            let subcommand = args.remove(0);
            return Some(String::from(subcommand));
        } else {
            return None;
        }
    }
}

pub fn get_timeline(args: &mut Vec<String>) -> Option<PathBuf> {
    // Todo(wistrandj): Move this function under impl Args.
    let mut timeline_switch: Option<usize> = None;
    let mut path_index: Option<usize> = None;

    for (i, arg) in args.iter().enumerate() {
        if arg == "--timeline" || arg == "-t" {
            if let None = path_index {
                timeline_switch = Some(i);
                path_index = Some(i + 1);
            } else {
                eprintln!("Multiple timelines (--timeline) given.");
                return None;
            }
        }
    }

    if path_index.is_none() {
        eprintln!("No timeline (--timeline) given");
        return None;
    }

    let timeline_switch = timeline_switch.unwrap();
    let path_index = path_index.unwrap();

    if path_index >= args.len() {
        eprintln!("Missing timeline (--timeline) argument");
        return None;
    }

    let timeline = args.remove(path_index);
    let timeline = PathBuf::from(timeline);
    args.remove(timeline_switch);

    return Some(timeline);
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_positive1() {
        let mut a1: Vec<String> = vec!["--timeline", "./timeline.db"].iter().map(|s| s.to_string()).collect();
        let mut a2: Vec<String> = vec!["first", "--timeline", "./timeline.db"].iter().map(|s| s.to_string()).collect();
        let mut a3: Vec<String> = vec!["first", "--timeline", "./timeline.db", "second"].iter().map(|s| s.to_string()).collect();

        let path = get_timeline(&mut a1);
        assert_eq!(a1.len(), 0);
        assert_eq!(path, Some(PathBuf::from("./timeline.db")));

        let path = get_timeline(&mut a2);
        assert_eq!(a2.len(), 1);
        assert_eq!(path, Some(PathBuf::from("./timeline.db")));

        let path = get_timeline(&mut a3);
        assert_eq!(a3.len(), 2);
        assert_eq!(path, Some(PathBuf::from("./timeline.db")));
    }

    #[test]
    fn test_negative() {
        let mut a1: Vec<String> = vec![];
        let mut a2: Vec<String> = vec!["--timeline"].iter().map(|s| s.to_string()).collect();
        let mut a3: Vec<String> = vec!["./timeline.db"].iter().map(|s| s.to_string()).collect();
        let mut a4: Vec<String> = vec!["first", "--timeline"].iter().map(|s| s.to_string()).collect();

        let mut alist = a1;
        let path = get_timeline(&mut alist);
        let len = alist.len();
        assert_eq!(path, None);
        assert_eq!(alist.len(), len);

        let mut alist = a2;
        let path = get_timeline(&mut alist);
        let len = alist.len();
        assert_eq!(path, None);
        assert_eq!(alist.len(), len);

        let mut alist = a3;
        let path = get_timeline(&mut alist);
        let len = alist.len();
        assert_eq!(path, None);
        assert_eq!(alist.len(), len);

        let mut alist = a4;
        let path = get_timeline(&mut alist);
        let len = alist.len();
        assert_eq!(path, None);
        assert_eq!(alist.len(), len);

    }
}


#[cfg(test)]
mod test1 {
    use super::*;
    #[test]
    fn test_argument_read() {
        let args = vec!["-t", "./timeline.db", "init"].iter().map(|s| s.to_string()).collect();
        let args = Args::from_vec(args);
        assert_eq!(args.subcommand, Some("init".to_string()));
        assert_eq!(args.timeline_file, Some(PathBuf::from("./timeline.db")));

        let args = vec!["init", "-t", "./timeline.db"].iter().map(|s| s.to_string()).collect();
        let args = Args::from_vec(args);
        assert_eq!(args.subcommand, Some("init".to_string()));
        assert_eq!(args.timeline_file, Some(PathBuf::from("./timeline.db")));

        let args = vec!["-t", "./timeline.db"].iter().map(|s| s.to_string()).collect();
        let args = Args::from_vec(args);
        assert_eq!(args.subcommand, None);
        assert_eq!(args.timeline_file, Some(PathBuf::from("./timeline.db")));
    }

    #[test]
    fn test_no_timeline() {
        let args = vec!["init"].iter().map(|s| s.to_string()).collect();
        let args = Args::from_vec(args);
        assert_eq!(args.subcommand, Some("init".to_string()));
        assert_eq!(args.timeline_file, None);
    }
}
