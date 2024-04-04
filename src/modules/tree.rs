use std::fs;
use std::path::{Path, PathBuf};
use crate::util::arg_parser::{Args, Commands::Tree};
use crate::envs::error_handler as err;
use crate::util::command as cmd;
use crate::util::create_gene_specific_fasta as gsf;
use crate::util::combine_fasta as cf;

pub fn run(args: &Args, bin: &crate::envs::variables::BinaryPaths) -> Result<(), Box<dyn std::error::Error>> {
    // Retrieve arguments
    let proteome_db = match &args.command {
        Some(Tree { proteome_db, .. }) => proteome_db.clone().to_string_lossy().into_owned(),
        _ => { crate::envs::error_handler::error(crate::envs::error_handler::ERR_ARGPARSE, Some("tree - proteome_db".to_string())); }
    };
    let input = match &args.command {
        Some(Tree { input, .. }) => input.clone().to_string_lossy().into_owned(),
        _ => { crate::envs::error_handler::error(crate::envs::error_handler::ERR_ARGPARSE, Some("tree - input".to_string())); }
    };
    let output = match &args.command {
        Some(Tree { output, .. }) => output.clone().to_string_lossy().into_owned(),
        _ => { crate::envs::error_handler::error(crate::envs::error_handler::ERR_ARGPARSE, Some("tree - output".to_string())); }
    };
    let aligner = match &args.command {
        Some(Tree { aligner, .. }) => aligner.clone(),
        _ => { crate::envs::error_handler::error(crate::envs::error_handler::ERR_ARGPARSE, Some("tree - aligner".to_string())); }
    };
    let tree_method = match &args.command {
        Some(Tree { tree_method, .. }) => tree_method.clone(),
        _ => { crate::envs::error_handler::error(crate::envs::error_handler::ERR_ARGPARSE, Some("tree - tree_method".to_string())); }
    };
    let aligner_options = match &args.command {
        Some(Tree { aligner_options, .. }) => aligner_options.clone(),
        _ => { crate::envs::error_handler::error(crate::envs::error_handler::ERR_ARGPARSE, Some("tree - aligner_options".to_string())); }
    };
    let tree_options = match &args.command {
        Some(Tree { tree_options, .. }) => tree_options.clone(),
        _ => { crate::envs::error_handler::error(crate::envs::error_handler::ERR_ARGPARSE, Some("tree - tree_options".to_string())); }
    };

    // If there is no output directory, make one
    if !Path::new(&output).exists() {
        fs::create_dir_all(&output)?;
    }

    // Get the gene_list
    let gene_list = fs::read_dir(&input)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_file() && path.extension().map_or(false, |ext| ext == "txt"))
        .collect::<Vec<_>>();
    // Create gene specific fasta
    gsf::create_gene_specific_fasta(&proteome_db, &input, &gene_list)?;

    // Build foldseek db
    let foldseek_path = match &bin.get("foldseek") {
        Some(bin) => &bin.path,
        None => { err::error(err::ERR_BINARY_NOT_FOUND, Some("foldseek".to_string())); }
    };
    // Iterate through the gene_list and build foldseek db
    let input_path = Path::new(&input);
    // Only need to build foldseek db when the aligner is foldmason
    if aligner == "foldmason" {
        for gene in gene_list.iter() {
            if let Some(gene_name) = gene.file_stem().and_then(|name| name.to_str()) {
                let gene_dir = input_path.join(gene_name);
                // amino acid db
                let mut cmd = std::process::Command::new(foldseek_path);
                let aa_fasta = gene_dir.join("aa.fasta");
                let aa_db = gene_dir.join(format!("{}_db", gene_name).as_str());
                let cmd_args = vec!["base:createdb",
                                    aa_fasta.to_str().unwrap(),
                                    aa_db.to_str().unwrap(),
                                    "--shuffle", "0"];
                let mut cmd = cmd.args(cmd_args);
                cmd::run(&mut cmd);
                // 3Di db
                let mut cmd = std::process::Command::new(foldseek_path);
                let di_fasta = gene_dir.join("3di.fasta");
                let di_db = gene_dir.join(format!("{}_db_ss", gene_name).as_str());
                let cmd_args = vec!["base:createdb",
                                    di_fasta.to_str().unwrap(),
                                    di_db.to_str().unwrap(),
                                    "--shuffle", "0"];
                let mut cmd = cmd.args(cmd_args);
                cmd::run(&mut cmd);
            }
        }
    }

    // Generate alignment
    let aligner_path = match &bin.get(&aligner) {
        Some(bin) => &bin.path,
        None => { err::error(err::ERR_BINARY_NOT_FOUND, Some(aligner.clone())); }
    };
    // Iterate through the gene_list and generate alignment
    if aligner == "mafft" {
        run_mafft(&aligner_path, input_path, &gene_list, &aligner_options)?;
    } else if aligner == "foldmason" {
        run_foldmason(&aligner_path, input_path, &gene_list, &aligner_options)?;
    } else {
        err::error(err::ERR_MODULE_NOT_IMPLEMENTED, Some("Need implementation".to_string()))
    }

    // Make the vector of alignment files
    let msa_list = gene_list.iter()
        .map(|gene| {
            let gene_name = gene.file_stem().and_then(|name| name.to_str()).unwrap();
            input_path.join(gene_name).join(format!("{}.fa", gene_name)).display().to_string()
        })
        .collect::<Vec<_>>();

    // Combine alignment
    let combined_fasta = Path::new(&output).join("combined.fasta");
    cf::combine_fasta(&msa_list, &combined_fasta)?;

    // Build tree
    if tree_method == "iqtree" {
        let iqtree_path = match &bin.get("iqtree") {
            Some(bin) => &bin.path,
            None => { err::error(err::ERR_BINARY_NOT_FOUND, Some("iqtree".to_string())); }
        };
        run_iqtree(&iqtree_path, &output, &combined_fasta.display().to_string(), &tree_options)?;
    } else {
        // TODO: Implement other tree building methods
        err::error(err::ERR_MODULE_NOT_IMPLEMENTED, Some("Need implementation".to_string()))
    }

    Ok(())
}

