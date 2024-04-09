use crate::util::arg_parser::{Args, Commands::Createdb};
use crate::util::fasta_io as fasta;
use crate::envs::variables as var;
use crate::envs::error_handler as err;
use crate::util::command as cmd;

use std::io::{BufWriter, Write};
use std::collections::HashMap;
use std::path::{Path, MAIN_SEPARATOR as SEP};

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
    let keep = match &args.command {
        Some(Createdb { keep, .. }) => keep,
        _ => { err::error(err::ERR_ARGPARSE, Some("createdb - keep".to_string())); }
    };
    let overwrite = match &args.command {
        Some(Createdb { overwrite, .. }) => overwrite,
        _ => { err::error(err::ERR_ARGPARSE, Some("createdb - overwrite".to_string())); }
    };

    // Get all the fasta files in input directory
    let mut fasta_files = Vec::new();
    if Path::new(&input).is_dir() {
        for entry in std::fs::read_dir(&input)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && (path.extension().unwrap_or_default() == "fasta" || path.extension().unwrap_or_default() == "fa") {
                fasta_files.push(path.clone().to_string_lossy().into_owned());
            }
        }
    } else {
        let path = Path::new(&input);
        if !path.is_file() { err::error(err::ERR_GENERAL, Some("Input is not a directory or a file".to_string())); }
        fasta_files.push(path.to_string_lossy().into_owned());
    }

    // Check if output file already exists
    if Path::new(&output).exists() && !*overwrite {
        err::error(err::ERR_OUTPUT_EXISTS, Some(output.clone()));
    }

    // Read in the fasta files
    // In the same time, write out the mapping file from gene to species (file name)
    // Try to obtain the parent directory of the output
    let parent = if let Some(p) = Path::new(&output).parent() {
        p.to_string_lossy().into_owned()
    } else {
        err::error(err::ERR_GENERAL, Some("Could not obtain parent directory of the output".to_string()))
    };
    // If the parent directory of the output doesn't exist, make one
    if !Path::new(&parent).exists() {
        std::fs::create_dir_all(&parent)?;
    }
    let mapping_file = format!("{}{}prot2spe.tsv", parent, SEP);
    let mut mapping_writer = BufWriter::new(std::fs::File::create(&mapping_file)?);
    let mut fasta_data = HashMap::new();
    for file in fasta_files {
        let species = Path::new(&file).file_stem().unwrap().to_str().unwrap();
        let each_fasta = fasta::read_fasta(&file);
        for (key, value) in each_fasta {
            fasta_data.insert(key.clone(), value);
            writeln!(mapping_writer, "{}\t{}", key, species)?;
        }
    }

    // Write out the combined amino acid fasta file into output directory
    let combined_aa = format!("{}{}combined_aa.fasta", parent, SEP);
    fasta::write_fasta(&combined_aa, &fasta_data)?;

    let input_3di = format!("{}{}combined_3di.fasta", parent, SEP);
    let inter_prob = format!("{}{}output_probabilities.csv", parent, SEP);
    let output_3di = format!("{}_ss", output);

    // Run python script
    let mut cmd = std::process::Command::new("python");
    let mut cmd = cmd
        .arg(format!("{}{}src{}py{}predict_3Di_encoderOnly.py", var::parent_dir(), SEP, SEP, SEP))
        .arg("-i").arg(&combined_aa)
        .arg("-o").arg(&input_3di)
        .arg("--model").arg(&model)
        .arg("--half").arg("0");
    cmd::run_at(&mut cmd, &Path::new(&var::parent_dir()));

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

    // Delete intermediate files
    if !*keep {
        std::fs::remove_file(mapping_file)?;
        std::fs::remove_file(combined_aa)?;
        std::fs::remove_file(input_3di)?;
        std::fs::remove_file(inter_prob)?;
    }

    Ok(())
}