use std::path::Path;
use crate::util::arg_parser::Args;
use crate::envs::error_handler as err;
use crate::util::command as cmd;

// Run foldseek search and convertalis
pub fn run(args: &Args, bin: &crate::envs::variables::BinaryPaths) -> Result<(), Box<dyn std::error::Error>> {
    // Retrieve arguments
    let input = args.search_input.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("search - input".to_string())); });
    let target = args.search_target.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("search - target".to_string())); });
    let output = args.search_output.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("search - output".to_string())); });
    let tmp = args.search_tmp.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("search - tmp".to_string())); });
    let keep_aln_db = args.search_keep_aln_db.unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("search - keep_aln_db".to_string())); });
    let search_options = args.search_search_options.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("search - foldseek_args".to_string())); });

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
        "search", &target, &input, &output_aln_db, &tmp,
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
        "convertalis", &target, &input, &output_aln_db, &output_m8,
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