use crate::seq::fasta_io::write_fasta;
use crate::util::message::print_message as mprint;
use crate::util::message::println_message as mprintln;

use std::collections::HashMap;
use std::io::prelude::*;
use std::path::MAIN_SEPARATOR as SEP;

fn download_table(_aa: &String) -> Result<String, Box<dyn std::error::Error>> {
    unimplemented!()
}

pub fn run(fasta_data: &HashMap<String, String>, afdb_local: &Option<String>, converted_aa: &String, converted_ss: &String, combined_aa: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut converted_aa_data: HashMap<String, String> = HashMap::new();
    let mut converted_ss_data: HashMap<String, String> = HashMap::new();
    let mut combined_data: HashMap<String, String> = HashMap::new();

    let mut fasta_split = vec![HashMap::<String, (String, String)>::new(); 256];
    // print(&"Splitting sequences by first two amino acids...".to_string(), 4);
    for (h, seq) in fasta_data {
        // add line feed to the end of the sequence
        let mut bytes = seq.clone().into_bytes(); bytes.push(10);
        let hash = format!("{:x}", md5::compute(bytes));
        let idx = usize::from_str_radix(&hash[..2], 16)?;
        fasta_split[idx].insert(h.clone(), (hash, seq.clone()));
    }

    let (mut conv, mut pred) = (0, 0);
    for i in 0..256 {
        mprint(&format!("\rLooking up AFDB tables... [{}/256]", i), 3);
        let hex = format!("{:02x}", i);
        if fasta_split[i].is_empty() {
            mprintln(&format!("No sequences starting with *{}. Skipping...", hex), 4);
            continue;
        }
        let table = match afdb_local {
            Some(path) => format!("{}{}{}.tsv", path, SEP, hex),
            None => download_table(&hex)?,
        };

        // load table to memory
        mprintln(&format!("Loading table for *{}...", hex), 4);
        let mut table_map: HashMap<String, String> = HashMap::new();
        let table_file = std::fs::File::open(table)?;
        let table_reader = std::io::BufReader::new(table_file);
        for line in table_reader.lines().filter_map(|l| l.ok()) {
            let mut split = line.split('\t');
            let key = split.next().unwrap().to_string();
            let value = split.next().unwrap().to_string();
            table_map.insert(key, value);
        }

        // convert sequences
        mprintln(&format!("Converting sequences starting with *{}...", hex), 4);
        for (h, (hash, seq)) in &fasta_split[i] {
            match table_map.get(hash) {
                Some(converted_seq) => {
                    converted_aa_data.insert(h.clone(), seq.clone());
                    converted_ss_data.insert(h.clone(), converted_seq.clone());
                    conv += 1;
                },
                None => {
                    combined_data.insert(h.clone(), seq.clone());
                    pred += 1;
                },
            }
        }
    }
    mprintln(&"\rLooking up AFDB tables... [256/256] Done".to_string(), 3);
    mprintln(&format!("{} sequences found from the lookup tables", conv), 3);
    mprintln(&format!("{} sequences not found and will be predicted", pred), 3);

    write_fasta(&converted_aa, &converted_aa_data)?;
    write_fasta(&converted_ss, &converted_ss_data)?;
    write_fasta(&combined_aa, &combined_data)?;

    Ok(())
}