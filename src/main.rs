mod envs;
mod modules;
use envs::error_handler as err;
use envs::variables as var;

fn usage() {
    println!("Unicore v{} {}", var::VERSION, var::STABLE_TEXT);
    println!("Usage: unicore <module> [options]");
    println!();
    println!("Available modules:");
    println!("  version    : Print version and information");
    println!("  help       : Print this help message");
    println!("  more to come...");
    println!();
}

const VALID_MODULES: [&str; 4] = [
    "help", "usage", "info", "version",
];
fn init(args: &Vec<String>) -> var::BinaryPaths {
    // check module
    let module = args.iter().nth(1).unwrap_or_else(|| {
        err::warning(err::ERR_GENERAL, Some("No module specified".to_string()));
        usage();
        std::process::exit(err::ERR_GENERAL);
    });
    if !VALID_MODULES.contains(&module.as_str()) {
        err::warning(err::ERR_MODULE_NOT_FOUND, Some(module.to_string()));
        usage();
        std::process::exit(err::ERR_MODULE_NOT_FOUND);
    }

    // load path config
    let cfg_path = format!("{}{}path.cfg", var::parent_dir(), std::path::MAIN_SEPARATOR);
    let mut bin = var::BinaryPaths::new();
    bin.init(&std::path::Path::new(&cfg_path)).unwrap_or_else(|_| err::error(err::ERR_GENERAL, Some("Could not initialize binary paths".to_string())));
    bin
}

fn run(args: &Vec<String>, bin: var::BinaryPaths) -> Result<(), Box<dyn std::error::Error>> {
    match args[1].as_str() {
        "help" | "usage" => usage(),
        "version" | "info" => modules::version::run(&args, bin),
        _ => unreachable!(),
    }
    Ok(())
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let bin = init(&args);
    run(&args, bin).unwrap_or_else(|e| err::error(err::ERR_GENERAL, Some(e.to_string())));
}
