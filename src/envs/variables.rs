use crate::envs::error_handler as err;

// global variables
pub const VERSION: &str = "0.1.0";
const STABLE: bool = false;
pub const STABLE_TEXT: &str = if STABLE { "" } else { "unstable" };
pub const STABLE_FULL: &str = if STABLE { "Stable" } else { "Unstable" };
pub const LOGO_ART: &str = r"
██╗   ██╗███╗   ██╗██╗ ██████╗ ██████╗ ██████╗ ███████╗
██║   ██║████╗  ██║██║██╔════╝██╔═══██╗██╔══██╗██╔════╝
██║   ██║██╔██╗ ██║██║██║     ██║   ██║██████╔╝███████╗
██║   ██║██║╚██╗██║██║██║     ██║   ██║██╔══██╗██╔══╝
╚██████╔╝██║ ╚████║██║╚██████╗╚██████╔╝██║  ██║███████╗
 ╚═════╝ ╚═╝  ╚═══╝╚═╝ ╚═════╝ ╚═════╝ ╚═╝  ╚═╝╚══════╝";

// environment paths
pub fn parent_dir() -> String {
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

pub fn current_dir() -> String {
    std::env::current_dir()
        .unwrap_or_else(|_| err::error(err::ERR_GENERAL, Some("Could not get current directory".to_string())))
        .to_str()
        .unwrap_or_else(|| err::error(err::ERR_GENERAL, Some("Could not convert path to string".to_string())))
        .to_string()
}

// binary paths
const VALID_BINARY: [&str; 7] = [
    "mmseqs", "foldseek", "mafft", "mafft-linsi", "foldmason", "iqtree", "fasttree",
];
pub struct Binary {
    name: String,
    pub path: String,
}
impl Binary {
    fn new(name: &str, path: &str) -> Self {
        Binary {
            name: name.to_string(),
            path: path.to_string(),
        }
    }
    fn test(&self, args: Vec<&str>) -> bool {
        std::process::Command::new(&self.path)
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
            if let Some(&i) = self.map.get(name) {
                self.bin[i].path = path.to_string();
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