use crate::util::arg_parser::Args;
use crate::util::fasta_io as fasta;
use crate::envs::variables as var;
use crate::envs::error_handler as err;
use crate::util::command as cmd;

use std::io::{BufWriter, Write};
use std::collections::HashMap;
use std::path::{Path, MAIN_SEPARATOR as SEP};

pub fn run(args: &Args, bin: &var::BinaryPaths) -> Result<(), Box<dyn std::error::Error>> {
    // Retrieve mandatory arguments
    let input = args.createdb_input.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("createdb - input".to_string())); });
    let output = args.createdb_output.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("createdb - output".to_string())); });
    let model = args.createdb_model.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("createdb - model".to_string())); });
    let keep = args.createdb_keep.unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("createdb - keep".to_string())); });
    let overwrite = args.createdb_overwrite.unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("createdb - overwrite".to_string())); });
    let max_len = args.createdb_max_len.unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("createdb - max_len".to_string())); });

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
    if Path::new(&output).exists() && !overwrite {
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

    // Generate gene origin mapping file
    let mapping_file = format!("{}.map", output);
    let mut mapping_writer = BufWriter::new(std::fs::File::create(&mapping_file)?);
    let mut fasta_data = HashMap::new();
    for file in fasta_files {
        let species = Path::new(&file).file_stem().unwrap().to_str().unwrap();
        let each_fasta = fasta::read_fasta(&file);
        for (key, value) in each_fasta {
            if let Some(max_len) = max_len {
                if value.len() > max_len { continue; }
            }
            // replace all whitespace characters with underscore
            let key = key.replace(|c: char| c.is_whitespace(), "_");
            let key = format!("unicore_{}", key);
            fasta_data.insert(key.clone(), value);
            writeln!(mapping_writer, "{}\t{}", key, species)?;
        }
    }

    // Write out the combined amino acid fasta file into output directory
    // If 'parent' is absolute path, make curr_dir to the parent directory of the 'parent'
    let curr_dir = if Path::new(&parent).is_absolute() {
        if let Some(p) = Path::new(&parent).parent() {
            p.to_string_lossy().into_owned()
        } else {
            err::error(err::ERR_GENERAL, Some("Could not obtain parent directory of the parent".to_string()))
        }
    } else {
        var::current_dir()
    };
    let combined_aa = format!("{}{}{}{}combined_aa.fasta", curr_dir, SEP, parent, SEP);
    fasta::write_fasta(&combined_aa, &fasta_data)?;

    let foldseek_path = match &bin.get("foldseek") {
        Some(bin) => &bin.path,
        None => { err::error(err::ERR_BINARY_NOT_FOUND, Some("foldseek".to_string())); }
    };

    // Check if weights exist
    let model = if Path::new(&model).join("model.bin").exists() {
        model
    } else if Path::new(&model).join(format!("model{}model.bin", SEP)).exists() {
        format!("{}{}model", model, SEP)
    } else {
        // Download the model
        std::fs::create_dir_all(format!("{}{}tmp", model, SEP))?;
        let mut cmd = std::process::Command::new(foldseek_path);
        let mut cmd = cmd
            .arg("databases").arg("ProstT5").arg(&model).arg(format!("{}{}tmp", model, SEP));
        cmd::run(&mut cmd);
        format!("{}{}model", model, SEP)
    };

    // Run foldseek createdb
    let mut cmd = std::process::Command::new(foldseek_path);
    let mut cmd = cmd
        .arg("createdb").arg(&combined_aa).arg(&output)
        .arg("--prostt5-model").arg(&model);
    cmd::run(&mut cmd);

    // Delete intermediate files
    if !keep {
        std::fs::remove_file(combined_aa)?;
    }

    Ok(())
/* TODO Delete this block if foldseek createdb works
    let input_3di = format!("{}{}{}{}combined_3di.fasta", curr_dir, SEP, parent, SEP);
    let inter_prob = format!("{}{}{}{}output_probabilities.csv", curr_dir, SEP, parent, SEP);
    let output_3di = format!("{}{}{}_ss", curr_dir, SEP, output);

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
    if !keep {
        // std::fs::remove_file(mapping_file)?;
        std::fs::remove_file(combined_aa)?;
        std::fs::remove_file(input_3di)?;
        std::fs::remove_file(inter_prob)?;
    }
*/
}