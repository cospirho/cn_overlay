// converts the cedict_ts.u8 file to a tsv file
// with character, pinyin, definition
// just makes it easier to parse and add new words
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use regex::Regex;

fn main() {
    let input_file = File::open("cedict_ts.u8").expect("Failed to open input file");
    let reader = BufReader::new(input_file);

    let mut dictionary = String::new();
    let re = Regex::new(r"(.*) (.*) \[(.*)\] /(.*)/").expect("Failed to create regex");

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.starts_with("#") {
                continue;
            }

            if let Some(captures) = re.captures(&line) {
                let entry = format!(
                    "{}\t{}\t{}\t\n",
                    captures.get(2).unwrap().as_str(),
                    captures.get(3).unwrap().as_str(),
                    captures.get(4).unwrap().as_str()
                );
                dictionary.push_str(&entry);
            }
        }
    }

    let mut output_file = File::create("dictionary.tsv").expect("Failed to create output file");
    output_file.write_all(dictionary.as_bytes()).expect("Failed to write to output file");
}
