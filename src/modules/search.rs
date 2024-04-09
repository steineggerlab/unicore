use crate::util::arg_parser::{Args, Commands::Search};
use crate::envs::error_handler as err;
use crate::util::command as cmd;

// Run foldseek search and convertalis
pub fn run(args: &Args, bin: &crate::envs::variables::BinaryPaths) -> Result<(), Box<dyn std::error::Error>> {
    // Retrieve arguments
    let input = match &args.command {
        Some(Search { input, .. }) => input.clone().to_string_lossy().into_owned(),
        _ => { err::error(err::ERR_ARGPARSE, Some("search - input".to_string())); }
    };
    let target = match &args.command {
        Some(Search { target, .. }) => target.clone().to_string_lossy().into_owned(),
        _ => { err::error(err::ERR_ARGPARSE, Some("search - target".to_string())); }
    };
    let output = match &args.command {
        Some(Search { output, .. }) => output.clone().to_string_lossy().into_owned(),
        _ => { err::error(err::ERR_ARGPARSE, Some("search - output".to_string())); }
    };
    let tmp = match &args.command {
        Some(Search { tmp, .. }) => tmp.clone().to_string_lossy().into_owned(),
        _ => { err::error(err::ERR_ARGPARSE, Some("search - tmp".to_string())); }
    };
    let keep_aln_db = match &args.command {
        Some(Search { keep_aln_db, .. }) => keep_aln_db.clone(),
        _ => { err::error(err::ERR_ARGPARSE, Some("search - keep_aln_db".to_string())); }
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

    let output_aln_db = format!("{}_aln", output);
    let output_m8 = format!("{}.m8", output);
    let mut foldseek_flag = vec![
        "search", &input, &target, &output_aln_db, &tmp,
    ];
    // Include foldseek_args into foldseek_flag
    foldseek_flag.extend(foldseek_args.iter());

    // Run foldseek search
    let mut cmd = std::process::Command::new(foldseek_path);
    let mut cmd = cmd.args(&foldseek_flag);
    cmd::run(&mut cmd);

    // Run foldseek convertalis
    let mut cmd = std::process::Command::new(foldseek_path);
    let foldseek_flag = vec![
        "convertalis",
        &input, &target, &output_aln_db, &output_m8,
    ];
    let mut cmd = cmd.args(&foldseek_flag);
    cmd::run(&mut cmd);

    // Delete intermediate database
    if !keep_aln_db {
        let mut cmd = std::process::Command::new(foldseek_path);
        let foldseek_flag = vec![
            "rmdb",
            &output_aln_db,
        ];
        let mut cmd = cmd.args(&foldseek_flag);
        cmd::run(&mut cmd);
    }

    // TODO: implement detection and removal of foldseek search temporary results

    Ok(())
}