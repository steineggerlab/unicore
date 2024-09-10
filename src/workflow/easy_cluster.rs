use crate::util::arg_parser::Args;
use crate::envs::variables as var;
// use crate::envs::error_handler as err;

use crate::modules::createdb::run as createdb;
use crate::modules::cluster::run as cluster;
use crate::modules::profile::run as profile;
use crate::modules::tree::run as tree;

pub fn run(args: &Args, bin: &var::BinaryPaths) -> Result<(), Box<dyn std::error::Error>> {
    // Run the createdb module
    println!("Running createdb module");
    createdb(args, bin)?;

    // Run the cluster module
    println!("Running cluster module");
    cluster(args, bin)?;

    // Run the profile module
    println!("Running profile module");
    profile(args, bin)?;

    // Run the tree module
    println!("Running tree module");
    tree(args, bin)?;

    Ok(())
}