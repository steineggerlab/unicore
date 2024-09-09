// Basic error handling for the environment
// Recieves error code and message, and prints the message to stderr
use crate::util::message;

pub const WRN_GENERAL: i32 = 0x00;
pub const ERR_GENERAL: i32 = 0x01;
pub const ERR_FILE_NOT_FOUND: i32 = 0x02;
pub const ERR_MODULE_NOT_IMPLEMENTED: i32 = 0x03;
pub const ERR_ARGPARSE: i32 = 0x04;
pub const ERR_BINARY_NOT_FOUND: i32 = 0x05;
pub const ERR_OUTPUT_EXISTS: i32 = 0x06;

fn build_message(code: i32, passed_object: Option<String>) -> String {
    let object = passed_object.unwrap_or_else(|| "".to_string());
    match code {
        WRN_GENERAL => format!("Warning: {}", object),
        ERR_GENERAL => format!("Error: {}", object),
        ERR_FILE_NOT_FOUND => format!("File not found: {}", object),
        ERR_MODULE_NOT_IMPLEMENTED => format!("Module not implemented: {}", object),
        ERR_ARGPARSE => format!("Argument parsing error: {}", object),
        ERR_BINARY_NOT_FOUND => format!("Binary not found: {}", object),
        ERR_OUTPUT_EXISTS => format!("Output file already exists: {}; use -o to overwrite", object),
        _ => "Unknown error".to_string(),
    }
}

// warning: prints message to stderr
#[allow(unused)]
pub fn warning(code: i32, passed_object: Option<String>) {
    message::eprintln_message(&build_message(code, passed_object), 2);
}

// error: prints message to stderr and exits with code
pub fn error(code: i32, passed_object: Option<String>) -> ! {
    message::eprintln_message(&build_message(code, passed_object), 1);
    std::process::exit(code);
}