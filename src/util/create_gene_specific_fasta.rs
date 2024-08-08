use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufWriter, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

use crate::envs::error_handler as err;

// Read in one file
fn read_db(filename: &String) -> Vec<String> {
    let mut db: Vec<String> = Vec::new();
    // Open the file
    let reader = BufReader::new(File::open(filename).unwrap());
    // If the first character's ascii value is 0, erase it
    for line in reader.lines() {
        let mut line = line.unwrap_or_else(|_| "Unable to read db".to_string());
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

pub fn create_gene_specific_fasta(input_db: &str, gene_dir: &PathBuf, gene_list: &Vec<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {

    // Read names, amino acid and 3di sequences
    let names = read_db(&format!("{}_h", input_db));
    let aa_seqs = read_db(&input_db.to_string());
    let di_seqs = read_db(&format!("{}_ss", input_db));

    // Check the lengths are all same
    if names.len() != aa_seqs.len() || names.len() != di_seqs.len() {
        err::error(err::ERR_GENERAL, Some("Lengths of names, amino acid and 3di sequences in database are not same".to_string()));
    }
    // Create a hash map of names and sequences
    let mut aa_hash = HashMap::new();
    let mut di_hash = HashMap::new();
    for i in 0..names.len() {
        aa_hash.insert(names[i].clone(), aa_seqs[i].clone());
        di_hash.insert(names[i].clone(), di_seqs[i].clone());
    }

    // Process each gene
    let mut cnt = 0;
    for gene_path in gene_list {
        if let Some(gene_name) = gene_path.file_stem().and_then(|name| name.to_str()) {
            let gene_output_dir = Path::new(gene_dir).join(gene_name);
            fs::create_dir_all(&gene_output_dir)?;

            let aa_file_path = gene_output_dir.join("aa.fasta");
            let di_file_path = gene_output_dir.join("3di.fasta");
            let mut aa_file = BufWriter::new(File::create(aa_file_path)?);
            let mut di_file = BufWriter::new(File::create(di_file_path)?);

            let gene_file = File::open(gene_path)?;
            let reader = BufReader::new(gene_file);
            for line in reader.lines().filter_map(|l| l.ok()) {
                let parts: Vec<_> = line.split_whitespace().collect();
                if parts.len() == 2 {
                    if let Some(aa_seq) = aa_hash.get(parts[0]) {
                        writeln!(aa_file, ">{}\n{}", parts[1], aa_seq)?;
                    } else {
                        err::error(err::ERR_GENERAL, Some(format!("Sequence {} not found in the database", parts[1])));
                    }
                    if let Some(di_seq) = di_hash.get(parts[0]) {
                        writeln!(di_file, ">{}\n{}", parts[1], di_seq)?;
                    } else {
                        err::error(err::ERR_GENERAL, Some(format!("Sequence {} not found in the database", parts[1])));
                    }
                } else {
                    // Raise an error
                    err::error(err::ERR_GENERAL, Some(format!("Invalid line in gene mapping file: {}", line)));
                }
            }

            cnt += 1;
            print!("\rCreating gene specific fasta files {}/{}...", cnt, gene_list.len());
        }
    }
    println!(" Done\nGene specific fasta files prepared in: {}", gene_dir.display());

    Ok(())
}
