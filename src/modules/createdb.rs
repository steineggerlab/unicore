use crate::seq::fasta_io as fasta;
use crate::envs::variables as var;
use crate::envs::error_handler as err;
use crate::util::arg_parser::Args;
use crate::util::command as cmd;
use crate::util::message as msg;
use crate::util::checkpoint as chkpnt;

use std::io::{BufWriter, Write};
use std::collections::HashMap;
use std::path::{Path, MAIN_SEPARATOR as SEP};
use std::process::Command as Cmd;

// Function of checking if the character is part of the specified set
fn need_replacement(c: char) -> bool {
    // Check if the character is in whitespace or ';', ':', ',', '=', '/', '(' or ')'
    c.is_whitespace() || c == ';' || c == ':' || c == ',' || c == '=' || c == '/' || c == '(' || c == ')'
}

pub fn run(args: &Args, bin: &var::BinaryPaths) -> Result<(), Box<dyn std::error::Error>> {
    // Retrieve mandatory arguments
    let input = args.createdb_input.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("createdb - input".to_string())); });
    let output = args.createdb_output.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("createdb - output".to_string())); });
    let model = args.createdb_model.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("createdb - model".to_string())); });
    let keep = args.createdb_keep.unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("createdb - keep".to_string())); });
    let overwrite = args.createdb_overwrite.unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("createdb - overwrite".to_string())); });
    let max_len = args.createdb_max_len.unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("createdb - max_len".to_string())); });
    let gpu = args.createdb_gpu.unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("createdb - gpu".to_string())); });
    let afdb_lookup = args.createdb_afdb_lookup.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("createdb - afdb_lookup".to_string())); });
    let custom_lookup = args.createdb_custom_lookup.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("createdb - custom_lookup".to_string())); });
    let threads = crate::envs::variables::threads();
    let foldseek_verbosity = (match var::verbosity() { 4 => 3, 3 => 2, _ => var::verbosity() }).to_string();

    // Check if lookup options conflict
    if afdb_lookup.is_some() && custom_lookup.is_some() {
        err::error(err::ERR_ARGPARSE, Some("Both afdb_lookup and custom_lookup are specified. Please specify only one.".to_string()));
    }

    // Try to obtain the parent directory of the output
    let parent = if let Some(p) = Path::new(&output).parent() {
        p.to_string_lossy().into_owned()
    } else {
        err::error(err::ERR_GENERAL, Some("Could not obtain parent directory of the output".to_string()))
    };
    // If the parent directory is empty, set it to '.'
    let parent = if parent.is_empty() { ".".to_string() } else { parent };
    // Print the parent directory
    msg::println_message(&format!("Parent directory: {}", parent), 4);
    // If the parent directory of the output doesn't exist, make one
    if !Path::new(&parent).exists() {
        std::fs::create_dir_all(&parent)?;
    }

    // Check if the checkpoint file exists
    let checkpoint_file = format!("{}/createdb.chk", parent);
    if Path::new(&checkpoint_file).exists() {
        // Read the checkpoint file
        let content = chkpnt::read_checkpoint(&checkpoint_file)?;
        if content == "1" && !overwrite {
            err::error(err::ERR_GENERAL, Some("Database already exists, skipping createdb module".to_string()));
        }
    } else {
        // Write the checkpoint file
        chkpnt::write_checkpoint(&checkpoint_file, "0")?;
    }
    
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

    // Read in the fasta files
    // In the same time, write out the mapping file from gene to species (file name)
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
            if value.len() < 2 {
                msg::println_message(&format!("Skipping {} as it is too short", key), 3);
                continue;
            }
            // replace all whitespace characters with underscore
            let key = key.replace(|c: char| need_replacement(c), "_");

            // hash sequence to avoid name collision
            let mut hash = format!("{:x}", md5::compute(value.clone()));
            hash.truncate(10);
            let hashed_name = format!("unicore_{}", hash);
            fasta_data.insert(hashed_name.clone(), value);
            writeln!(mapping_writer, "{}\t{}\t{}", hashed_name, species, key)?;
        }
    }
    mapping_writer.flush()?;

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
    let converted_aa = format!("{}{}{}{}converted_aa.fasta", curr_dir, SEP, parent, SEP);
    let converted_ss = format!("{}{}{}{}converted_ss.fasta", curr_dir, SEP, parent, SEP);
    if afdb_lookup.is_some() {
        // this will split data into converted and combined fasta files
        crate::seq::afdb_lookup::run(&fasta_data, &afdb_lookup.clone().unwrap(), &converted_aa, &converted_ss, &combined_aa, false)?;
    } else if custom_lookup.is_some() {
        crate::seq::afdb_lookup::run(&fasta_data, &afdb_lookup.clone().unwrap(), &converted_aa, &converted_ss, &combined_aa, true)?;
    } else {
        fasta::write_fasta(&combined_aa, &fasta_data, false)?;
    }

    // Use foldseek to create the database
    let foldseek_path = match &bin.get("foldseek") {
        Some(bin) => &bin.path,
        _none => { err::error(err::ERR_BINARY_NOT_FOUND, Some("foldseek".to_string())); }
    };

    // Check if old weights exist
    if Path::new(&model).join("cnn.safetensors").exists() || Path::new(&model).join(format!("model{}cnn.safetensors", SEP)).exists() {
        err::error(err::ERR_GENERAL, Some("Old weight files detected from the given path. Please provide different path for the model weights".to_string()));
    }
    // Check if weights exist
    if !Path::new(&model).join("prostt5-f16.gguf").exists() {
        // Download the model
        std::fs::create_dir_all(format!("{}{}tmp", model, SEP))?;
        let mut cmd = std::process::Command::new(foldseek_path);
        let mut cmd = cmd
            .arg("databases").arg("ProstT5").arg(&model).arg(format!("{}{}tmp", model, SEP)).arg("--threads").arg(threads.to_string());
        cmd::run(&mut cmd);
    }

    // Run foldseek createdb
    let mut cmd = std::process::Command::new(foldseek_path);
    let cmd = cmd
        .arg("createdb").arg(&combined_aa).arg(&output)
        .arg("--prostt5-model").arg(&model)
        .arg("--threads").arg(threads.to_string());
    let mut cmd = if gpu {
        cmd.arg("--gpu").arg("1")
    } else { cmd };
    cmd::run(&mut cmd);

    if afdb_lookup.is_some() || custom_lookup.is_some() {
        let foldseek_path = match &bin.get("foldseek") {
            Some(bin) => &bin.path,
            _none => { err::error(err::ERR_BINARY_NOT_FOUND, Some("foldseek".to_string())); }
        };
        let converted_aa_db = format!("{}{}{}{}converted", curr_dir, SEP, parent, SEP);
        let converted_h_db = format!("{}{}{}{}converted_h", curr_dir, SEP, parent, SEP);
        let converted_ss_db = format!("{}{}{}{}converted_ss", curr_dir, SEP, parent, SEP);
        let converted_ss_h_db = format!("{}{}{}{}converted_ss_h", curr_dir, SEP, parent, SEP);
        cmd::run(Cmd::new(foldseek_path).arg("base:createdb").arg(&converted_aa).arg(&converted_aa_db).arg("--shuffle").arg("0").arg("-v").arg(foldseek_verbosity.as_str()));
        cmd::run(Cmd::new(foldseek_path).arg("base:createdb").arg(&converted_ss).arg(&converted_ss_db).arg("--shuffle").arg("0").arg("-v").arg(foldseek_verbosity.as_str()));

        // Concatenate the two databases
        let output_ss = format!("{}_ss", output);
        let output_h = format!("{}_h", output);
        let concat_aa_db = format!("{}{}{}{}concat_aa", curr_dir, SEP, parent, SEP);
        let concat_ss_db = format!("{}{}{}{}concat_ss", curr_dir, SEP, parent, SEP);
        let concat_h_db = format!("{}{}{}{}concat_h", curr_dir, SEP, parent, SEP);
        cmd::run(Cmd::new(foldseek_path).arg("base:concatdbs").arg(&output).arg(&converted_aa_db).arg(&concat_aa_db).arg("-v").arg(foldseek_verbosity.as_str()).arg("--threads").arg("1"));
        cmd::run(Cmd::new(foldseek_path).arg("base:concatdbs").arg(&output_ss).arg(&converted_ss_db).arg(&concat_ss_db).arg("-v").arg(foldseek_verbosity.as_str()).arg("--threads").arg("1"));
        cmd::run(Cmd::new(foldseek_path).arg("base:concatdbs").arg(&output_h).arg(&converted_h_db).arg(&concat_h_db).arg("-v").arg(foldseek_verbosity.as_str()).arg("--threads").arg("1"));

        // Rename databases
        cmd::run(Cmd::new(foldseek_path).arg("base:mvdb").arg(&concat_aa_db).arg(&output).arg("-v").arg(foldseek_verbosity.as_str()));
        cmd::run(Cmd::new(foldseek_path).arg("base:mvdb").arg(&concat_ss_db).arg(&output_ss).arg("-v").arg(foldseek_verbosity.as_str()));
        cmd::run(Cmd::new(foldseek_path).arg("base:mvdb").arg(&concat_h_db).arg(&output_h).arg("-v").arg(foldseek_verbosity.as_str()));

        // Delete intermediate files
        if !keep {
            std::fs::remove_file(converted_aa)?;
            std::fs::remove_file(converted_ss)?;
            std::fs::remove_file(format!("{}.source", concat_aa_db)).or_else(|_| Ok::<(), Box<dyn std::error::Error>>(()))?;
            cmd::run(Cmd::new(foldseek_path).arg("base:rmdb").arg(&converted_aa_db).arg("-v").arg(foldseek_verbosity.as_str()));
            cmd::run(Cmd::new(foldseek_path).arg("base:rmdb").arg(&converted_h_db).arg("-v").arg(foldseek_verbosity.as_str()));
            cmd::run(Cmd::new(foldseek_path).arg("base:rmdb").arg(&converted_ss_db).arg("-v").arg(foldseek_verbosity.as_str()));
            cmd::run(Cmd::new(foldseek_path).arg("base:rmdb").arg(&converted_ss_h_db).arg("-v").arg(foldseek_verbosity.as_str()));
        }
    }

    // Delete intermediate files
    if !keep {
        std::fs::remove_file(combined_aa)?;
    }

    // Write the checkpoint file
    chkpnt::write_checkpoint(&checkpoint_file, "1")?;
    

    Ok(())
}
