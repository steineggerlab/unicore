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
    let msa_for_tree = args.tree_msa_for_tree.unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("tree - msa_for_tree".to_string())); });
    let rate_matrix_3di = args.tree_rate_matrix_3di.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("tree - rate_matrix_3di".to_string())); });
    let tree_builder = args.tree_tree_builder.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("tree - tree_builder".to_string())); });
    let aligner_options = args.tree_aligner_options.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("tree - aligner_options".to_string())); });
    let tree_options = args.tree_tree_options.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("tree - tree_options".to_string())); });
    let threshold = args.tree_threshold.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("tree - threshold".to_string())); });
    let threads = crate::envs::variables::threads();

    // If there is no output directory, make one
    if !Path::new(&output).exists() {
        fs::create_dir_all(&output)?;
    }

    // If msa_for_tree is not 0, aligner should be foldmason
    if msa_for_tree != 0 && aligner != "foldmason" {
        err::error(err::ERR_GENERAL, Some("If --msa-for-tree is set to 1 or 2, the aligner must be foldmason".to_string()));
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

    // Extract the rate matrix name from rate_matrix_3di and also the directory containing it
    let rate_matrix_3di_path = Path::new(&rate_matrix_3di);
    let rate_matrix_3di_name = rate_matrix_3di_path.file_stem().and_then(|name| name.to_str()).unwrap_or_else(|| { err::error(err::ERR_GENERAL, Some("Invalid rate matrix file path".to_string())); });
    let rate_matrix_3di_dir = rate_matrix_3di_path.parent().and_then(|dir| dir.to_str()).unwrap_or_else(|| { err::error(err::ERR_GENERAL, Some("Invalid rate matrix file path".to_string())); });
    // rate_matrix_3di_dir + "matrices.nex"
    let matrix_dir = Path::new(rate_matrix_3di_dir).join("matrices.nex").display().to_string();
    
    let combined_fasta = Path::new(&output).join("combined.fasta");
    let combined_fasta_partition = Path::new(&output).join("combined.fasta.partitions").display().to_string();
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
            err::error(err::ERR_GENERAL, Some("Unrecognized aligner".to_string()));
        }

        // Make the vector of alignment files
        let mut msa_list = gene_list.iter()
            .map(|gene| {
                let gene_name = gene.file_stem().and_then(|name| name.to_str()).unwrap();
                gene_fasta_dir.join(gene_name).join(format!("{}.fa.filtered", gene_name)).display().to_string()
            })
            .collect::<Vec<_>>();
        
        // If msa_for_tree is 2, also include the 3di filtered msa
        if msa_for_tree == 1 {
            // Change the msa_list to include the 3di filtered msa
            msa_list = gene_list.iter()
                .map(|gene| {
                    let gene_name = gene.file_stem().and_then(|name| name.to_str()).unwrap();
                    gene_fasta_dir.join(gene_name).join(format!("{}.fa.filtered.3di", gene_name)).display().to_string()
                })
                .collect::<Vec<_>>();
        } else if msa_for_tree == 2 {
            // Add the 3di filtered msa to the msa_list
            let mut msa_list_3di = gene_list.iter()
                .map(|gene| {
                    let gene_name = gene.file_stem().and_then(|name| name.to_str()).unwrap();
                    gene_fasta_dir.join(gene_name).join(format!("{}.fa.filtered.3di", gene_name)).display().to_string()
                })
                .collect::<Vec<_>>();
            msa_list.append(&mut msa_list_3di);
        }

        // Combine alignment
        cf::combine_fasta(&msa_list, &output, &msa_for_tree, &tree_builder, &rate_matrix_3di)?;

        if no_inference {
            return Ok(());
        }
    } else {
        msg::println_message(&format!("Concatenated alignment file {} already exists, skipping alignment step", combined_fasta.display().to_string()), 3);
    }

    // Define tree options
    let tree_options = if tree_options.is_some() {
        tree_options.unwrap()
    } else if msa_for_tree == 0 {
        if tree_builder == "iqtree" { "-m JTT+F+I+G -B 1000".to_string() }
        else if tree_builder == "raxml-ng" { "--model JTT+F+I+G --seed 12345 --all --tree pars{90},rand{10}".to_string() }
        else if tree_builder == "fasttree" { "-gamma -boot 1000".to_string() }
        else { err::error(err::ERR_GENERAL, Some("Unrecognized tree builder".to_string())); }
    } else if msa_for_tree == 1 {
        // Only 3Di
        if tree_builder == "iqtree" { format!("-m {}+F+I+G -mdef {} -B 1000", rate_matrix_3di_name, matrix_dir) }
        else if tree_builder == "raxml-ng" { format!("--model PROTGTR{{{}}}+F+I+G --seed 12345 --all --tree pars{{90}},rand{{10}}", rate_matrix_3di) }
        else if tree_builder == "fasttree" { err::error(err::ERR_GENERAL, Some("Support for FastTree is not yet implemented".to_string())); }
        // else if tree_builder == "fasttree" { format!("-matrix {} -boot 1000", rate_matrix_3di) }
        else { err::error(err::ERR_GENERAL, Some("Unrecognized tree builder".to_string())); }
    } else if msa_for_tree == 2 {
        // Partition file
        if tree_builder == "iqtree" { format!("-q {} -mdef {} -B 1000", combined_fasta_partition, matrix_dir) }
        else if tree_builder == "raxml-ng" { format!("--model {} --seed 12345 --all --tree pars{{90}},rand{{10}}", combined_fasta_partition) }
        else if tree_builder == "fasttree" { err::error(err::ERR_GENERAL, Some("Partition method is not supported in FastTree".to_string())) }
        else { err::error(err::ERR_GENERAL, Some("Unrecognized tree builder".to_string())); }
    } else {
        err::error(err::ERR_GENERAL, Some("Invalid value for msa_for_tree".to_string()));
    };

    // Build tree
    msg::print_message(&"Inferring phylogenetic tree...".to_string(), 3);
    if tree_builder == "iqtree" {
        run_iqtree(&tree_builder_path, &output, &combined_fasta.display().to_string(), &tree_options, threads)?;
    } else if tree_builder == "raxml-ng" {
        run_raxml(&tree_builder_path, &output, &combined_fasta.display().to_string(), &tree_options, threads)?;
    } else if tree_builder == "fasttree" {
        run_fasttree(&tree_builder_path, &output, &combined_fasta.display().to_string(), &tree_options)?;
    } else { err::error(err::ERR_GENERAL, Some("Unrecognized tree builder".to_string())); }
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
            let output_msa_3di = gene_dir.join(format!("{}.fa.filtered.3di", gene_name)).display().to_string();
            filter_msa(&(msa_fasta.display().to_string() + "_aa.fa"), &output_msa, threshold)?;
            filter_msa(&(msa_fasta.display().to_string() + "_3di.fa"), &output_msa_3di, threshold)?;
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

