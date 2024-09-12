use crate::util::arg_parser::Args;
use crate::util::message::println_message as mprintln;
use crate::util::checkpoint::read_checkpoint as read_chkpnt;
use crate::envs::variables as var;
use crate::envs::error_handler as err;

use crate::modules::createdb::run as createdb;
use crate::modules::search::run as search;
use crate::modules::profile::run as profile;
use crate::modules::tree::run as tree;

use std::path::Path;

pub fn run(args: &Args, bin: &var::BinaryPaths) -> Result<(), Box<dyn std::error::Error>> {
    // Run the createdb module
    // If there already is a database, we don't need to create it again
    // Check if the checkpoint file exists
    let output = args.createdb_output.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("createdb - output".to_string())); });
    let overwrite = args.createdb_overwrite.unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("createdb - overwrite".to_string())); });
    // Try to obtain the parent directory of the output
    let parent = if let Some(p) = Path::new(&output).parent() {
        p.to_string_lossy().into_owned()
    } else {
        err::error(err::ERR_GENERAL, Some("Could not obtain parent directory of the output".to_string()))
    };

    // Read in the checkpoint file and check if createdb has been run
    if std::path::Path::new(&format!("{}/createdb.txt", parent)).exists() {
        let content = read_chkpnt(&format!("{}/createdb.txt", parent))?;
        if overwrite || content == "0" {
            mprintln(&"Running createdb module".to_string(), 3);
            createdb(args, bin)?;
        } else {
            mprintln(&"Database already exists, skipping createdb module".to_string(), 3);
        }
    } else {
        mprintln(&"Running createdb module".to_string(), 3);
        createdb(args, bin)?;
    }

    // Run the search module
    let output: String = args.search_output.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("search - output".to_string())); });
    // Try to obtain the parent directory of the output
    let parent = if let Some(p) = Path::new(&output).parent() {
        p.to_string_lossy().into_owned()
    } else {
        err::error(err::ERR_GENERAL, Some("Could not obtain parent directory of the output".to_string()))
    };
    // Check if the checkpoint file exists
    if std::path::Path::new(&format!("{}/search.txt", parent)).exists() {
        let content = read_chkpnt(&format!("{}/search.txt", parent))?;
        if content == "1" {
            mprintln(&"Search database already exists, skipping search module".to_string(), 3);
        } else {
            mprintln(&"Running search module".to_string(), 3);
            search(args, bin)?;
        }
    } else {
        mprintln(&"Running search module".to_string(), 3);
        search(args, bin)?;
    }

    // Run the profile module
    // Check if {output} directory has a checkpoint file
    let output: String = args.profile_output.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("profile - output".to_string())); });
    if std::path::Path::new(&format!("{}/profile.txt", output)).exists() {
        let content = read_chkpnt(&format!("{}/profile.txt", output))?;
        if content == "1" {
            mprintln(&"Profiled database already exists, skipping profile module".to_string(), 3);
        } else {
            mprintln(&"Running profile module".to_string(), 3);
            profile(args, bin)?;
        }
    } else {
        mprintln(&"Running profile module".to_string(), 3);
        profile(args, bin)?;
    }

    // Run the tree module
    // Check if {output} directory has a checkpoint file
    let output = args.tree_output.clone().unwrap_or_else(|| { crate::envs::error_handler::error(crate::envs::error_handler::ERR_ARGPARSE, Some("tree - output".to_string())); });
    if std::path::Path::new(&format!("{}/tree.txt", output)).exists() {
        let content = read_chkpnt(&format!("{}/tree.txt", output))?;
        if content == "1" {
            mprintln(&"Tree output directory not empty, skipping tree module".to_string(), 3);
        } else {
            mprintln(&"Running tree module".to_string(), 3);
            tree(args, bin)?;
        }
    } else {
        mprintln(&"Running tree module".to_string(), 3);
        tree(args, bin)?;
    }

    Ok(())
}