mod cock;
use cock::*;

use std::collections::HashMap;

use regex::Regex;

fn main() {
    let mut args = std::env::args().skip(1);
    let prefix = args.next().unwrap();
    let from = args.next().unwrap();
    let to = args.next().unwrap();
    let txt_file = std::fs::read_to_string(format!("{from}.txt")).expect("Failed to read txt");
    let xml_file = std::fs::read_to_string(format!("{from}.xml")).expect("Failed to read xml");
    let notes = parse_notes(&txt_file);
    let mut set : CustomSet = quick_xml::de::from_str(&xml_file).expect("Failed to read xml");

    for card in &mut set.cards.card {
        let name = if card.name.starts_with(&prefix) {
            card.name[prefix.len()..].to_owned()
        } else {
            card.name = format!("{prefix}{}", card.name);
            continue
        };
        card.name = format!("{prefix}{name}");
        let note_str = *notes.get(&*name).unwrap_or_else(||panic!("Failed to find note for `{}`", name));
        let note: CardNote = quick_xml::de::from_str(note_str).unwrap_or_else(|e|panic!("Failed to parse note for `{}`: {:?}", name, e));
        if !card.related.is_empty() {
            panic!("`{}` has related cards", name);
        }
        card.related = note.related;
    }

    let mut output = r#"<?xml version="1.0" encoding="UTF-8"?>"#.to_owned();
    quick_xml::se::to_writer(&mut output, &set).unwrap();
    std::fs::write(format!("{to}.xml"), output).unwrap();
}

fn parse_notes<'a>(file: &'a str) -> HashMap<String, &'a str> {
    let name_regex = Regex::new(r"^(.*?)\[/b\]").unwrap();
    let note_regex = Regex::new(r"(?s)\[spoiler\]Card Notes: (.*?)\[/spoiler\]").unwrap();
    let remove_regex  = Regex::new(r"’").unwrap();
    let mut map = HashMap::new();

    for card in file.split("[b]").skip(1) {
        let name = name_regex.captures(card).unwrap()[1].to_owned();
        let note: &'a str = note_regex.captures(card).map_or("<note></note>", |cap|cap.get(1).unwrap().as_str());
        let key = remove_regex.replace_all(&name, "'").trim().to_owned();
        if map.insert(key, note).is_some() {
            panic!("`{name}` is defined multiple times");
        }
    }

    map
}
