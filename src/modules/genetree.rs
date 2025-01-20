use std::fs;
use std::path::{Path, PathBuf};

use crate::envs::error_handler as err;
use crate::util::arg_parser::Args;
use crate::util::checkpoint as chkpnt;
use crate::util::message as msg;
use crate::modules::tree::{run_mafft, run_foldmason, run_iqtree};

pub fn run(args: &Args, bin: &crate::envs::variables::BinaryPaths) -> Result<(), Box<dyn std::error::Error>> {
    // Retrieve arguments
    let input = args.genetree_input.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("genetree - input".to_string())) });
    let names = args.genetree_names.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("genetree - names".to_string())) });
    let tree_builder = args.genetree_tree_builder.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("genetree - tree builder".to_string())) });
    let tree_options = args.genetree_tree_options.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("genetree - tree options".to_string())) });
    let refilter = args.genetree_refilter.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("genetree - refilter".to_string())) });
    let aligner = args.genetree_aligner.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("genetree - aligner".to_string())) });
    let aligner_options = args.genetree_aligner_options.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("genetree - aligner options".to_string())) });
    let threshold = args.genetree_threshold.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("genetree - threshold".to_string())) });
    let threads = crate::envs::variables::threads();

    // Check if the input directory exists
    if !Path::new(&input).exists() {
        err::error(err::ERR_GENERAL, Some("Input directory does not exist".to_string()));
    }

    // Check if input/fasta also exists
    if !Path::new(&format!("{}/fasta", input)).exists() {
        err::error(err::ERR_GENERAL, Some("Input directory does not contain core structure fasta directories".to_string()));
    }

    // Write the checkpoint file
    chkpnt::write_checkpoint(&format!("{}/genetree.chk", input), "0")?;

    // print out threads
    msg::println_message(&format!("Using {} threads", threads), 4);

    // Check tree builder
    let tree_builder_path = match &bin.get(&tree_builder) {
        Some(bin) => &bin.path,
        _none => { err::error(err::ERR_BINARY_NOT_FOUND, Some(tree_builder.clone())); }
    };
    
    let mut names_list = Vec::new();
    // If names is not empty, read in the names
    if !names.is_empty() {
        let names_path = Path::new(&names);
        if !names_path.exists() {
            err::error(err::ERR_GENERAL, Some("Names file does not exist".to_string()));
        }
        let names_content = fs::read_to_string(names_path)?;
        for name in names_content.lines() {
            names_list.push(name.to_string());
        }
    }
        
    // Prepare gene specific fasta directory
    let gene_fasta_dir = Path::new(&input).join("fasta");
    let mut gene_list = fs::read_dir(&gene_fasta_dir)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .collect::<Vec<PathBuf>>();

    // Filter gene_list by names_list if names_list is not empty
    let mut filtered_gene_list = Vec::new();
    if !names_list.is_empty() {
        for gene in gene_list.iter() {
            let gene_name = gene.file_stem().and_then(|name| name.to_str()).unwrap();
            if names_list.contains(&gene_name.to_string()) {
                filtered_gene_list.push(gene.clone());
            }
        }
        if filtered_gene_list.is_empty() {
            err::error(err::ERR_GENERAL, Some("No gene names matched".to_string()));
        }
        gene_list = filtered_gene_list;
    }

    if refilter {
        // Check aligner binary
        let aligner_path = match &bin.get(&aligner) {
            Some(bin) => &bin.path,
            _none => { err::error(err::ERR_BINARY_NOT_FOUND, Some(aligner.clone())); }
        };
        let aligner_options = aligner_options.unwrap_or_else(|| "".to_string());

        // Iterate through the gene_list and generate alignment
        if aligner == "mafft" || aligner == "mafft-linsi" {
            run_mafft(&aligner_path, &gene_fasta_dir, &gene_list, &aligner_options, threshold, threads)?;
        } else if aligner == "foldmason" {
            run_foldmason(&aligner_path, &gene_fasta_dir, &gene_list, &aligner_options, threshold, threads)?;
        } else {
            err::error(err::ERR_MODULE_NOT_IMPLEMENTED, Some("Need implementation".to_string()))
        }
    }

    // Make the vector of alignment files
    let msa_list = gene_list.iter()
        .map(|gene| {
            let gene_name = gene.file_stem().and_then(|name| name.to_str()).unwrap();
            gene_fasta_dir.join(gene_name).join(format!("{}.fa.filtered", gene_name)).display().to_string()
        })
        .collect::<Vec<_>>();

    // Iterate through the alignment files and generate gene specific phylogenetic trees
    msg::print_message(&format!("\rInferring gene specific phylogenetic trees {}/{}...", 0, msa_list.len()), 3);
    for (i, msa) in msa_list.iter().enumerate() {
        let msa_parent = Path::new(msa).parent().unwrap().display().to_string();
        // If there is existing iqtree output, delete it
        // First, find the files start with iqtree
        let iqtree_files = fs::read_dir(&msa_parent)?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| path.file_name().and_then(|name| name.to_str()).unwrap().starts_with("iqtree"))
            .collect::<Vec<PathBuf>>();
        // Then, delete the files
        for file in iqtree_files.iter() {
            fs::remove_file(file)?;
        }
        if tree_builder == "iqtree" {
            run_iqtree(&tree_builder_path, &msa_parent, &msa, &tree_options, threads)?;
        } else {
            // TODO: Implement other tree builders
            err::error(err::ERR_MODULE_NOT_IMPLEMENTED, Some("Need implementation".to_string()))
        }
        msg::print_message(&format!("\rInferring gene specific phylogenetic trees {}/{}...", i+1, msa_list.len()), 3);
    }
    msg::println_message(&"Done".to_string(), 3);

    // Write the checkpoint file
    chkpnt::write_checkpoint(&format!("{}/genetree.chk", input), "1")?;
    
    Ok(())
}