use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

// Function that reads in a fasta file and outputs a hashmap of the sequences
pub fn read_fasta(file: &str) -> HashMap<String, String> {
    let file = File::open(file).expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut sequences = HashMap::new();
    let mut header = String::new();
    let mut sequence = String::new();
    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        if line.starts_with('>') {
            if !header.is_empty() {
                sequences.insert(header.clone(), sequence.clone());
                sequence.clear();
            }
            header = line[1..].to_string();
        } else {
            sequence.push_str(&line);
        }
    }
    sequences.insert(header, sequence);
    sequences
}