use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::io::Write;
use std::process::Command;

use crate::envs::error_handler as err;
use crate::envs::variables as var;
use crate::util::arg_parser::Args;
use crate::util::command as cmd;
use crate::util::checkpoint as chkpnt;
use crate::util::message as msg;
use crate::seq::create_gene_specific_fasta as gsf;
use crate::seq::combine_fasta as cf;
use crate::seq::fasta_io as fasta;

pub fn run(args: &Args, bin: &crate::envs::variables::BinaryPaths) -> Result<(), Box<dyn std::error::Error>> {
    // Retrieve arguments
    let db = args.tree_db.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("tree - proteome_db".to_string())); });
    let input = args.tree_input.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("tree - input".to_string())); });
    let output = args.tree_output.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("tree - output".to_string())); });
    let aligner = args.tree_aligner.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("tree - aligner".to_string())); });
    let no_inference = args.tree_no_inference.unwrap_or(false);
    let tree_builder = args.tree_tree_builder.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("tree - tree_builder".to_string())); });
    let aligner_options = args.tree_aligner_options.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("tree - aligner_options".to_string())); });
    let tree_options = args.tree_tree_options.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("tree - tree_options".to_string())); });
    let threshold = args.tree_threshold.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("tree - threshold".to_string())); });
    let threads = crate::envs::variables::threads();

    // If there is no output directory, make one
    if !Path::new(&output).exists() {
        fs::create_dir_all(&output)?;
    }

    // Write the checkpoint file
    chkpnt::write_checkpoint(&format!("{}/tree.chk", output), "0")?;

    // print out threads
    msg::println_message(&format!("Using {} threads", threads), 4);

    // Check aligner binary
    let aligner_path = match &bin.get(&aligner) {
        Some(bin) => &bin.path,
        _none => { err::error(err::ERR_BINARY_NOT_FOUND, Some(aligner.clone())); }
    };
    let aligner_options = aligner_options.unwrap_or_else(|| "".to_string());

    // Check tree builder
    let tree_builder_path = match &bin.get(&tree_builder) {
        Some(bin) => &bin.path,
        _none => { err::error(err::ERR_BINARY_NOT_FOUND, Some(tree_builder.clone())); }
    };
    
    let combined_fasta = Path::new(&output).join("combined.fasta");
    // Check if combined fasta exists
    // If it does, skip the alignment step
    if !Path::new(&combined_fasta).exists() {
        // Prepare gene specific fasta directory
        let gene_fasta_dir = Path::new(&output).join("fasta");
        fs::create_dir_all(&gene_fasta_dir)?;

        // Get the gene_list
        let gene_list = fs::read_dir(&input)?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| path.is_file() && path.extension().map_or(false, |ext| ext == "txt"))
            .collect::<Vec<_>>();
        // Create gene specific fasta
        gsf::create_gene_specific_fasta(&db, &gene_fasta_dir, &gene_list)?;
        
        // Build foldseek db
        let foldseek_path = match &bin.get("foldseek") {
            Some(bin) => &bin.path,
            _none => { err::error(err::ERR_BINARY_NOT_FOUND, Some("foldseek".to_string())); }
        };
        // Iterate through the gene_list and build foldseek db
        // Only need to build foldseek db when the aligner is foldmason
        if aligner == "foldmason" {
            let foldseek_verbosity = (match var::verbosity() { 4 => 3, 3 => 2, _ => var::verbosity() }).to_string();
            for (i, gene) in gene_list.iter().enumerate() {
                if let Some(gene_name) = gene.file_stem().and_then(|name| name.to_str()) {
                    let gene_dir = gene_fasta_dir.join(gene_name);
                    // amino acid db
                    let mut cmd = Command::new(foldseek_path);
                    let aa_fasta = gene_dir.join("aa.fasta");
                    let aa_db = gene_dir.join(format!("{}_db", gene_name).as_str());
                    let mut cmd_args = vec!["base:createdb",
                                        aa_fasta.to_str().unwrap(),
                                        aa_db.to_str().unwrap(),
                                        "--shuffle", "0"];
                    cmd_args.push("-v"); cmd_args.push(foldseek_verbosity.as_str());
                    let mut cmd = cmd.args(cmd_args);
                    cmd::run(&mut cmd);
                    // 3Di db
                    let mut cmd = Command::new(foldseek_path);
                    let di_fasta = gene_dir.join("3di.fasta");
                    let di_db = gene_dir.join(format!("{}_db_ss", gene_name).as_str());
                    let mut cmd_args = vec![
                        "base:createdb",
                        di_fasta.to_str().unwrap(),
                        di_db.to_str().unwrap(),
                        "--shuffle", "0"];
                    cmd_args.push("-v"); cmd_args.push(foldseek_verbosity.as_str());
                    let mut cmd = cmd.args(cmd_args);
                    cmd::run(&mut cmd);
                }
                msg::print_message(&format!("\rBuilding foldseek databases {}/{}...", i + 1, gene_list.len()), 3);
            }
            msg::println_message(&" Done".to_string(), 3);
        }

        // Iterate through the gene_list and generate alignment
        if aligner == "mafft" || aligner == "mafft-linsi" {
            run_mafft(&aligner_path, &gene_fasta_dir, &gene_list, &aligner_options, threshold, threads)?;
        } else if aligner == "foldmason" {
            run_foldmason(&aligner_path, &gene_fasta_dir, &gene_list, &aligner_options, threshold, threads)?;
        } else {
            err::error(err::ERR_MODULE_NOT_IMPLEMENTED, Some("Need implementation".to_string()))
        }

        // Make the vector of alignment files
        let msa_list = gene_list.iter()
            .map(|gene| {
                let gene_name = gene.file_stem().and_then(|name| name.to_str()).unwrap();
                gene_fasta_dir.join(gene_name).join(format!("{}.fa.filtered", gene_name)).display().to_string()
            })
            .collect::<Vec<_>>();

        // Combine alignment
        cf::combine_fasta(&msa_list, &output)?;

        if no_inference {
            return Ok(());
        }
    } else {
        msg::println_message(&"Concatenated alignment file already exists".to_string(), 3);
    }

    // Define tree options
    let tree_options = if tree_options.is_some() {
        tree_options.unwrap()
    } else {
        if tree_builder == "iqtree" { "-m JTT+F+I+G -B 1000".to_string() }
        else if tree_builder == "raxml" { "-m PROTCATJTT -p 12345 -x 12345 -f a -N 1000".to_string() }
        else if tree_builder == "fasttree" { "-gamma -boot 1000".to_string() }
        else { err::error(err::ERR_GENERAL, Some("Unrecognized tree builder".to_string())); }
    };

    // Build tree
    msg::print_message(&"Inferring phylogenetic tree...".to_string(), 3);
    if tree_builder == "iqtree" {
        run_iqtree(&tree_builder_path, &output, &combined_fasta.display().to_string(), &tree_options, threads)?;
    } else {
        // TODO: Implement other tree building methods
        err::error(err::ERR_MODULE_NOT_IMPLEMENTED, Some("Need implementation".to_string()))
    }
    msg::println_message(&" Done".to_string(), 3);

    // Write the checkpoint file
    chkpnt::write_checkpoint(&format!("{}/tree.chk", output), "1")?;

    Ok(())
}

