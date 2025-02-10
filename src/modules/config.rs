use crate::envs::error_handler as err;
use crate::envs::variables as var;
use crate::util::command as cmd;
use crate::util::message as msg;
use crate::util::arg_parser::Args;
use color_print::cstr;

fn task_check(bin: &crate::envs::variables::BinaryPaths) -> Result<(), Box<dyn std::error::Error>> {
    msg::println_message(&format!("{}", cstr!(r#"<bold><underline>System:</underline></bold>"#)), 3);
    msg::println_message(&format!("Unicore version: {}", var::VERSION), 3);
    msg::println_message(&format!("OS: {}", std::env::consts::OS), 3);
    msg::println_message(&format!("Threads: {}", var::threads()), 3);
    println!();
    msg::println_message(&format!("{}", cstr!(r#"<bold><underline>Dependencies:</underline></bold>"#)), 3);
    msg::println_message(&format!("MMseqs2: {} .. {}",
                                  if let Some(&ref bin) = &bin.get("mmseqs") { if bin.set { bin.path.clone() } else { "Unset".to_string() } } else { "Undefined".to_string() },
                                  if let Some(&ref bin) = &bin.get("mmseqs") { if bin.set { if binary_run_test(&bin.path, "mmseqs") { cstr!(r#"<green>ok</green>"#) } else { cstr!(r#"<red>no</red>"#) } } else { cstr!(r#"<dim>n/a</dim>"#) } } else { cstr!(r#"<dim>n/a</dim>"#) },
    ), 3);
    msg::println_message(&format!("Foldseek: {} .. {}",
                                  if let Some(&ref bin) = &bin.get("foldseek") { if bin.set { bin.path.clone() } else { "Unset".to_string() } } else { "Undefined".to_string() },
                                  if let Some(&ref bin) = &bin.get("foldseek") { if bin.set { if binary_run_test(&bin.path, "foldseek") { cstr!(r#"<green>ok</green>"#) } else { cstr!(r#"<red>no</red>"#) } } else { cstr!(r#"<dim>n/a</dim>"#) } } else { cstr!(r#"<dim>n/a</dim>"#) },
    ), 3);
    msg::println_message(&format!("FoldMason: {} .. {}",
                                  if let Some(&ref bin) = &bin.get("foldmason") { if bin.set { bin.path.clone() } else { "Unset".to_string() } } else { "Undefined".to_string() },
                                  if let Some(&ref bin) = &bin.get("foldmason") { if bin.set { if binary_run_test(&bin.path, "foldmason") { cstr!(r#"<green>ok</green>"#) } else { cstr!(r#"<red>no</red>"#) } } else { cstr!(r#"<dim>n/a</dim>"#) } } else { cstr!(r#"<dim>n/a</dim>"#) },
    ), 3);
    msg::println_message(&format!("MAFFT: {} .. {}",
                                  if let Some(&ref bin) = &bin.get("mafft") { if bin.set { bin.path.clone() } else { "Unset".to_string() } } else { "Undefined".to_string() },
                                  if let Some(&ref bin) = &bin.get("mafft") { if bin.set { if binary_run_test(&bin.path, "mafft") { cstr!(r#"<green>ok</green>"#) } else { cstr!(r#"<red>no</red>"#) } } else { cstr!(r#"<dim>n/a</dim>"#) } } else { cstr!(r#"<dim>n/a</dim>"#) },
    ), 3);
    msg::println_message(&format!("IQ-TREE: {} .. {}",
                                  if let Some(&ref bin) = &bin.get("iqtree") { if bin.set { bin.path.clone() } else { "Unset".to_string() } } else { "Undefined".to_string() },
                                  if let Some(&ref bin) = &bin.get("iqtree") { if bin.set { if binary_run_test(&bin.path, "iqtree") { cstr!(r#"<green>ok</green>"#) } else { cstr!(r#"<red>no</red>"#) } } else { cstr!(r#"<dim>n/a</dim>"#) } } else { cstr!(r#"<dim>n/a</dim>"#) },
    ), 3);
    msg::println_message(&format!("FastTree: {} .. {}",
                                  if let Some(&ref bin) = &bin.get("fasttree") { if bin.set { bin.path.clone() } else { "Unset".to_string() } } else { "Undefined".to_string() },
                                  if let Some(&ref bin) = &bin.get("fasttree") { if bin.set { if binary_run_test(&bin.path, "fasttree") { cstr!(r#"<green>ok</green>"#) } else { cstr!(r#"<red>no</red>"#) } } else { cstr!(r#"<dim>n/a</dim>"#) } } else { cstr!(r#"<dim>n/a</dim>"#) },
    ), 3);
    msg::println_message(&format!("RAxML: {} .. {}",
                                  if let Some(&ref bin) = &bin.get("raxml") { if bin.set { bin.path.clone() } else { "Unset".to_string() } } else { "Undefined".to_string() },
                                  if let Some(&ref bin) = &bin.get("raxml") { if bin.set { if binary_run_test(&bin.path, "raxml") { cstr!(r#"<green>ok</green>"#) } else { cstr!(r#"<red>no</red>"#) } } else { cstr!(r#"<dim>n/a</dim>"#) } } else { cstr!(r#"<dim>n/a</dim>"#) },
    ), 3);
    Ok(())
}

fn binary_run_test(path: &str, sw: &str) -> bool {
    if !var::VALID_BINARY.contains(&sw) { return false; }
    let mut test_command = std::process::Command::new(path);
    let test_command = match sw {
        "mmseqs" | "foldseek" | "foldmason" => test_command.arg("version"),
        "mafft" | "iqtree" => test_command.arg("--version"),
        "fasttree" => &mut test_command,
        "raxml" => test_command.arg("-v"),
        _ => return false,
    };
    cmd::run_code(test_command) == 0
}

fn binary_status(bin: &crate::envs::variables::BinaryPaths, sw: &str) -> u8 {
    if let Some(&ref bin) = &bin.get(sw) {
        if !std::path::Path::new(&bin.path).exists() { return 1; } // Binary path does not exist
    } else { return 2; } // Binary path is not set

    0
}
const TASK_SET_MMSEQS: u8 = 0x02;
const TASK_SET_FOLDSEEK: u8 = 0x03;
const TASK_SET_FOLDMASON: u8 = 0x04;
const TASK_SET_MAFFT: u8 = 0x05;
const TASK_SET_MAFFT_LINSI: u8 = 0x06;
const TASK_SET_IQTREE: u8 = 0x07;
const TASK_SET_FASTTREE: u8 = 0x08;
const TASK_SET_RAXML: u8 = 0x09;

pub fn run(args: &Args, bin: &crate::envs::variables::BinaryPaths) -> Result<(), Box<dyn std::error::Error>> {
    if args.config_check.is_some() && args.config_check.unwrap() { task_check(bin)?; }
    else if args.config_set_mmseqs.is_some() { TASK_SET_MMSEQS; }
    else if args.config_set_foldseek.is_some() { TASK_SET_FOLDSEEK; }
    else if args.config_set_foldmason.is_some() { TASK_SET_FOLDMASON; }
    else if args.config_set_mafft.is_some() { TASK_SET_MAFFT; }
    else if args.config_set_mafft_linsi.is_some() { TASK_SET_MAFFT_LINSI; }
    else if args.config_set_iqtree.is_some() { TASK_SET_IQTREE; }
    else if args.config_set_fasttree.is_some() { TASK_SET_FASTTREE; }
    else if args.config_set_raxml.is_some() { TASK_SET_RAXML; }
    else { err::error(err::ERR_ARGPARSE, Some("No task specified".to_string())) };
    Ok(())
}