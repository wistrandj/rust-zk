use rusqlite::{Connection, params};
use crate::model::schema::Tag as TagSQL;
use crate::model::schema::TagFeature as TagFeature;
use crate::card;

// Note(wistrandj): Structs in this module should use FnOnce. Each of them are an
// action with side effect on the filesystem or on the timeline file. They can only
// run once. So the FnOnce trait describes them the best.

/// A command to set a tag to the given card.
pub struct SetTag<'a> {
    connection: &'a Connection,
    tag_name: String,
    major_card_number: usize,
}

pub struct DeleteTag<'a> {
    /// Delete a tag from the given card.
    connection: &'a Connection,
    tag_name: String,
    major_card_number: usize,
}

pub struct DeleteTagAll<'a> {
    /// Delete tag from all cards.
    connection: &'a Connection,
    tag_name: String
}

pub struct CreateTagHistoryBatch<'a> {
    connection: &'a Connection,
    batch_id: usize,
}

pub struct DropLatestTagHistoryBatch<'a> {
    connection: &'a Connection,
    batch_id: usize,
}

pub struct ShowTag<'a> {
    /// Find tags from the database.
    connection: &'a Connection,

    // Either None, which shows all tags. If it's a list, then show tags for the given cards only.
    major_card_numbers: Option<Vec<usize>>,
}

pub struct ShowAllCardsHavingTag<'a> {
    connection: &'a Connection,
    tag_names: Vec<String>,
}

impl<'a> ShowTag<'a> {
    pub fn new_show_all_tags(connection: &'a Connection) -> ShowTag<'a> {
        return ShowTag {
            connection,
            major_card_numbers: None,
        }
    }

    pub fn new_show_card_tags(connection: &'a Connection, major_card_numbers: &Vec<usize>) -> ShowTag<'a> {
        let mut copy = Vec::new();
        for it in major_card_numbers {
            copy.push(*it);
        }
        return ShowTag {
            connection,
            major_card_numbers: Some(copy),
        }
    }

    pub fn call_once(&self) -> Result<Vec<String>, &'static str> {
        let feat = TagFeature::new(self.connection);
        if let Some(major_card_numbers) = &self.major_card_numbers {
            return feat.find_tags_of_cards(major_card_numbers);
        } else {
            return feat.all_tags();
        }
    }
}

impl<'a> SetTag<'a> {
    pub fn new(connection: &'a Connection, tag_name: &str, face: card::Face) -> Result<Option<SetTag<'a>>, &'static str> {
        if !is_valid_tag(tag_name) {
            return Err("Invalid tag name");
        }

        let feat = TagFeature::new(connection);
        let major_card_number: usize = face.major_number();
        let tag_exists = feat.tag_is_set(tag_name, major_card_number)?;

        if tag_exists {
            Ok(None)
        } else {
            Ok(Some(SetTag {
                connection,
                tag_name: String::from(tag_name),
                major_card_number,
            }))
        }
    }

    pub fn call_once(self, args: ()) -> Result<(), &'static str> {
        let feat = TagFeature::new(self.connection);
        return feat.set_tag_to_card(&self.tag_name, self.major_card_number);
    }
}


impl<'a> DeleteTag<'a> {
    pub fn new(connection: &'a Connection, tag: &str, face: card::Face) -> Result<DeleteTag<'a>, &'static str> {
        let major_card_number: usize = face.major_number();

        Ok(DeleteTag {
            connection,
            tag_name: String::from(tag),
            major_card_number,
        })
    }

    pub fn call_once(self) -> Result<(), &'static str>{
        let feat = TagFeature::new(self.connection);
        return feat.unset_tag_of_card(&self.tag_name, self.major_card_number);
    }
}

impl<'a> DeleteTagAll<'a> {
    pub fn new(connection: &'a Connection, tag_name: &str) -> Result<DeleteTagAll<'a>, &'static str> {
        println!("02 here");
        let feat = TagFeature::new(&connection);
        let tag_exists = feat.tag_exists(tag_name)?;
        println!("Tag exists or not: {}", tag_exists);
        if tag_exists {
            return Ok(DeleteTagAll {
                connection,
                tag_name: String::from(tag_name),
            });
        } else {
            return Err("Tag does not exists");
        }
    }

    pub fn call_once(&self) -> Result<(), &'static str> {
        println!("03 here");
        println!("DeleteTagAll: call_once");
        let feat = TagFeature::new(self.connection);
        feat.unset_tag_from_all_cards(&self.tag_name)
    }
}

impl<'a> CreateTagHistoryBatch<'a> {
    pub fn new(connection: &'a Connection) -> Result<CreateTagHistoryBatch<'a>, &'static str> {
        let feat = TagFeature::new(connection);
        let latest_batch_id = feat.latest_batch_id_or_zero()?;
        let this_batch_id = latest_batch_id + 1;
        Ok(CreateTagHistoryBatch {
            connection,
            batch_id: this_batch_id
        })
    }

    pub fn call_once(&self) {
        panic!("Should not be called");
    }
}

impl<'a> DropLatestTagHistoryBatch<'a> {
    pub fn new(connection: &'a Connection) -> Result<DropLatestTagHistoryBatch<'a>, &'static str> {
        let feat = TagFeature::new(connection);
        let latest_batch_id = feat.latest_batch_id_or_zero()?;
        Ok(DropLatestTagHistoryBatch {
            connection,
            batch_id: latest_batch_id
        })
    }

    pub fn call_once(self) -> Result<(), &'static str> {
        let feat = TagFeature::new(self.connection);
        feat.delete_card_tags_in_a_batch(self.batch_id)?;
        feat.delete_batch_from_history(self.batch_id)?;
        Ok(())
    }
}

impl<'a> ShowAllCardsHavingTag<'a> {
    pub fn new(connection: &'a Connection, tag_names: &Vec<String>) -> ShowAllCardsHavingTag<'a> {
        let mut tag_names_vec: Vec<String> = Vec::new();
        for tag_name in tag_names {
            let tag_name_copy = String::from(tag_name);
            tag_names_vec.push(tag_name_copy);
        }

        return ShowAllCardsHavingTag {
            connection,
            tag_names: tag_names_vec,
        }
    }

    pub fn call_once(&self) -> Result<Vec<usize>, &'static str> {
        let feat = TagFeature::new(self.connection);
        return feat.find_all_cards_for_given_tags(&self.tag_names);
    }
}

pub fn is_valid_tag(name: &str) -> bool {
    let mut check_first_char = true;
    let mut first_char_letter = true;
    let mut valid_letters = true;
    let empty_string = name.len() == 0;

    for ch in name.chars() {
        if check_first_char {
            check_first_char = false;
            first_char_letter = ch.is_ascii_alphabetic();
        }

        let valid = ch.is_ascii_alphanumeric() || ch == '_' || ch == '-';
        if !valid {
            valid_letters = false;
        }
    }
    return first_char_letter && !empty_string && valid_letters;
}