pub fn run_mafft(mafft_path: &String, parent: &Path, gene_list: &Vec<PathBuf>, mafft_options: &String, threshold: usize, threads: usize) -> Result<(), Box<dyn std::error::Error>> {
    for (i, gene) in gene_list.iter().enumerate() {
        if let Some(gene_name) = gene.file_stem().and_then(|name| name.to_str()) {
            let gene_dir = parent.join(gene_name);
            let mut cmd = Command::new(mafft_path);
            // parse mafft_options into vector
            let mut cmd_args = mafft_options.split_whitespace().collect::<Vec<&str>>();
            // Include threads option
            let threads_copy = threads.to_string();
            if !cmd_args.contains(&"--thread") {
                cmd_args.push("--thread");
                cmd_args.push(threads_copy.as_str());
            }
            cmd_args.push("--anysymbol");
            if var::verbosity() < 4 { cmd_args.push("--quiet"); }

            // add input and output
            let aa_fasta = gene_dir.join("aa.fasta");
            cmd_args.push(aa_fasta.to_str().unwrap());
            let msa_fasta = gene_dir.join(format!("{}.fa", gene_name));
            let msa_file = fs::File::create(&msa_fasta)?;
            let mut cmd = cmd.args(cmd_args).stdout(msa_file);

            cmd::run(&mut cmd);

            // output_msa is msa_fasta + ".filtered"
            let output_msa = gene_dir.join(format!("{}.fa.filtered", gene_name)).display().to_string();
            filter_msa(&msa_fasta.display().to_string(), &output_msa, threshold)?;
        }
        msg::print_message(&format!("\rAligning genes {}/{}...", i + 1, gene_list.len()), 3);
    }
    msg::println_message(&" Done".to_string(), 3);
    Ok(())
}

