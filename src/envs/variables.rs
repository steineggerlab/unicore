use crate::envs::error_handler as err;
use crate::util::message as msg;
use std::fs::File;
use std::path::MAIN_SEPARATOR as SEP;
use std::process::Command;
use num_cpus;

// global variables
pub const VERSION: &str = "1.1.1";
const STABLE: bool = true;
pub const STABLE_TEXT: &str = if STABLE { "" } else { "unstable" };
pub const STABLE_FULL: &str = if STABLE { "Stable" } else { "Unstable" };
pub const LOGO_ART: &str = r"
██╗   ██╗███╗   ██╗██╗ ██████╗ ██████╗ ██████╗ ███████╗
██║   ██║████╗  ██║██║██╔════╝██╔═══██╗██╔══██╗██╔════╝
██║   ██║██╔██╗ ██║██║██║     ██║   ██║██████╔╝███████╗
██║   ██║██║╚██╗██║██║██║     ██║   ██║██╔══██╗██╔══╝
╚██████╔╝██║ ╚████║██║╚██████╗╚██████╔╝██║  ██║███████╗
 ╚═════╝ ╚═╝  ╚═══╝╚═╝ ╚═════╝ ╚═════╝ ╚═╝  ╚═╝╚══════╝";

pub const CITATION: &str = "Kim, D., Park, S., & Steinegger, M. (2025). Unicore enables scalable and accurate phylogenetic reconstruction with structural core genes. Genome Biology and Evolution, evaf109";

// environment paths
pub fn parent_dir() -> String {
    // assume binary path = parent/bin/unicore
    std::env::current_exe()
        .unwrap_or_else(|_| err::error(err::ERR_GENERAL, Some("Could not get current directory".to_string())))
        .parent().unwrap_or_else(|| err::error(err::ERR_GENERAL, Some("Could not get parent directory".to_string())))
        .parent().unwrap_or_else(|| err::error(err::ERR_GENERAL, Some("Could not get parent directory".to_string())))
        .to_str()
        .unwrap_or_else(|| err::error(err::ERR_GENERAL, Some("Could not convert path to string".to_string())))
        .to_string()
}
pub fn src_parent_dir() -> String {
    // assume binary path = parent/target/release/unicore
    std::env::current_exe()
        .unwrap_or_else(|_| err::error(err::ERR_GENERAL, Some("Could not get current directory".to_string())))
        .parent().unwrap_or_else(|| err::error(err::ERR_GENERAL, Some("Could not get parent directory".to_string())))
        .parent().unwrap_or_else(|| err::error(err::ERR_GENERAL, Some("Could not get parent directory".to_string())))
        .parent().unwrap_or_else(|| err::error(err::ERR_GENERAL, Some("Could not get parent directory".to_string())))
        .to_str()
        .unwrap_or_else(|| err::error(err::ERR_GENERAL, Some("Could not convert path to string".to_string())))
        .to_string()
}
pub fn test_parent_dir() -> String {
    // assume binary path = parent/target/debug/deps/unicore-*
    std::env::current_exe()
        .unwrap_or_else(|_| err::error(err::ERR_GENERAL, Some("Could not get current directory".to_string())))
        .parent().unwrap_or_else(|| err::error(err::ERR_GENERAL, Some("Could not get parent directory".to_string())))
        .parent().unwrap_or_else(|| err::error(err::ERR_GENERAL, Some("Could not get parent directory".to_string())))
        .parent().unwrap_or_else(|| err::error(err::ERR_GENERAL, Some("Could not get parent directory".to_string())))
        .parent().unwrap_or_else(|| err::error(err::ERR_GENERAL, Some("Could not get parent directory".to_string())))
        .to_str()
        .unwrap_or_else(|| err::error(err::ERR_GENERAL, Some("Could not convert path to string".to_string())))
        .to_string()
}

