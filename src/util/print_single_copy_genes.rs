use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: {} <gene_to_spe_list> <m8_file> <output_dir> [<threshold>]", args[0]);
        std::process::exit(1);
    }

    let gene_to_spe_list = &args[1];
    let m8_file = &args[2];
    let output_dir = &args[3];
    let threshold: f64 = args.get(4).and_then(|t| t.parse().ok()).unwrap_or(0.8);

    let mut gene_to_spe: HashMap<String, HashSet<String>> = HashMap::new();
    let mut species_set: HashSet<String> = HashSet::new();

    // Read the gene to species list
    let file = File::open(gene_to_spe_list)?;
    let reader = BufReader::new(file);
    for line in reader.lines().filter_map(|l| l.ok()) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let af_gene = parts[0];
        let spe = parts[1].to_string();

        let gene = if af_gene.starts_with("AF-") {
            af_gene.split('-').nth(1).unwrap_or(af_gene)
        } else {
            af_gene
        }.to_string();

        gene_to_spe.entry(gene).or_insert_with(HashSet::new).insert(spe.clone());
        species_set.insert(spe);
    }

    let species_count = species_set.len() as f64;

    // Ensure the output directory exists
    fs::create_dir_all(output_dir)?;

    // Process the m8 file
    let file = File::open(m8_file)?;
    let reader = BufReader::new(file);
    let mut curr_query = None;
    let mut spe_dict: HashMap<String, HashSet<String>> = HashMap::new();

    for line in reader.lines().filter_map(|l| l.ok()) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let query = parts[0].to_string();
        let target = parts[1];

        let target_gene = if target.starts_with("AF-") {
            target.split('-').nth(1).unwrap_or(target)
        } else {
            target
        };

        if Some(&query) != curr_query.as_ref() {
            if let Some(q) = curr_query.take() {
                output_genes(&q, &spe_dict, species_count, threshold, output_dir)?;
            }
            curr_query = Some(query);
            spe_dict.clear();
        }

        if let Some(species) = gene_to_spe.get(target_gene) {
            for spe in species {
                spe_dict.entry(spe.to_string()).or_insert_with(HashSet::new).insert(target_gene.to_string());
            }
        }
    }

    if let Some(q) = curr_query {
        output_genes(&q, &spe_dict, species_count, threshold, output_dir)?;
    }

    Ok(())
}

fn output_genes(query: &str, spe_dict: &HashMap<String, HashSet<String>>, species_count: f64, threshold: f64, output_dir: &str) -> io::Result<()> {
    let single_copy = spe_dict.values().filter(|targets| targets.len() == 1).count() as f64;
    let single_copy_percent = single_copy / species_count;
    if single_copy_percent >= threshold {
        let output_path = Path::new(output_dir).join(format!("{}.txt", query.split('-').nth(1).unwrap_or(query)));
        let mut output_file = File::create(output_path)?;

        for (spe, targets) in spe_dict {
            if targets.len() == 1 {
                let target = targets.iter().next().unwrap();
                writeln!(output_file, "{}\t{}", target, spe)?;
            }
        }
    }
    Ok(())
}
