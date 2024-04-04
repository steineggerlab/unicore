use std::fs::File;
use std::io::{BufReader, BufRead};

// Read in one file
pub fn read_db(filename: &String) -> Vec<String> {
    let mut db: Vec<String> = Vec::new();
    // Open the file
    let reader = BufReader::new(File::open(filename).unwrap());
    // If the first character's ascii value is 0, erase it
    for line in reader.lines() {
        let mut line = line.unwrap_or_else(|e| "Unable to read db".to_string());
        if line.chars().next().unwrap() as u8 == 0 {
            line.remove(0);
        }
        // Push if the length is greater than 0
        if line.len() > 0 {
            db.push(line);
        }
    }
    db
}