pub fn current_dir() -> String {
    std::env::current_dir()
        .unwrap_or_else(|_| err::error(err::ERR_GENERAL, Some("Could not get current directory".to_string())))
        .to_str()
        .unwrap_or_else(|| err::error(err::ERR_GENERAL, Some("Could not convert path to string".to_string())))
        .to_string()
}

pub fn locate_path_cfg() -> String {
    if File::open(format!("{}{}etc{}path.cfg", parent_dir(), SEP, SEP)).is_ok() {
        format!("{}{}etc{}path.cfg", parent_dir(), SEP, SEP)
    } else if File::open(format!("{}{}path.cfg", src_parent_dir(), SEP)).is_ok() {
        format!("{}{}path.cfg", src_parent_dir(), SEP)
    } else if File::open(format!("{}{}path.cfg", test_parent_dir(), SEP)).is_ok() {
        format!("{}{}path.cfg", test_parent_dir(), SEP)
    } else {
        err::error(err::ERR_GENERAL, Some("Could not locate path.cfg".to_string()));
    }
}

// binary paths
pub const VALID_BINARY: [&str; 8] = [
    "mmseqs", "foldseek", "mafft", "mafft-linsi", "foldmason", "iqtree", "fasttree", "raxml-ng"
];
pub struct Binary {
    name: String,
    pub path: String,
    pub set: bool,
}
impl Binary {
    fn new(name: &str, path: &str) -> Self {
        Binary {
            name: name.to_string(),
            path: path.to_string(),
            set: false,
        }
    }
    fn test(&self, args: Vec<&str>) -> bool {
        Command::new(&self.path)
            .args(args)
            .output()
            .is_ok()
    }
}

use std::collections::HashMap;
pub struct BinaryPaths {
    bin: Vec<Binary>,
    map: HashMap<String, usize>,
}
impl BinaryPaths {
    pub fn new() -> Self {
        let mut bin = Vec::new();
        let mut map = HashMap::new();
        for (i, &name) in VALID_BINARY.iter().enumerate() {
            bin.push(Binary::new(name, name));
            map.insert(name.to_string(), i);
        }
        BinaryPaths { bin, map }
    }
    pub fn init(&mut self, config_path: &std::path::Path) -> Result<(), std::io::Error> {
        let config = std::fs::read_to_string(config_path)?;
        for line in config.lines() {
            if line.is_empty() || line.starts_with('#') { continue; }
            let mut split = line.split('=');
            let name = split.next().unwrap_or("");
            let path = split.next().unwrap_or("");
            if path.len() == 0 { continue; }
            if let Some(&i) = self.map.get(name) {
                self.bin[i].path = path.to_string();
                self.bin[i].set = true;
            }
        }
        Ok(())
    }
    pub fn get(&self, name: &str) -> Option<&Binary> {
        self.map.get(name).map(|&i| &self.bin[i])
    }
    pub fn set(&mut self, name: &str, path: &str) {
        if let Some(&i) = self.map.get(name) {
            self.bin[i].path = path.to_string();
        }
    }
    pub fn test(&self, name: &str, args: Vec<&str>) -> bool {
        self.get(name).map(|bin| bin.test(args)).unwrap_or(false)
    }
}

pub static mut VERBOSITY: u8 = 3;
pub fn set_verbosity(verbosity: u8) {
    unsafe { VERBOSITY = verbosity; }
}
pub fn verbosity() -> u8 {
    unsafe { VERBOSITY }
}

pub static mut THREADS: usize = 0;
pub fn set_threads(threads: usize) {
    let system_cpus = num_cpus::get();
    let threads = if threads > system_cpus {
        msg::eprintln_message(&format!("Warning: the given number of threads is greater than the number of system CPUs; adjusting to {}", system_cpus), 2);
        system_cpus
    } else { threads };
    let threads = if threads > 0 { threads } else {
        msg::println_message(&format!("Automatically setting the number of threads to {}", system_cpus), 4);
        system_cpus
    };
    unsafe { THREADS = threads; }
}
pub fn threads() -> usize {
    unsafe { THREADS }
}
