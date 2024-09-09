mod envs;
mod modules;
mod util;
mod seq;

use envs::error_handler as err;
use envs::variables as var;
use util::arg_parser as parser;

// load path config
fn load_config(bin: &mut var::BinaryPaths) {
    let cfg_path = format!("{}{}path.cfg", var::parent_dir(), std::path::MAIN_SEPARATOR);
    bin.init(&std::path::Path::new(&cfg_path)).unwrap_or_else(|_| err::error(err::ERR_GENERAL, Some("Could not initialize binary paths".to_string())));
}

fn run(args: &parser::Args, bin: &var::BinaryPaths) -> Result<(), Box<dyn std::error::Error>> {
    if args.version { modules::version::run(args, bin); return Ok(()); }
    envs::variables::set_verbosity(args.verbosity);
    match &args.command {
        Some(parser::Commands::Createdb { .. }) => {
            modules::createdb::run(args, bin).unwrap_or_else(|e| err::error(err::ERR_GENERAL, Some(e.to_string())));
        },
        Some(parser::Commands::Cluster { .. }) => {
            modules::cluster::run(args, bin).unwrap_or_else(|e| err::error(err::ERR_GENERAL, Some(e.to_string())));
        },
        Some(parser::Commands::Search { .. }) => {
            modules::search::run(args, bin).unwrap_or_else(|e| err::error(err::ERR_GENERAL, Some(e.to_string())));
        },
        Some(parser::Commands::Profile { .. }) => {
            modules::profile::run(args, bin).unwrap_or_else(|e| err::error(err::ERR_GENERAL, Some(e.to_string())));
        },
        Some(parser::Commands::Tree { .. }) => {
            modules::tree::run(args, bin).unwrap_or_else(|e| err::error(err::ERR_GENERAL, Some(e.to_string())));
        },
        /* Some(_) => {
            err::error(err::ERR_MODULE_NOT_IMPLEMENTED, std::env::args().nth(1));
        } */
        _ => err::error(err::ERR_GENERAL, Some("Unreachable".to_string())),
    }
    Ok(())
}
fn main() {
    // Parse arguments
    let args = parser::Args::parse();

    // Retrieve bin from the config file
    let mut bin = var::BinaryPaths::new();
    load_config(&mut bin);

    run(&args, &bin).unwrap_or_else(|e| err::error(err::ERR_GENERAL, Some(e.to_string())));
}
