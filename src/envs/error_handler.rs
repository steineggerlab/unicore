// Basic error handling for the environment
// Recieves error code and message, and prints the message to stderr

pub const ERR_GENERAL: i32 = 0x01;
pub const ERR_FILE_NOT_FOUND: i32 = 0x02;
pub const ERR_MODULE_NOT_FOUND: i32 = 0x03;

fn build_message(code: i32, passed_object: Option<String>) -> String {
    let object = passed_object.unwrap_or_else(|| "".to_string());
    match code {
        ERR_GENERAL => format!("Error: {}", object),
        ERR_FILE_NOT_FOUND => format!("File not found: {}", object),
        ERR_MODULE_NOT_FOUND => format!("Module not found: {}", object),
        _ => "Unknown error".to_string(),
    }
}

// warning: prints message to stderr
pub fn warning(code: i32, passed_object: Option<String>) {
    eprintln!("{}", build_message(code, passed_object));
}

// error: prints message to stderr and exits with code
pub fn error(code: i32, passed_object: Option<String>) -> ! {
    eprintln!("{}", build_message(code, passed_object));
    std::process::exit(code);
}