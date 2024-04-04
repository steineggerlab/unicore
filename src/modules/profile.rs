use crate::util::arg_parser::{Args, Commands::Profile};
use crate::envs::variables as var;
use crate::util::arg_parser::Commands::Search;
use crate::util::generate_single_copy_gene;

pub fn run(args: &Args, bin: &crate::envs::variables::BinaryPaths) -> Result<(), Box<dyn std::error::Error>> {
    // Retrieve arguments
    let input = match &args.command {
        Some(Profile { input, .. }) => input.clone().to_string_lossy().into_owned(),
        _ => { crate::envs::error_handler::error(crate::envs::error_handler::ERR_ARGPARSE, Some("profile - input".to_string())); }
    };
    let mapping = match &args.command {
        Some(Profile { mapping, .. }) => mapping.clone().to_string_lossy().into_owned(),
        _ => { crate::envs::error_handler::error(crate::envs::error_handler::ERR_ARGPARSE, Some("profile - mapping".to_string())); }
    };
    let output = match &args.command {
        Some(Profile { output, .. }) => output.clone().to_string_lossy().into_owned(),
        _ => { crate::envs::error_handler::error(crate::envs::error_handler::ERR_ARGPARSE, Some("profile - output".to_string())); }
    };
    let threshold = match &args.command {
        Some(Profile { threshold, .. }) => *threshold,
        _ => { crate::envs::error_handler::error(crate::envs::error_handler::ERR_ARGPARSE, Some("profile - threshold".to_string())); }
    };
    let print_copiness = match &args.command {
        Some(Profile { print_copiness, .. }) => *print_copiness,
        _ => { crate::envs::error_handler::error(crate::envs::error_handler::ERR_ARGPARSE, Some("profile - print_copiness".to_string())); }
    };

    generate_single_copy_gene::profile(&input, &mapping, &output, &threshold, &print_copiness)?;
    
    Ok(())
}