use crate::envs::error_handler as err;
use crate::seq::fasta_io::write_fasta;
use crate::util::message::print_message as mprint;
use crate::util::message::println_message as mprintln;

use std::collections::HashMap;
use std::io::prelude::*;
use std::path::MAIN_SEPARATOR as SEP;

const AA: [char; 20] = ['A', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W', 'Y'];
fn aa_map(aa: char) -> usize {
    match aa {
        'A' => 0, 'C' => 1, 'D' => 2, 'E' => 3, 'F' => 4,
        'G' => 5, 'H' => 6, 'I' => 7, 'K' => 8, 'L' => 9,
        'M' => 10, 'N' => 11, 'P' => 12, 'Q' => 13, 'R' => 14,
        'S' => 15, 'T' => 16, 'V' => 17, 'W' => 18, 'Y' => 19,
        _ => 999 + aa as usize,
    }
}

fn download_table(_aa: &String) -> Result<String, Box<dyn std::error::Error>> {
    unimplemented!()
}

pub fn run(fasta_data: &HashMap<String, String>, afdb_local: &Option<String>, converted_aa: &String, converted_ss: &String, combined_aa: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut converted_aa_data: HashMap<String, String> = HashMap::new();
    let mut converted_ss_data: HashMap<String, String> = HashMap::new();
    let mut combined_data: HashMap<String, String> = HashMap::new();

    let mut fasta_split = vec![HashMap::<String, String>::new(); 400];
    // print(&"Splitting sequences by first two amino acids...".to_string(), 4);
    for (h, seq) in fasta_data {
        if seq.len() < 3 {
            err::warning(err::WRN_GENERAL, Some(format!("Short sequence detected: {} (length: {}).", h, seq.len())));
            combined_data.insert(h.clone(), seq.clone());
            continue;
        }

        let (i, j) = (aa_map(seq.chars().nth(1).unwrap()), aa_map(seq.chars().nth(2).unwrap()));
        if i > 999 || j > 999 {
            let invc = if i > 999 { i - 999 } else { j - 999 } as u8 as char;
            err::warning(err::WRN_GENERAL, Some(format!("Invalid amino acid {} detected from {}.", invc, h)));
            combined_data.insert(h.clone(), seq.clone());
            continue;
        }
        let idx = aa_map(seq.chars().skip(1).next().unwrap()) * 20 + aa_map(seq.chars().skip(2).next().unwrap());
        fasta_split[idx].insert(h.clone(), seq.clone());
    }

    for i in 0..400 {
        mprint(&format!("\rLooking up AFDB tables... [{}/400]", i), 3);
        let aa = format!("{}{}", AA[i / 20], AA[i % 20]);
        if fasta_split[i].is_empty() {
            mprintln(&format!("No sequences starting with *{}. Skipping...", aa), 4);
            continue;
        }
        let table = match afdb_local {
            Some(path) => format!("{}{}{}.tsv", path, SEP, aa),
            None => download_table(&aa)?,
        };

        // load table to memory
        mprintln(&format!("Loading table for *{}...", aa), 4);
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
        mprintln(&format!("Converting sequences starting with *{}...", aa), 4);
        for (h, seq) in &fasta_split[i] {
            match table_map.get(seq) {
                Some(converted_seq) => {
                    converted_aa_data.insert(h.clone(), seq.clone());
                    converted_ss_data.insert(h.clone(), converted_seq.clone());
                },
                None => {
                    combined_data.insert(h.clone(), seq.clone());
                },
            }
        }
    }
    mprintln(&"\rLooking up AFDB tables... [400/400] Done".to_string(), 3);

    write_fasta(&converted_aa, &converted_aa_data)?;
    write_fasta(&converted_ss, &converted_ss_data)?;
    write_fasta(&combined_aa, &combined_data)?;

    Ok(())
}