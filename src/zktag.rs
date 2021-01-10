use crate::varg::Args;
use rusqlite::Connection;
use std::path::PathBuf;
use crate::model::tag as tag_lib;
use crate::card;
use crate::model;

fn set_tag_to_given_cards(args: &Args) -> Result<(), &'static str> {
    let mut iter = args.args.iter();
    let tag_name = iter.next().unwrap();
    if !tag_lib::is_valid_tag(&tag_name) {
        eprintln!("Invalid tag name: {}", tag_name);
        return Err("Invalid tag name")
    }
    let cards_iter = iter;

    let timeline_file = args.timeline_file.as_ref().unwrap();
    let connection = model::open_timeline(timeline_file).unwrap();

    let mut set_tag_commands: Vec<tag_lib::SetTag> = Vec::new();

    for card_name in cards_iter {
        let card_path = PathBuf::from(card_name);
        let card_file_name = card_path.file_name().unwrap();
        let card_file_name: String = card_file_name.to_string_lossy().to_string();
        let card_face = card::Face::from_name(&card_file_name);
        if let Some(card_face) = card_face {
            let maybe_set_tag = tag_lib::SetTag::new(&connection, tag_name, card_face)?;
            if let Some(set_tag) = maybe_set_tag {
                set_tag_commands.push(set_tag);
            }
        } else {
            eprintln!("{}", format!("Not a card name: {}", card_file_name));
        }
    }

    // Assuming there is no faults, do the side-effect thing.

    for cmd in set_tag_commands {
        // Todo(wistrandj): NIL argument.
        cmd.call_once(());
    }

    Ok(())
}

fn delete_tag_of_given_cards(connection: &Connection, tag: &str, cards: &[String]) -> Result<(), &'static str> {
    let mut cmds = Vec::new();

    for card in cards {
        let card_face = card::Face::from_name(card.as_str());
        if let Some(card_face) = card_face {
            let delete_command = tag_lib::DeleteTag::new(connection, tag, card_face)?;
            cmds.push(delete_command);
        } else {
            return Err("Invalid card name given");
        }
    }

    for cmd in cmds {
        cmd.call_once()?;
    }

    Ok(())
}

fn delete_whole_tag(connection: &Connection, tag: &str) -> Result<(), &'static str> {
    let cmd = tag_lib::DeleteTagAll::new(connection, tag)?;
    let r = cmd.call_once();
    return r;
}

fn help_text() {
    println!("Usage of tag subcommand:");
    println!("Set the tag DCN1 to cards");
    println!("  zk -t ./here.zk tag DCN1 15 101");
    println!("");
    println!("Show all tags");
    println!("   zk -t ./here.zk tag --list");
    println!("");
    println!("Show tags of the given card");
    println!("   zk -t ./here.zk tag --list 101");
    println!("");
    println!("Delete a tag of the given card");
    println!("   zk -t ./here.zk tag --delete DCN! 101");
}

pub fn zktag(timeline: &PathBuf, args: &Args) -> Result<(), &'static str> {
    // Todo(wistrandj): Feature to rename a tag
    let parameters = &args.args;
    if parameters.len() == 0 {
        println!("Need more arguments");
    }

    let first_argument = parameters.get(0).unwrap();
    if first_argument == "--list" || first_argument == "-l" {
        // Todo(wistrandj): This feature shows all tags. Add a feature to this switch: The user can
        // give a list of cards. Show only tags on those cards. User does not give any cards: show
        // all cards.
        let timeline_file = args.timeline_file.as_ref().unwrap();
        let connection = model::open_timeline(timeline_file).unwrap();

        if parameters.len() > 1 {  // The first argument is --list or -l
            // Show tags on the given cards
            let mut major_numbers = Vec::new();

            for card_name in &parameters[1..] {
                let cardface = card::Face::from_name(card_name);
                if let Some(cardface) = cardface {
                    let major_number = cardface.major_number();
                    major_numbers.push(major_number);
                } else {
                    return Err("Invalid card name given")
                }
            }

            let show_tag = tag_lib::ShowTag::new_show_card_tags(&connection, &major_numbers);
            let tags: Vec<String> = show_tag.call_once()?;
            for tag in tags {
                println!("{}", tag);
            }
        } else {
            // Show all tags. User did not give any extra card arguments.
            let show_tag = tag_lib::ShowTag::new_show_all_tags(&connection);
            let tags: Vec<String> = show_tag.call_once()?;
            for tag in tags {
                println!("{}", tag);
            }
        }
    } else if first_argument == "--delete" || first_argument == "-d" {
        let timeline_file = args.timeline_file.as_ref().unwrap();
        let connection = model::open_timeline(timeline_file).unwrap();
        if let Some(tag_name) = parameters.get(1) {
            let cards_or_empty_list = &parameters.as_slice()[2..];
            if cards_or_empty_list.len() == 0 {
                let success = delete_whole_tag(&connection, tag_name);
                match success {
                    Ok(_) => { return Ok(()) },
                    Err(msg) => {
                        eprintln!("{}", msg);
                        return Err(msg);
                    }
                }
            } else {
                return delete_tag_of_given_cards(&connection, tag_name, cards_or_empty_list);
            }
        } else {
            eprintln!("Missing tag");
        }
    } else if first_argument == "--show" || first_argument == "-s" {
        // Show all tags of given list of cards.
        let mut user_args_tags = Vec::new();
        if parameters.len() == 1 {   // The first argument is "--show" or "-s"
            eprintln!("Missing tag name");
            return Err("Missing tag name")
        }
        for possibly_tag_argument in &parameters[1..] {
            let tag_argument = String::from(possibly_tag_argument);
            if tag_lib::is_valid_tag(&tag_argument) {
                user_args_tags.push(tag_argument);
            }

            if user_args_tags.len() == 0 {
                eprintln!("No tags given");
                return Err("No tags given");
            }

        }
        let timeline_file = args.timeline_file.as_ref().unwrap();
        let connection = model::open_timeline(timeline_file).unwrap();
        let cmd = tag_lib::ShowAllCardsHavingTag::new(&connection, &user_args_tags);

        let result = cmd.call_once();

        if let Ok(major_card_numbers) = result {
            for major_card in major_card_numbers {
                println!("{}", major_card);
            }
        } else  if let Err(msg) = result{
            eprintln!("{}", msg);
            return Err(msg);
        }
    } else if args.args.len() > 1 {
        // Set a tag to given cards
        // The first argument has to be the tag name. Rest of them are the cards. Either name of
        // the cards or a path to each of them.
        set_tag_to_given_cards(args)?;
    } else {
        help_text();
        return Ok(());
    }

    Ok(())
}
