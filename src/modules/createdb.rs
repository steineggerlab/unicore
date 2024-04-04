use std::path::MAIN_SEPARATOR as SEP;

use crate::util::arg_parser::{Args, Commands::Createdb};
use crate::envs::variables as var;
use crate::envs::error_handler as err;
use crate::util::command as cmd;

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

    let input_3di = format!("{}.3di", input);
    let output_3di = format!("{}_ss", output);

    // Run python script
    let mut cmd = std::process::Command::new("python3");
    let mut cmd = cmd
        .arg(format!("{}{}lib{}predict_3Di_encoderOnly.py", var::parent_dir(), SEP, SEP))
        .arg("-i").arg(&input)
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
        .arg("base:createdb").arg(&input).arg(&output)
        .arg("--shuffle").arg("0");
    cmd::run(&mut cmd);

    // Build foldseek 3di db
    let mut cmd = std::process::Command::new(foldseek_path);
    let mut cmd = cmd
        .arg("base:createdb").arg(&input_3di).arg(&output_3di)
        .arg("--shuffle").arg("0");
    cmd::run(&mut cmd);

    Ok(())
}