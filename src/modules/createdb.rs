use std::path::MAIN_SEPARATOR as SEP;

use crate::util::arg_parser::{Args, Commands::Createdb};
use crate::util::fasta_io as fasta;
use crate::envs::variables as var;
use crate::envs::error_handler as err;
use crate::util::command as cmd;
use std::io::Write;
use std::collections::HashMap;

pub fn run(args: &Args, bin: &var::BinaryPaths) -> Result<(), Box<dyn std::error::Error>> {
    // Retrieve mandatory arguments
    let input = match &args.command {
        Some(Createdb { input, .. }) => input.clone().to_string_lossy().into_owned(),
        _ => { err::error(err::ERR_ARGPARSE, Some("createdb - input".to_string())); }
    };
    let output = match &args.command {
        Some(Createdb { output, .. }) => output.clone().to_string_lossy().into_owned(),
        _ => { err::error(err::ERR_ARGPARSE, Some("createdb - output".to_string())); }
    };
    let model = match &args.command {
        Some(Createdb { model, .. }) => model.clone().to_string_lossy().into_owned(),
        _ => { err::error(err::ERR_ARGPARSE, Some("createdb - model".to_string())); }
    };
    let delete_fasta = match &args.command {
        Some(Createdb { delete_fasta, .. }) => delete_fasta,
        _ => { err::error(err::ERR_ARGPARSE, Some("createdb - delete_fasta".to_string())); }
    };

    // Get all the fasta files in input directory
    let mut fasta_files = Vec::new();
    for entry in std::fs::read_dir(&input)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && (path.extension().unwrap_or_default() == "fasta" || path.extension().unwrap_or_default() == "fa")  {
            fasta_files.push(path.display().to_string());
        }
    }

    // Read in the fasta files
    // In the same time, write out the mapping file from gene to species (file name)
    // Parent directory of the output
    let parent = std::path::Path::new(&output).parent().unwrap().display().to_string();
    // If the parent directory of the output doesn't exist, make one
    if !std::path::Path::new(&parent).exists() {
        std::fs::create_dir_all(&parent)?;
    }
    let mapping_file = format!("{}/prot2spe.tsv", parent);
    let mut mapping_writer = std::io::BufWriter::new(std::fs::File::create(&mapping_file)?);
    let mut fasta_data = HashMap::new();
    for file in fasta_files {
        let species = std::path::Path::new(&file).file_stem().unwrap().to_str().unwrap();
        let each_fasta = fasta::read_fasta(&file);
        for (key, value) in each_fasta {
            fasta_data.insert(key.clone(), value);
            writeln!(mapping_writer, "{}\t{}", key, species)?;
        }
    }

    // Write out the combined amino acid fasta file into output directory
    let combined_aa = format!("{}/combined_aa.fasta", parent);
    fasta::write_fasta(&combined_aa, &fasta_data)?;

    let input_3di = format!("{}/combined_3di.fasta", parent);
    let output_3di = format!("{}_ss", output);

    // Run python script
    let mut cmd = std::process::Command::new("python3");
    let mut cmd = cmd
        .arg(format!("{}{}src{}py{}predict_3Di_encoderOnly.py", var::parent_dir(), SEP, SEP, SEP))
        .arg("-i").arg(&combined_aa)
        .arg("-o").arg(&input_3di)
        .arg("--model").arg(&model)
        .arg("--half").arg("0");
    cmd::run(&mut cmd);

    // Build foldseek db
    let foldseek_path = match &bin.get("foldseek") {
        Some(bin) => &bin.path,
        None => { err::error(err::ERR_BINARY_NOT_FOUND, Some("foldseek".to_string())); }
    };
    let mut cmd = std::process::Command::new(foldseek_path);
    let mut cmd = cmd
        .arg("base:createdb").arg(&combined_aa).arg(&output)
        .arg("--shuffle").arg("0");
    cmd::run(&mut cmd);

    // Build foldseek 3di db
    let mut cmd = std::process::Command::new(foldseek_path);
    let mut cmd = cmd
        .arg("base:createdb").arg(&input_3di).arg(&output_3di)
        .arg("--shuffle").arg("0");
    cmd::run(&mut cmd);

    // Delete fasta files
    if *delete_fasta {
        std::fs::remove_file(combined_aa)?;
        std::fs::remove_file(input_3di)?;
    }

    Ok(())
}