pub fn run_foldmason(foldmason_path: &String, parent: &Path, gene_list: &Vec<PathBuf>, foldmason_options: &String, threshold: usize, threads: usize) -> Result<(), Box<dyn std::error::Error>> {
    let foldseek_verbosity = (match var::verbosity() { 4 => 3, 3 => 2, _ => var::verbosity() }).to_string();
    for (i, gene) in gene_list.iter().enumerate() {
        if let Some(gene_name) = gene.file_stem().and_then(|name| name.to_str()) {
            let gene_dir = parent.join(gene_name);
            let mut cmd = Command::new(foldmason_path);
            let db = gene_dir.join(format!("{}_db", gene_name));
            let msa_fasta = gene_dir.join(gene_name);
            let mut cmd_args = vec!["structuremsa",
                            db.to_str().unwrap(),
                            msa_fasta.to_str().unwrap()];
            cmd_args.push("-v"); cmd_args.push(foldseek_verbosity.as_str());
            // parse foldmason_options into vector
            let mut cmd_options = foldmason_options.split_whitespace().collect::<Vec<&str>>();
            let threads_copy = threads.to_string();
            if !cmd_options.contains(&"--threads") {
                cmd_args.push("--threads");
                cmd_args.push(threads_copy.as_str());
            }
            cmd_args.append(&mut cmd_options);
            let mut cmd = cmd.args(cmd_args);
            cmd::run(&mut cmd);
            // output_msa is msa_fasta + ".filtered"
            let output_msa = gene_dir.join(format!("{}.fa.filtered", gene_name)).display().to_string();
            filter_msa(&(msa_fasta.display().to_string() + "_aa.fa"), &output_msa, threshold)?;
        }
        msg::print_message(&format!("\rAligning genes {}/{}...", i + 1, gene_list.len()), 3);
    }
    msg::println_message(&" Done".to_string(), 3);
    Ok(())
}

pub fn run_iqtree(iqtree_path: &String, output_dir: &String, msa_fasta: &String, iqtree_options: &String, threads: usize) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new(iqtree_path);
    let mut cmd_options = iqtree_options.split_whitespace().collect::<Vec<&str>>();
    // If there is "--prefix" in the option
    let mut cmd_args = vec!["-s", msa_fasta];
    let output_file = Path::new(output_dir).join("iqtree").display().to_string();
    if !cmd_options.contains(&"--prefix"){
        cmd_args.push("--prefix");
        cmd_args.push(output_file.as_str());
    }
    // Include threads option
    let threads_copy = threads.to_string();
    if !cmd_options.contains(&"-T"){
        cmd_args.push("-T");
        cmd_args.push(threads_copy.as_str());
    }

    cmd_args.push("--quiet"); // TODO: verbose option should disable this

    // parse iqtree_options into vector
    cmd_args.append(&mut cmd_options);
    let mut cmd = cmd.args(cmd_args);
    cmd::run(&mut cmd);
    Ok(())
}

// Only write columns that have >=threshold coverage
fn filter_msa(input_msa: &String, output_msa: &String, threshold: usize) -> Result<(), Box<dyn std::error::Error>> {
    // Read in fasta file
    let msa: HashMap<String, String> = fasta::read_fasta(input_msa);
    let seq_num = msa.len();

    // Iterate through the sequences and fill non_gap_cnt
    let mut non_gap_cnt: Vec<usize> = vec![0; msa.values().next().unwrap().len()];
    for seq in msa.values() {
        for (i, c) in seq.chars().enumerate() {
            if c != '-' {
                non_gap_cnt[i] += 1;
            }
        }
    }

    // Indices of non_gap_cnt >= threshold
    let indices: Vec<usize> = non_gap_cnt.iter().enumerate()
        .filter(|(_, &x)| x * 100 >= threshold * seq_num)
        .map(|(i, _)| i)
        .collect();
    // Write the filtered MSA
    let file = fs::File::create(output_msa)?;
    let mut file_writer = std::io::BufWriter::new(file);
    for (header, sequence) in msa.iter() {
        writeln!(file_writer, ">{}", header)?;
        for i in indices.iter() {
            write!(file_writer, "{}", sequence.chars().nth(*i).unwrap())?;
        }
        writeln!(file_writer, "")?;
    }
    file_writer.flush()?;
    Ok(())
}