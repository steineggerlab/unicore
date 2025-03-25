use crate::seq::fasta_io::write_fasta;
use crate::util::message::print_message as mprint;
use crate::util::message::println_message as mprintln;
use crate::util::command as cmd;

use std::collections::HashMap;
use std::io::prelude::*;
use std::path::MAIN_SEPARATOR as SEP;
use std::process::Command as Cmd;

use reqwest;

fn download_table(path: &String) -> Result<(), Box<dyn std::error::Error>> {
    // create directory if not exists
    if std::path::Path::new(path).exists() {
        mprintln(&format!("Directory {} already exists.", path), 4);
    } else {
        mprintln(&format!("Creating directory {}...", path), 4);
        std::fs::create_dir_all(path)?;
    }

    // download the tables
    mprint(&"Downloading AFDB lookup tables (this may take a while)... 0.0%".to_string(), 3);
    for i in 0..256 {
        let hex = format!("{:02x}", i);
        let url = format!("https://unicore.steineggerlab.workers.dev/md5/{}.tsv.gz", hex);
        let file = format!("{}{}{}.tsv.gz", path, SEP, hex);
        let mut resp = reqwest::blocking::get(&url)?;
        let mut file = std::fs::File::create(&file)?;
        std::io::copy(&mut resp, &mut file)?;
        mprint(&format!("\rDownloading AFDB lookup tables (this may take a while)... {:.1}%", (i as f64 + 1.0) / 2.56), 3);
    }
    mprintln(&"\rDownloading AFDB lookup tables (this may take a while)... 100.0% Done".to_string(), 3);

    // decompress the tables
    mprint(&"Decompressing the tables... 0.0%".to_string(), 3);
    for i in 0..256 {
        let hex = format!("{:02x}", i);
        let file = format!("{}{}{}.tsv.gz", path, SEP, hex);
        cmd::run(Cmd::new("gzip").arg("-d").arg(&file));
        mprint(&format!("\rDecompressing the tables... {:.1}%", (i as f64 + 1.0) / 2.56), 3);
    }
    mprintln(&"\rDecompressing the tables... 100.0% Done".to_string(), 3);

    Ok(())
}

pub fn run(fasta_data: &HashMap<String, String>, afdb_local: &String, converted_aa: &String, converted_ss: &String, combined_aa: &String) -> Result<(), Box<dyn std::error::Error>> {
    // check if the directory is present
    let path = afdb_local.clone();
    let mut md5_path = format!("{}{}md5", path, SEP);
    if std::path::Path::new(&path).exists() && std::fs::File::open(&format!("{}{}00.tsv", path, SEP)).is_ok() { md5_path = path.clone(); }
    if std::fs::File::open(&format!("{}{}00.tsv", md5_path, SEP)).is_err() {
        mprintln(&"AFDB lookup tables not found.".to_string(), 0);
        mprint(&format!("Trying to download the tables to {} (~30GB). Continue? [y/n]: ", path), 0);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if input.trim().to_lowercase() != "y" {
            mprintln(&"Download cancelled. Aborting the program.".to_string(), 0);
            std::process::exit(0);
        }
        download_table(&md5_path)?;
    }

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
    mprint(&"Looking up the AFDB tables... 0.0%".to_string(), 3);
    for i in 0..256 {
        mprint(&format!("\rLooking up the AFDB tables... {:.1}%", (i as f64 + 1.0) / 2.56), 3);
        let hex = format!("{:02x}", i);
        if fasta_split[i].is_empty() {
            mprintln(&format!("\nNo sequences starting with *{}. Skipping...", hex), 4);
            continue;
        }
        let table = format!("{}{}{}.tsv", md5_path, SEP, hex);

        // load table to memory
        mprintln(&format!("\nLoading table for *{}...", hex), 4);
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
    mprintln(&"\rLooking up the AFDB tables... 100.0% Done".to_string(), 3);
    mprintln(&format!("{} sequences found from the lookup tables", conv), 3);
    mprintln(&format!("{} sequences not found and will be predicted", pred), 3);

    write_fasta(&converted_aa, &converted_aa_data, true)?;
    write_fasta(&converted_ss, &converted_ss_data, true)?;
    write_fasta(&combined_aa, &combined_data, false)?;

    Ok(())
}