pub fn run_raxml(raxml_path: &String, output_dir: &String, msa_fasta: &String, raxml_options: &String, threads: usize) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new(raxml_path);
    let mut cmd_options = raxml_options.split_whitespace().collect::<Vec<&str>>();
    let mut cmd_args = vec!["--msa", msa_fasta];

    // get the prefix for the output file
    let prefix = Path::new(output_dir).canonicalize()?.display().to_string() + "/raxml-ng";
    if !cmd_options.contains(&"--prefix") {
        cmd_args.push("--prefix");
        cmd_args.push(prefix.as_str());
    }

    let threads_copy = threads.to_string();
    if !cmd_options.contains(&"--threads"){
        cmd_args.push("--threads");
        cmd_args.push(threads_copy.as_str());
    }

    cmd_args.append(&mut cmd_options);
    let mut cmd = cmd.args(cmd_args);
    cmd::run(&mut cmd);
    Ok(())
}

pub fn run_fasttree(fasttree_path: &String, output_dir: &String, msa_fasta: &String, fasttree_options: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new(fasttree_path);
    let mut cmd_options = fasttree_options.split_whitespace().collect::<Vec<&str>>();
    let mut cmd_args = vec![];

    cmd_args.append(&mut cmd_options);
    cmd_args.push(msa_fasta.as_str());

    let mut cmd = cmd.args(cmd_args);
    let output_file = Path::new(output_dir).join("fasttree.nwk").display().to_string();
    cmd.stdout(fs::File::create(output_file)?);
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