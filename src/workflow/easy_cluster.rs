use crate::util::arg_parser::Args;
use crate::util::message::println_message as mprintln;
use crate::util::checkpoint::read_checkpoint as read_chkpnt;
use crate::envs::variables as var;
use crate::envs::error_handler as err;

use crate::modules::createdb::run as createdb;
use crate::modules::cluster::run as cluster;
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
    if std::path::Path::new(&format!("{}/createdb.chk", parent)).exists() {
        let content = read_chkpnt(&format!("{}/createdb.chk", parent))?;
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

    // Run the cluster module
    let output: String = args.cluster_output.clone().unwrap_or_else(|| { err::error(err::ERR_ARGPARSE, Some("cluster - output".to_string())); });
    // Try to obtain the parent directory of the output
    let parent = if let Some(p) = Path::new(&output).parent() {
        p.to_string_lossy().into_owned()
    } else {
        err::error(err::ERR_GENERAL, Some("Could not obtain parent directory of the output".to_string()))
    };
    // Check if the checkpoint file exists
    if std::path::Path::new(&format!("{}/cluster.chk", parent)).exists() {
        let content = read_chkpnt(&format!("{}/cluster.chk", parent))?;
        if content == "1" {
            mprintln(&"Clustered database already exists, skipping cluster module".to_string(), 3);
        } else {
            mprintln(&"Running cluster module".to_string(), 3);
            cluster(args, bin)?;
        }
    } else {
        mprintln(&"Running cluster module".to_string(), 3);
        cluster(args, bin)?;
    }

    // Run the profile module
    // Check if {output} directory has a checkpoint file
    let output = args.profile_output.clone().unwrap_or_else(|| { crate::envs::error_handler::error(crate::envs::error_handler::ERR_ARGPARSE, Some("profile - output".to_string())); });
    if std::path::Path::new(&format!("{}/profile.chk", output)).exists() {
        let content = read_chkpnt(&format!("{}/profile.chk", output))?;
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
    if std::path::Path::new(&format!("{}/tree.chk", output)).exists() {
        let content = read_chkpnt(&format!("{}/tree.chk", output))?;
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