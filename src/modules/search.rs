use crate::util::arg_parser::{Args, Commands::Search};
use crate::envs::error_handler as err;
use crate::util::command as cmd;

// Run foldseek search and convertalis
pub fn run(args: &Args, bin: &crate::envs::variables::BinaryPaths) -> Result<(), Box<dyn std::error::Error>> {
    // Retrieve arguments
    let input_db = match &args.command {
        Some(Search { input_db, .. }) => input_db.clone().to_string_lossy().into_owned(),
        _ => { err::error(err::ERR_ARGPARSE, Some("search - input_db".to_string())); }
    };
    let target_db = match &args.command {
        Some(Search { target_db, .. }) => target_db.clone().to_string_lossy().into_owned(),
        _ => { err::error(err::ERR_ARGPARSE, Some("search - target_db".to_string())); }
    };
    let output_db = match &args.command {
        Some(Search { output_db, .. }) => output_db.clone().to_string_lossy().into_owned(),
        _ => { err::error(err::ERR_ARGPARSE, Some("search - output_db".to_string())); }
    };
    let output_db_m8 = format!("{}.m8", output_db);
    let tmp = match &args.command {
        Some(Search { tmp, .. }) => tmp.clone().to_string_lossy().into_owned(),
        _ => { err::error(err::ERR_ARGPARSE, Some("search - tmp".to_string())); }
    };
    let delete_tmp = match &args.command {
        Some(Search { delete_tmp, .. }) => delete_tmp.clone(),
        _ => { err::error(err::ERR_ARGPARSE, Some("search - delete_tmp".to_string())); }
    };
    let search_options = match &args.command {
        Some(Search { search_options, .. }) => search_options.clone(),
        _ => { err::error(err::ERR_ARGPARSE, Some("search - foldseek_args".to_string())); }
    };

    // foldseek_arg into vector, parsing by space
    let foldseek_args: Vec<&str> = search_options.split_whitespace().collect();

    // Get foldseek
    let foldseek_path = match &bin.get("foldseek") {
        Some(bin) => &bin.path,
        None => { err::error(err::ERR_BINARY_NOT_FOUND, Some("foldseek".to_string())); }
    };

    let mut foldseek_flag = vec![
        "search", &input_db, &target_db, &output_db, &tmp,
    ];
    // Include foldseek_args into foldseek_flag
    foldseek_flag.extend(foldseek_args.iter());

    // Run foldseek search
    let mut cmd = std::process::Command::new(foldseek_path);
    let mut cmd = cmd.args(&foldseek_flag);
    cmd::run(&mut cmd);

    // If delete_tmp is true, remove tmp directory
    if delete_tmp {
        std::fs::remove_dir_all(&tmp)?;
    }

    // Run foldseek convertalis
    let mut cmd = std::process::Command::new(foldseek_path);
    let mut foldseek_flag = vec![
        "convertalis",
        &input_db, &target_db, &output_db, &output_db_m8,
    ];
    let mut cmd = cmd.args(&foldseek_flag);
    cmd::run(&mut cmd);

    Ok(())
}