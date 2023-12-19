mod cock;
use cock::*;

use dotenv::{dotenv, var};
use std::collections::HashMap;

use regex::Regex;

fn main() {
    dotenv().unwrap();

    let prefix = var("PREFIX").unwrap_or_else(|e|panic!("PREFIX not set: {e}"));
    let from = var("INPUT").unwrap_or_else(|_|panic!("INPUT not set"));
    let to = var("OUTPUT").unwrap_or_else(|_|panic!("OUTPUT not set"));

    let txt_file = std::fs::read_to_string(format!("{from}.txt")).expect("Failed to read txt");
    let xml_file = std::fs::read_to_string(format!("{from}.xml")).expect("Failed to read xml");
    let notes = parse_notes(&txt_file);
    let mut set : CustomSet = quick_xml::de::from_str(&xml_file).expect("Failed to read xml");

    for card in &mut set.cards.card {
        if !card.name.starts_with(&prefix) {
            card.name = format!("{prefix}{}", card.name);
            continue
        }
        let key = card.name[prefix.len()..].to_owned();
        let (name, note_str) = notes.get(&*key).unwrap_or_else(||panic!("Failed to find note for `{}`", key));
        card.name = format!("{prefix}{name}");
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

fn parse_notes<'a>(file: &'a str) -> HashMap<String, (String, &'a str)> {
    let name_regex = Regex::new(r"^(.*?)\[/b\]").unwrap();
    let note_regex = Regex::new(r"(?s)\[spoiler\]Card Notes: (.*?)\[/spoiler\]").unwrap();
    let remove_regex  = Regex::new(r"’").unwrap();
    let mut map = HashMap::new();

    for card in file.split("[b]").skip(1) {
        let note_name = name_regex.captures(card).unwrap()[1].to_owned();
        let note: &'a str = note_regex.captures(card).map_or("<note></note>", |cap|cap.get(1).unwrap().as_str());
        let name = remove_regex.replace_all(&note_name, "'").into_owned();
        let key = name.trim().to_owned();
        if map.insert(key, (name, note)).is_some() {
            panic!("`{note_name}` is defined multiple times");
        }
    }

    map
}
