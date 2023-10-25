use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

pub struct DictionaryEntry {
    pub pinyin: String,
    pub definitions: String,
}

pub fn parse_dictionary() -> HashMap<String, DictionaryEntry> {
    let mut dictionary = HashMap::new();
    let file = File::open("cedict_ts.u8").expect("Dictionary file not found");
    let reader = BufReader::new(file);
    let entry_regex = Regex::new(r"(\S+) \[(\w+)\] (.+)").expect("Failed to create regex");

    for line in reader.lines() {
        if let Ok(line) = line {
            if let Some(captures) = entry_regex.captures(&line) {
                let character = captures[1].to_string();
                let pinyin = captures[2].to_string();
                let definitions = captures[3].to_string();

                let entry = DictionaryEntry { pinyin, definitions };
                dictionary.insert(character, entry);
            }
        }
    }

    dictionary
}

pub fn lookup_character<'a>(character: &'a str, dictionary: &'a HashMap<String, DictionaryEntry>) -> Option<&'a DictionaryEntry> {
    dictionary.get(character)
}