use crate::envs::variables as var;
use crate::envs::error_handler as err;
use crate::util::arg_parser::Args;
use crate::util::checkpoint as chkpnt;
use crate::util::message as msg;

use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;

fn profile(tsv_file: &str, mapping: &str, output_dir: &str, threshold: usize, print_copiness: bool) -> io::Result<()> {
    let mut gene_to_spe: HashMap<String, HashSet<String>> = HashMap::new();
    let mut species_set: HashSet<String> = HashSet::new();

    // Read the gene to species list
    let file = File::open(mapping)?;
    let reader = BufReader::new(file);
    for line in reader.lines().filter_map(|l| l.ok()) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let af_gene = parts[0].to_string();
        let spe = parts[1].to_string();

        gene_to_spe.entry(af_gene).or_insert_with(HashSet::new).insert(spe.clone());
        species_set.insert(spe);
    }

    let species_count = species_set.len();

    // Process the m8 file and output the statistics
    let mut output = if print_copiness { Some(File::create(format!("{}/copiness.tsv", output_dir))?) } else { None };
    // Write out the first line
    if let Some(output) = output.as_mut() {
        writeln!(output, "Query\tMultipleCopyPercent\tSingleCopyPercent")?;
    }
    let file = File::open(tsv_file)?;
    let reader = BufReader::new(file);
    let mut curr_query: Option<String> = None;
    let mut spe_cnt: HashMap<String, i32> = HashMap::new();
    let mut gene2spe: HashMap<String, HashSet<String>> = HashMap::new();
    let mut spe_full_cnt: HashMap<String, i32> = HashMap::new();
    // Initialize spe_full_cnt with species_set
    for spe in species_set {
        spe_full_cnt.insert(spe, 0);
    }
    let (mut total_cnt, mut core_cnt) = (0, 0);

    msg::print_message(&"Profiling the taxonomic distribution of the genes...".to_string(), 3);
    for line in reader.lines().filter_map(|l| l.ok()) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let query = parts[0].to_string();
        let target = parts[1];

        if Some(&query) != curr_query.as_ref() {
            if let Some(q) = curr_query.take() {
                total_cnt += 1;
                let is_core = output_statistics_and_genes(&mut output, &q, &spe_cnt, &gene2spe, species_count, threshold, output_dir)
                    .expect("Failed to write out the statistics and genes");
                if is_core {
                    core_cnt += 1;
                    // Update the full count if the gene is considered as core
                    for (spe, count) in spe_cnt.iter() {
                        if *count == 1 {
                            if let Some(full_count) = spe_full_cnt.get_mut(spe) {
                                *full_count += 1;
                            } else {
                                err::error(err::ERR_GENERAL, Some(format!("Species {} not found in the mapping file", spe)));
                            }
                        }
                    }
                }
            }
            curr_query = Some(query);
            spe_cnt.clear();
            gene2spe.clear();
        }

        if let Some(species) = gene_to_spe.get(target) {
            for spe in species {
                *spe_cnt.entry(spe.to_string()).or_insert(0) += 1;
                gene2spe.entry(spe.to_string()).or_insert_with(HashSet::new).insert(target.to_string());
            }
        }
    }

    if let Some(q) = curr_query {
        total_cnt += 1;
        let is_core = output_statistics_and_genes(&mut output, &q, &spe_cnt, &gene2spe, species_count, threshold, output_dir)
            .expect("Failed to write out the statistics and genes");
        if is_core {
            core_cnt += 1;
            // Update the full count if the gene is considered as core
            for (spe, count) in spe_cnt.iter() {
                if *count == 1 {
                    if let Some(full_count) = spe_full_cnt.get_mut(spe) {
                        *full_count += 1;
                    } else {
                        err::error(err::ERR_GENERAL, Some(format!("Species {} not found in the mapping file", spe)));
                    }
                }
            }
        }
    }
    
    msg::println_message(&" Done".to_string(), 3);
    msg::println_message(&format!("{} structural core genes found from {} candidates", core_cnt, total_cnt), 3);
    
    // Check if there is any species that has less than 50% of the core genes
    let core_threshold = (core_cnt + 1) / 2;
    for (spe, count) in spe_full_cnt {
        if count < core_threshold {
            err::warning(err::WRN_GENERAL, Some(format!("Species {} has only {} core genes out of {} core genes", spe, count, core_cnt)));
        }
    }
    
    Ok(())
}

fn output_statistics_and_genes<W: Write>(output: &mut Option<W>, query: &str, spe_cnt: &HashMap<String, i32>, gene2spe: &HashMap<String, HashSet<String>>, species_count: usize, threshold: usize, output_dir: &str) -> io::Result<bool> {
    let single_copy = spe_cnt.values().filter(|&&count| count == 1).count();
    let multiple_copy = spe_cnt.len();

    let single_copy_percent = single_copy as f64 * 100.0 / species_count as f64;
    let multiple_copy_percent = multiple_copy as f64 * 100.0 / species_count as f64;

    // Write out to copiness.tsv
    msg::println_message(&format!("Gene {} reported {:.2}% single copy and {:.2}% multiple copy", query, single_copy_percent, multiple_copy_percent), 4);
    if let Some(output) = output.as_mut() {
        writeln!(output, "{}\t{}\t{}", query, multiple_copy_percent, single_copy_percent)?;
    }

    // Write out the gene list if it is considered as core gene
    if single_copy * 100 >= threshold * species_count {
        let output_path = Path::new(output_dir).join(format!("{}.txt", query.split('-').nth(1).unwrap_or(query)));
        let mut output_file = BufWriter::new(File::create(output_path)?);

        for (spe, targets) in gene2spe {
            if targets.len() == 1 {
                let target = targets.iter().next().unwrap();
                writeln!(output_file, "{}\t{}", target, spe)?;
            }
        }
        output_file.flush()?;
        Ok(true)
    } else { Ok(false) }
}

pub fn run(args: &Args, _: &var::BinaryPaths) -> Result<(), Box<dyn std::error::Error>> {
    // Retrieve arguments
    let input_db = args.profile_input_db.clone().unwrap_or_else(|| { crate::envs::error_handler::error(crate::envs::error_handler::ERR_ARGPARSE, Some("profile - input".to_string())); });
    let input_tsv = args.profile_input_tsv.clone().unwrap_or_else(|| { crate::envs::error_handler::error(crate::envs::error_handler::ERR_ARGPARSE, Some("profile - mapping".to_string())); });
    let output = args.profile_output.clone().unwrap_or_else(|| { crate::envs::error_handler::error(crate::envs::error_handler::ERR_ARGPARSE, Some("profile - output".to_string())); });
    let threshold = args.profile_threshold.unwrap_or_else(|| { crate::envs::error_handler::error(crate::envs::error_handler::ERR_ARGPARSE, Some("profile - threshold".to_string())); });
    let print_copiness = args.profile_print_copiness.unwrap_or_else(|| { crate::envs::error_handler::error(crate::envs::error_handler::ERR_ARGPARSE, Some("profile - print_copiness".to_string())); });

    // If there is no output directory, make one
    if !Path::new(&output).exists() {
        fs::create_dir_all(&output)?;
    }

    // Write the checkpoint file
    chkpnt::write_checkpoint(&format!("{}/profile.chk", output), "0")?;

    let mapping = format!("{}.map", input_db);
    profile(&input_tsv, &mapping, &output, threshold, print_copiness)?;

    // Write the checkpoint file
    chkpnt::write_checkpoint(&format!("{}/profile.chk", output), "1")?;
    
    Ok(())
}