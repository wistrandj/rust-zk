use std::path::{PathBuf, Path};
use std::cmp::{Eq, Ord, PartialOrd, Ordering};

enum Component {
    Number(usize),
    Char(String),
}

pub struct CardFace {
    name_components: Vec<Component>,
}


fn name_components(name: &str) -> Option<Vec<Component>> {
    if name.len() == 0 {
        return None;
    }

    let mut check_first_char = true;
    for c in name.chars() {
        let valid = c.is_ascii_digit() || c.is_ascii_lowercase();
        if !valid {
            return None;
        }

        if check_first_char {
            check_first_char = false;
            if !c.is_ascii_digit() {
                return None;
            }
        }
    }

    let mut chars_iter = name.chars().enumerate().peekable();
    let mut ranges: Vec<&str> = Vec::new();
    let mut range_start: usize = 0;

    loop {
        let next = chars_iter.next();
        if next.is_none() {
            break;
        }

        let (i, c) = next.unwrap();
        let c_is_digit = c.is_ascii_digit();
        let c_is_char = c.is_ascii_lowercase();

        let peek = chars_iter.peek();
        if let Some((_, d)) = peek {
            let d_is_digit = d.is_ascii_digit();
            let d_is_char = d.is_ascii_lowercase();

            let changed = (c_is_digit && d_is_char) || (c_is_char && d_is_digit);

            if changed {
                let s: &str = &name[range_start..=i];
                ranges.push(s);
                range_start = i + 1;
            }
        } else {
            let s: &str = &name[range_start..=i];
            ranges.push(s);
            range_start = i + 1;
        }
    }

    let mut result: Vec<Component> = Vec::new();
    for range in ranges {
        let first_char = range.chars().next().unwrap();
        if first_char.is_ascii_digit() {
            let number: usize = range.parse().unwrap();
            result.push(Component::Number(number));
        } else {
            let s = String::from(range);
            result.push(Component::Char(s));
        }
    }

    return Some(result);
}

impl CardFace {
    pub fn from_name(name: &str) -> Option<CardFace> {
        let comps = name_components(name);
        if let Some(comps) = comps {
            return Some(CardFace {
                name_components: comps
            });
        } else {
            return None;
        }
    }

    pub fn from_number(major_number: usize) -> CardFace {
        CardFace {
            name_components: vec![Component::Number(major_number)],
        }
    }

    pub fn major_number(&self) -> usize {
        let number = self.name_components.get(0).unwrap();
        if let Component::Number(number) = number {
            return number.clone();
        }
        panic!("Not reachable");
    }

    pub fn name(&self) -> String {
        let mut name = String::new();
        for comp in &self.name_components {
            let s = match comp {
                Component::Number(number) => number.to_string(),
                Component::Char(chars) => String::from(chars),
            };
            name.push_str(s.as_str());
        }

        return name;
    }

    pub fn is_major(&self) -> bool {
        return self.name_components.len() == 1;
    }

    pub fn location_in(&self, dir: &Path) -> PathBuf {
        let mut file = PathBuf::from(dir);
        file.push(self.name());
        return file;
    }
}


/* Ordering implementations */

impl PartialEq for Component {
    fn eq(&self, that: &Component) -> bool {
        match (self, that) {
            (Component::Number(n), Component::Number(m)) => n == m,
            (Component::Char(n), Component::Char(m)) => n == m,
            _ => {
                panic!("Compare numbers to chars");
            }
        }
    }
}

impl Eq for Component { }

impl PartialOrd for Component {
    fn partial_cmp(&self, that: &Component) -> Option<Ordering> {
        match (self, that) {
            (Component::Number(n), Component::Number(m)) => n.partial_cmp(m),
            (Component::Char(n), Component::Char(m)) => n.partial_cmp(m),
            _ => None
        }
    }
}

impl PartialEq for CardFace {
    fn eq(&self, that: &CardFace) -> bool {
        self.cmp(that) == Ordering::Equal
    }
}

impl Eq for CardFace { }

impl PartialOrd for CardFace {
    fn partial_cmp(&self, that: &CardFace) -> Option<Ordering> {
        let both = self.name_components.iter().zip(&that.name_components);

        for (this, that) in both {
            let comp_order = this.partial_cmp(that);
            if let Some(Ordering::Equal) = comp_order {
                return Some(Ordering::Equal);
            }
            return comp_order;
        }

        // All components were the same until now. If there's more components left
        // in either list, that's a deeper sub-card and therefore greater.
        return self.name_components.len().partial_cmp(&that.name_components.len());
    }
}

impl Ord for CardFace {
    fn cmp(&self, that: &CardFace) -> Ordering {
        // Valid cards can be always compared. Component has only PartialOrd because
        // Numbers cannot be compared against Chars. A valid cards have Numbers and
        // Chars in alternating order so invalid comparison cannot happen.
        return self.partial_cmp(that).unwrap();
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_create() {
        let cards = ["123", "123a", "123a1", "123a1b", "123a1b2"];
        for name in &cards {
            let card = CardFace::from_name(&name).unwrap();
            assert_eq!(card.major_number(), 123usize);
            assert_eq!(card.name(), *name);
        }
    }

    #[test]
    fn test_number() {
        let numbers = [1usize, 123, 4_usize.pow(31)];
        for number in numbers.iter() {
            let name = number.to_string();
            let card = CardFace::from_name(name.as_str()).unwrap();
            assert_eq!(card.major_number(), *number);
        }
    }

    #[test]
    fn test_negative() {
        let cards = ["", "a123", "a123a1", "123A", "123?"];
        for name in &cards {
            let card = CardFace::from_name(&name);
            assert!(card.is_none());
        }
    }

    #[test]
    fn test_location_in() {
        let card = CardFace::from_name("123a").unwrap();
        let dir = PathBuf::from("./foo/bar");
        assert_eq!(card.location_in(&dir), PathBuf::from("./foo/bar/124a"));
    }
}
