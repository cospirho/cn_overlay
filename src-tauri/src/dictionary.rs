use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct DictionaryEntry {
    pub pinyin: String,
    pub definitions: String,
}

pub fn parse_dictionary() -> HashMap<String, DictionaryEntry> {
    let mut dictionary = HashMap::new();
    let file = File::open("./dict/dictionary.tsv").expect("Dictionary file not found");
    let reader = BufReader::with_capacity(1024 * 1024, file);

    for line in reader.lines() {
        if let Ok(line) = line {
            let mut parts = line.split(['\t']);
            let character = match parts.next() {
                Some(val) => val.to_string(),
                None => {
                    eprintln!("Unable to read dictionary line: {}\n\tMissing first element", line);
                    continue;
                }
            };
    
            let pinyin = match parts.next() {
                Some(val) => {
                    val.to_string()
                }
                None => {
                    eprintln!("Unable to read dictionary line: {}\n\tMissing second element", line);
                    continue;
                }
            };

            let definitions = match parts.next() {
                Some(val) => {
                    val.to_string()
                }
                None => {
                    eprintln!("Unable to read dictionary line: {}\n\tMissing second element", line);
                    continue;
                }
            };

            let entry = DictionaryEntry {
                pinyin: pinyin,
                definitions: definitions
            };
            dictionary.insert(character, entry);
        }
    }

    dictionary
}