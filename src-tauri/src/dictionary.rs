use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct DictionaryEntry {
    pub pinyin: String,
    pub definitions: String,
}

pub fn parse_dictionary() -> HashMap<String, DictionaryEntry> {
    let mut dictionary = HashMap::new();
    let file = File::open("cedict_ts.u8").expect("Dictionary file not found");
    let reader = BufReader::with_capacity(1024 * 1024, file);

    let mut definitions_buffer = String::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let mut parts = line.split(' ');
            let character = parts.next().unwrap().to_string();
            let pinyin = parts.next().unwrap().to_string();
            let definitions = parts.collect::<Vec<&str>>().join(" ");
            definitions_buffer.clear();
            for definition in definitions.split('/') {
                definitions_buffer.push_str(definition);
            }

            let entry = DictionaryEntry {
                pinyin,
                definitions: definitions_buffer.clone(),
            };
            dictionary.insert(character, entry);
        }
    }

    dictionary
}