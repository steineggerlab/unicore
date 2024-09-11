use crate::util::arg_parser::Args;
use crate::util::message::println_message as mprintln;
use crate::envs::variables as var;
use crate::envs::error_handler as err;

use crate::modules::createdb::run as createdb;
use crate::modules::cluster::run as cluster;
use crate::modules::profile::run as profile;
use crate::modules::tree::run as tree;

pub fn run(args: &Args, bin: &var::BinaryPaths) -> Result<(), Box<dyn std::error::Error>> {
    // Run the createdb module
    // If there already is a database, we don't need to create it again
    // Check if {input} file exists
    let output = args.createdb_output.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("createdb - output".to_string())); });
    if !std::path::Path::new(&output).exists() {
        mprintln(&"Running createdb module".to_string(), 3);
        createdb(args, bin)?;
    } else {
        mprintln(&"Database already exists, skipping createdb module".to_string(), 3);
    }

    // Run the cluster module
    let output: String = args.cluster_output.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("cluster - output".to_string())); });
    // Check if {output}.tsv file exists
    if !std::path::Path::new(&format!("{}.tsv", output)).exists() {
        mprintln(&"Running cluster module".to_string(), 3);
        cluster(args, bin)?;
    } else {
        mprintln(&"Clustered database already exists, skipping cluster module".to_string(), 3);
    }

    // Run the profile module
    // Check if {output} directory has files in it
    let output = args.profile_output.clone().unwrap_or_else(|| { crate::envs::error_handler::error(crate::envs::error_handler::ERR_ARGPARSE, Some("profile - output".to_string())); });
    if !std::path::Path::new(&output).exists()
    || std::fs::read_dir(&output)?.count() > 0 {
        mprintln(&"Running profile module".to_string(), 3);
        profile(args, bin)?;
    } else {
        mprintln(&"Profiled database already exists, skipping profile module".to_string(), 3);
    }

    // Run the tree module
    // Check if {output} directory has files in it
    let output = args.tree_output.clone().unwrap_or_else(|| { crate::envs::error_handler::error(crate::envs::error_handler::ERR_ARGPARSE, Some("tree - output".to_string())); });
    if !std::path::Path::new(&output).exists()
    || std::fs::read_dir(&output)?.count() == 0 {
        mprintln(&"Running tree module".to_string(), 3);
        tree(args, bin)?;
    } else {
        mprintln(&"Tree output directory not empty, skipping tree module".to_string(), 3);
    }

    Ok(())
}