fn run_mafft(mafft_path: &String, parent: &Path, gene_list: &Vec<PathBuf>, mafft_options: &String) -> Result<(), Box<dyn std::error::Error>> {
    for gene in gene_list.iter() {
        if let Some(gene_name) = gene.file_stem().and_then(|name| name.to_str()) {
            let gene_dir = parent.join(gene_name);
            let mut cmd = std::process::Command::new(mafft_path);
            // parse mafft_options into vector
            let mut cmd_args = mafft_options.split_whitespace().collect::<Vec<&str>>();
            // add input and output
            let aa_fasta = gene_dir.join("aa.fasta");
            let msa_fasta = gene_dir.join(format!("{}.fa", gene_name));
            cmd_args.push(aa_fasta.to_str().unwrap());
            cmd_args.push(">");
            cmd_args.push(msa_fasta.to_str().unwrap());
            let mut cmd = cmd.args(cmd_args);
            cmd::run(&mut cmd);
        }
    }
    Ok(())
}

fn run_foldmason(foldmason_path: &String, parent: &Path, gene_list: &Vec<PathBuf>, foldmason_options: &String) -> Result<(), Box<dyn std::error::Error>> {
    for gene in gene_list.iter() {
        if let Some(gene_name) = gene.file_stem().and_then(|name| name.to_str()) {
            let gene_dir = parent.join(gene_name);
            let mut cmd = std::process::Command::new(foldmason_path);
            let db = gene_dir.join(format!("{}_db", gene_name));
            let msa_fasta = gene_dir.join(gene_name);
            let mut cmd_args = vec!["structuremsa",
                            db.to_str().unwrap(),
                            msa_fasta.to_str().unwrap()];
            // parse foldmason_options into vector
            let mut cmd_options = foldmason_options.split_whitespace().collect::<Vec<&str>>();
            // Check "--comp-bias-corr" is in the option
            if !cmd_options.contains(&"--comp-bias-corr") {
                cmd_args.push("--comp-bias-corr");
                cmd_args.push("0");
            } else {
                // make sure "0" comes right after "--comp-bias-corr"
                // If not, replace the value
                let index = cmd_options.iter().position(|&x| x == "--comp-bias-corr").unwrap();
                if index + 1 == cmd_options.len() {
                    cmd_options.push("0");
                } else {
                    // replace the position to "0"
                    if cmd_options[index + 1] != "0" {
                        println!("--comp-bias-corr should be 0. Replacing the value to 0.");
                        cmd_options[index + 1] = "0";
                    }
                }
            }
            cmd_args.append(&mut cmd_options);
            let mut cmd = cmd.args(cmd_args);
            cmd::run(&mut cmd);
        }
    }
    Ok(())
}

fn run_iqtree(iqtree_path: &String, output_dir: &String, msa_fasta: &String, iqtree_options: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = std::process::Command::new(iqtree_path);
    let mut cmd_options = iqtree_options.split_whitespace().collect::<Vec<&str>>();
    // If there is "--prefix" in the option
    let mut cmd_args = vec!["-s", msa_fasta];
    let output_file = Path::new(output_dir).join("iqtree").display().to_string();
    if !cmd_options.contains(&"--prefix"){
        cmd_args.push("--prefix");
        cmd_args.push(output_file.as_str().clone());
    }
    // parse iqtree_options into vector
    cmd_args.append(&mut cmd_options);
    let mut cmd = cmd.args(cmd_args);
    cmd::run(&mut cmd);
    Ok(())
}