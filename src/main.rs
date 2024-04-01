mod envs;
mod modules;
mod util;
use envs::error_handler as err;
use envs::variables as var;
use util::arg_parser as parser;
use clap::Parser;

fn usage() {
    println!("Unicore v{} {}", var::VERSION, var::STABLE_TEXT);
    println!("Usage: unicore <module> [options]");
    println!();
    println!("Available modules:");
    println!("  version    : Print version and information");
    println!("  help       : Print this help message");
    println!("  more to come...");
    println!();
}

const VALID_MODULES: [&str; 4] = [
    "help", "usage", "info", "version",
];
fn init() -> var::BinaryPaths {
    // load path config
    let cfg_path = format!("{}{}path.cfg", var::parent_dir(), std::path::MAIN_SEPARATOR);
    let mut bin = var::BinaryPaths::new();
    bin.init(&std::path::Path::new(&cfg_path)).unwrap_or_else(|_| err::error(err::ERR_GENERAL, Some("Could not initialize binary paths".to_string())));
    bin
}

fn run(args: &parser::Args, bin: var::BinaryPaths) -> Result<(), Box<dyn std::error::Error>> {
    match args.command {
        Some(parser::Commands::profile { .. }) => {
            // TODO: error handling
            modules::profile::run(args).unwrap_or_else(|e| err::error(err::ERR_GENERAL, Some(e.to_string())));
        },
        _ => unreachable!(),
    }
    Ok(())
}
fn main() {
    let args = parser::Args::parse();
    // Retrieve bin from the config file
    let bin = init();
    run(&args, bin).unwrap_or_else(|e| err::error(err::ERR_GENERAL, Some(e.to_string())));
}
