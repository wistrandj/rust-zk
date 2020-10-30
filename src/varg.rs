use std::env;
use std::path::PathBuf;

pub fn get_timeline(args: &mut Vec<String>) -> Option<PathBuf> {
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
