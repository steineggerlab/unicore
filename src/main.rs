mod envs;
mod modules;
mod workflow;
mod util;
mod seq;

use envs::error_handler as err;
use envs::variables as var;
use util::arg_parser as parser;

// load path config
fn load_config(bin: &mut var::BinaryPaths, test: bool) {
    let cfg_path = format!("{}{}path.cfg", if test { var::test_parent_dir() } else { var::parent_dir() }, std::path::MAIN_SEPARATOR);
    bin.init(&std::path::Path::new(&cfg_path)).unwrap_or_else(|_| err::error(err::ERR_GENERAL, Some("Could not initialize binary paths".to_string())));
}

fn run(args: &parser::Args, bin: &var::BinaryPaths, test: bool) -> Result<(), Box<dyn std::error::Error>> {
    if test { modules::version::run(args, bin); return Ok(()); }
    if args.version { modules::version::run(args, bin); return Ok(()); }
    envs::variables::set_verbosity(args.verbosity);
    envs::variables::set_threads(args.threads);
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
        Some(parser::Commands::EasyCore { .. }) => {
            workflow::easy_core::run(args, bin).unwrap_or_else(|e| err::error(err::ERR_GENERAL, Some(e.to_string())));
        },
        Some(parser::Commands::EasySearch { .. }) => {
            workflow::easy_search::run(args, bin).unwrap_or_else(|e| err::error(err::ERR_GENERAL, Some(e.to_string())));
        }
        /* Some(_) => {
            err::error(err::ERR_MODULE_NOT_IMPLEMENTED, std::env::args().nth(1));
        } */
        _ => err::error(err::ERR_GENERAL, Some("No module name given. Run 'unicore help' for more information".to_string())),
    }
    Ok(())
}
fn main() {
    // Parse arguments
    let args = parser::Args::parse();

    // Retrieve bin from the config file
    let mut bin = var::BinaryPaths::new();
    load_config(&mut bin, false);

    run(&args, &bin, false).unwrap_or_else(|e| err::error(err::ERR_GENERAL, Some(e.to_string())));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config() {
        load_config(&mut var::BinaryPaths::new(), true);
    }

    #[test]
    fn test_run() {
        run(&parser::Args::default(), &var::BinaryPaths::new(), true).unwrap();
    }
}