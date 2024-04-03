use crate::util::arg_parser as parser;
use crate::envs::variables as var;

pub fn run(_: &parser::Args, _: &var::BinaryPaths) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}