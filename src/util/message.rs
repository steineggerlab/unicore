use std::io::Write;
use crate::envs::variables::verbosity as system_verbosity;
use crate::envs::error_handler as err;
pub fn print_message(msg: &String, verbosity: u8) {
    if verbosity <= system_verbosity() {
        print!("{}", msg);
        match std::io::stdout().flush() {
            Ok(_) => {},
            Err(e) => err::error(err::ERR_GENERAL, Some(e.to_string())),
        }
    }
}
pub fn println_message(msg: &String, verbosity: u8) {
    if verbosity <= system_verbosity() {
        println!("{}", msg);
    }
}
pub fn eprintln_message(msg: &String, verbosity: u8) {
    if verbosity <= system_verbosity() {
        eprintln!("{}", msg);
    }
}