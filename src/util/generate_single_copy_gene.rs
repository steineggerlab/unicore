use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;

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

    let species_count = species_set.len() as f32;

    // Process the m8 file and output the statistics
    let mut output = File::create(format!("{}/copiness.tsv",output_dir))?;
    // Write out the first line
    writeln!(output, "Query\tMultipleCopyPercent\tSingleCopyPercent")?;
    let file = File::open(m8_file)?;
    let reader = BufReader::new(file);
    let mut curr_query: Option<String> = None;
    let mut spe_cnt: HashMap<String, i32> = HashMap::new();
    let mut gene2spe: HashMap<String, HashSet<String>> = HashMap::new();

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
                output_statistics_and_genes(&mut output, &q, &spe_cnt, &gene2spe, species_count, *threshold, output_dir)?;
            }
            curr_query = Some(query);
            spe_cnt.clear();
            gene2spe.clear();
        }

        if let Some(species) = gene_to_spe.get(target_gene) {
            for spe in species {
                *spe_cnt.entry(spe.to_string()).or_insert(0) += 1;
                gene2spe.entry(spe.to_string()).or_insert_with(HashSet::new).insert(target_gene.to_string());
            }
        }
    }

    if let Some(q) = curr_query {
        output_statistics_and_genes(&mut output, &q, &spe_cnt, &gene2spe, species_count, *threshold, output_dir)?;
    }

    Ok(())
}

fn output_statistics_and_genes<W: Write>(output: &mut W, query: &str, spe_cnt: &HashMap<String, i32>, gene2spe: &HashMap<String, HashSet<String>>, species_count: f32, threshold: f32, output_dir: &str) -> io::Result<()> {
    let single_copy = spe_cnt.values().filter(|&&count| count == 1).count() as f32;
    let multiple_copy = spe_cnt.len() as f32;

    let single_copy_percent = single_copy / species_count;
    let multiple_copy_percent = multiple_copy / species_count;

    // Write out to copiness.tsv
    writeln!(output, "{}\t{}\t{}", query, multiple_copy_percent, single_copy_percent)?;

    // Write out the gene list if it is considered as core gene
    if single_copy_percent >= threshold {
        let output_path = Path::new(output_dir).join(format!("{}.txt", query.split('-').nth(1).unwrap_or(query)));
        let mut output_file = BufWriter::new(File::create(output_path)?);

        for (spe, targets) in gene2spe {
            if targets.len() == 1 {
                let target = targets.iter().next().unwrap();
                writeln!(output_file, "{}\t{}", target, spe)?;
            }
        }
    }

    Ok(())
}
