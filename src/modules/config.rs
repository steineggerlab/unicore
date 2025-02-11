use std::os::unix::fs::MetadataExt;
use std::io::Write;
use crate::envs::error_handler as err;
use crate::envs::variables as var;
use crate::envs::variables::BinaryPaths;
use crate::util::command as cmd;
use crate::util::message as msg;
use crate::util::arg_parser::Args;
use color_print::cstr;

fn task_check(bin: &BinaryPaths) -> Result<(), Box<dyn std::error::Error>> {
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
        "mafft" | "mafft-linsi" | "iqtree" => test_command.arg("--version"),
        "fasttree" => &mut test_command,
        "raxml" => test_command.arg("-v"),
        _ => return false,
    };
    cmd::run_code(test_command) == 0
}

fn set_binary(bin: &BinaryPaths, path: &str, sw: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !std::fs::File::open(path).is_ok() { err::error(err::ERR_BINARY_NOT_FOUND, Some(format!("{}", path))); }
    if std::fs::metadata(path)?.is_dir() { err::error(err::ERR_FILE_INVALID, Some(format!("{}", path))); }
    if std::fs::metadata(path)?.mode() & 0o111 == 0 { err::error(err::ERR_BINARY_NOT_EXECUTABLE, Some(format!("{}", path))); }
    if !binary_run_test(path, sw) { err::error(err::ERR_BINARY_INVALID, Some(format!("{}", path))); }

    let path = std::fs::canonicalize(path)?.to_str().unwrap().to_string();
    msg::println_message(&format!("Setting dependency {} to {}...", sw, path), 3);
    let mut cfg = std::fs::File::create(var::locate_path_cfg())?;
    for &prog in var::VALID_BINARY.iter() {
        if prog == sw { cfg.write_all(format!("{}={}\n", prog, path).as_bytes())?; }
        else if bin.get(prog).is_none() || !bin.get(prog).unwrap().set { cfg.write_all(format!("#{}=\n", prog).as_bytes())?; }
        else { cfg.write_all(format!("{}={}\n", prog, bin.get(prog).unwrap().path).as_bytes())?; }
    }
    msg::println_message(&"Done. Please run \"unicore config -c\" to check".to_string(), 3);

    Ok(())
}

pub fn run(args: &Args, bin: &BinaryPaths) -> Result<(), Box<dyn std::error::Error>> {
    if args.config_check.is_some() && args.config_check.unwrap() { task_check(bin)?; }
    else if args.config_set_mmseqs.is_some() { set_binary(bin, args.config_set_mmseqs.clone().unwrap().as_str(), "mmseqs")?; }
    else if args.config_set_foldseek.is_some() { set_binary(bin, args.config_set_foldseek.clone().unwrap().as_str(), "foldseek")?; }
    else if args.config_set_foldmason.is_some() { set_binary(bin, args.config_set_foldmason.clone().unwrap().as_str(), "foldmason")?; }
    else if args.config_set_mafft.is_some() { set_binary(bin, args.config_set_mafft.clone().unwrap().as_str(), "mafft")?; }
    else if args.config_set_mafft_linsi.is_some() { set_binary(bin, args.config_set_mafft_linsi.clone().unwrap().as_str(), "mafft-linsi")?; }
    else if args.config_set_iqtree.is_some() { set_binary(bin, args.config_set_iqtree.clone().unwrap().as_str(), "iqtree")?; }
    else if args.config_set_fasttree.is_some() { set_binary(bin, args.config_set_fasttree.clone().unwrap().as_str(), "fasttree")?; }
    else if args.config_set_raxml.is_some() { set_binary(bin, args.config_set_raxml.clone().unwrap().as_str(), "raxml")?; }
    else { err::error(err::ERR_ARGPARSE, Some("No task specified".to_string())) };
    Ok(())
}