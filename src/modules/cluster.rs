use std::path::Path;
use crate::util::arg_parser::Args;
use crate::envs::error_handler as err;
use crate::util::command as cmd;

// Run foldseek cluster and createtsv
pub fn run(args: &Args, bin: &crate::envs::variables::BinaryPaths) -> Result<(), Box<dyn std::error::Error>> {
    // Retrieve arguments
    let input: String = args.cluster_input.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("cluster - input".to_string())); });
    let output: String = args.cluster_output.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("cluster - output".to_string())); });
    let tmp: String = args.cluster_tmp.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("cluster - tmp".to_string())); });
    let keep_cluster_db: bool = args.cluster_keep_cluster_db.unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("cluster - keep_cluster_db".to_string())); });
    let cluster_options: String = args.cluster_cluster_options.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("cluster - cluster_args".to_string())); });

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

    // cluster_arg into vector, parsing by space
    let cluster_args: Vec<&str> = cluster_options.split_whitespace().collect();

    // Get foldseek
    let foldseek_path = match &bin.get("foldseek") {
        Some(bin) => &bin.path,
        None => { err::error(err::ERR_BINARY_NOT_FOUND, Some("foldseek".to_string())); }
    };

    let output_cluster_db = format!("{}_cluster", output);
    let output_tsv = format!("{}.tsv", output);
    let mut foldseek_flag = vec![
        "cluster", &input, &output_cluster_db, &tmp,
    ];
    // Include cluster_args into foldseek_flag
    foldseek_flag.extend(cluster_args.iter());

    // Run foldseek cluster
    let mut cmd = std::process::Command::new(foldseek_path);
    let mut cmd = cmd.args(&foldseek_flag);
    cmd::run(&mut cmd);

    // Run foldseek createtsv
    let mut cmd = std::process::Command::new(foldseek_path);
    let foldseek_flag = vec![
        "createtsv", &input, &input, &output_cluster_db, &output_tsv,
    ];
    let mut cmd = cmd.args(&foldseek_flag);
    cmd::run(&mut cmd);

    // Delete intermediate database
    if !keep_cluster_db {
        let mut cmd = std::process::Command::new(foldseek_path);
        let foldseek_flag = vec![
            "rmdb", 
            &output_cluster_db,
        ];
        let mut cmd = cmd.args(&foldseek_flag);
        cmd::run(&mut cmd);
    }

    // TODO: Implement detection and removal of foldseek cluster temporary results

    Ok(())
}