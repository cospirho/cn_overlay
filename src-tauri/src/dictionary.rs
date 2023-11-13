use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct DictionaryEntry {
    pub pinyin: String,
    pub definitions: String,
}

//TODO maybe just parse this once and save it to a file as something easier/faster to read
pub fn parse_dictionary() -> HashMap<String, DictionaryEntry> {
    let mut dictionary = HashMap::new();
    let file = File::open("cedict_ts.u8").expect("Dictionary file not found");
    let reader = BufReader::with_capacity(1024 * 1024, file);

    let mut pinyin_buffer = String::new();
    let mut definitions_buffer = String::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let mut parts = line.split([' ', '[', ']']);
    
            // Use match or if let to handle potential errors
            let trad_character = match parts.next() {
                Some(val) => val,
                None => {
                    continue;
                }
            };
    
            let character = match parts.next() {
                Some(val) => val.to_string(),
                None => {
                    continue;
                }
            };
    
            pinyin_buffer.clear();
            for i in 0..character.len() {
                let pinyin = match parts.next() {
                    Some(val) => {
                        pinyin_buffer.push_str(val);
                    }
                    None => {
                        continue;
                    }
                };
            }
            let definitions = parts.collect::<Vec<&str>>().join(" ");
            definitions_buffer.clear();
            for definition in definitions.split('/') {
                definitions_buffer.push_str(definition);
                definitions_buffer.push_str("/");
            }

            let entry = DictionaryEntry {
                pinyin: pinyin_buffer.clone(),
                definitions: definitions_buffer.clone(),
            };
            dictionary.insert(character, entry);
        }
    }

    dictionary
}