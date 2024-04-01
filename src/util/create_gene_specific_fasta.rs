use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use crate::util::read_fasta as read_fasta;

fn create_gene_specific_fasta(args: Vec<String>) -> io::Result<()> {
    if args.len() != 4 {
        eprintln!("Usage: {} <input_aa_fasta> <input_3di_fasta> <gene_dir>", args[0]);
        std::process::exit(1);
    }

    let input_aa_fasta = &args[1];
    let input_3di_fasta = &args[2];
    let gene_dir = &args[3];

    // Read amino acid fasta file
    let aa_hash = read_fasta::read_fasta(input_aa_fasta);

    // Read 3di fasta file
    let di_hash = read_fasta::read_fasta(input_3di_fasta);
    // Check if the sequences in the 3di fasta file exist in the aa fasta file
    // If not, throw an error
    for (name, _seq) in &di_hash {
        if let Some(_aa_seq) = aa_hash.get(name) {
            continue;
        } else {
            eprintln!("Error: Sequence {} not found in the amino acid fasta file", name);
            std::process::exit(1);
        }
    }

    // Process each gene
    let gene_list = fs::read_dir(gene_dir)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_file() && path.extension().map_or(false, |ext| ext == "txt"))
        .collect::<Vec<_>>();

    let mut cnt = 0;
    for gene_path in &gene_list {
        if let Some(gene_name) = gene_path.file_stem().and_then(|name| name.to_str()) {
            let gene_output_dir = Path::new(gene_dir).join(gene_name);
            fs::create_dir_all(&gene_output_dir)?;

            let aa_file_path = gene_output_dir.join("aa.fasta");
            let di_file_path = gene_output_dir.join("3di.fasta");
            let mut aa_file = File::create(aa_file_path)?;
            let mut di_file = File::create(di_file_path)?;

            let gene_file = File::open(gene_path)?;
            let reader = BufReader::new(gene_file);
            for line in reader.lines().filter_map(|l| l.ok()) {
                let parts: Vec<_> = line.split_whitespace().collect();
                if parts.len() == 2 {
                    if let Some(aa_seq) = aa_hash.get(parts[0]) {
                        writeln!(aa_file, ">{}", parts[1])?;
                        writeln!(aa_file, "{}", aa_seq)?;
                    } else {
                        eprintln!("Error: Sequence {} not found in the amino acid fasta file", parts[0]);
                        std::process::exit(1);
                    }
                    if let Some(di_seq) = di_hash.get(parts[0]) {
                        writeln!(di_file, ">{}", parts[1])?;
                        writeln!(di_file, "{}", di_seq)?;
                    } else {
                        eprintln!("Error: Sequence {} not found in the 3di fasta file", parts[1]);
                        std::process::exit(1);
                    }
                } else {
                    // Raise an error
                    eprintln!("Error: Invalid line in gene file: {}", line);
                }
            }

            cnt += 1;
            print!("\rCreated gene specific fasta files for {}/{}: {}/{}", gene_dir, gene_name, cnt, gene_list.len());
        }
    }
    println!("{} done!", gene_dir);

    Ok(())
}
