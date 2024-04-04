use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

pub fn profile(m8_file: &str, mapping: &str, output_dir: &str, threshold: &f32, print_copiness: &bool) -> io::Result<()> {
    let mut gene_to_spe: HashMap<String, HashSet<String>> = HashMap::new();
    let mut species_set: HashSet<String> = HashSet::new();

    // Read the gene to species list
    let file = File::open(mapping)?;
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

    // Process the m8 file and output the statistics
    let mut output = File::create(format!("{}/copiness.tsv",output_dir))?;
    // Write out the first line
    writeln!(output, "Query\tMultipleCopyPercent\tSingleCopyPercent")?;
    let file = File::open(m8_file)?;
    let reader = BufReader::new(file);
    let mut curr_query: Option<String> = None;
    let mut spe_dict: HashMap<String, i32> = HashMap::new();

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
                output_statistics(&mut output, &q, &spe_dict, species_count)?;
            }
            curr_query = Some(query);
            spe_dict.clear();
        }

        if let Some(species) = gene_to_spe.get(target_gene) {
            for spe in species {
                *spe_dict.entry(spe.to_string()).or_insert(0) += 1;
            }
        }
    }

    if let Some(q) = curr_query {
        output_statistics(&mut output, &q, &spe_dict, species_count)?;
    }

    Ok(())
}

fn output_statistics<W: Write>(output: &mut W, query: &str, spe_dict: &HashMap<String, i32>, species_count: f64) -> io::Result<()> {
    let single_copy = spe_dict.values().filter(|&&count| count == 1).count() as f64;
    let multiple_copy = spe_dict.len() as f64;

    let single_copy_percent = single_copy / species_count;
    let multiple_copy_percent = multiple_copy / species_count;

    writeln!(output, "{}\t{}\t{}", query, multiple_copy_percent, single_copy_percent)
}
