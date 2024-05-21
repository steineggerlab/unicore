use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(disable_version_flag = true, arg_required_else_help = true)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
    /// Print version and information
    #[arg(short, long)]
    pub version: bool,
}

// Check if the threshold is in range
fn threshold_in_range(s: &str) -> Result<i32, String> {
    let threshold: i32 = s.parse().map_err(|_| "Not a number".to_string())?;
    if threshold < 0 || threshold > 100 {
        Err(format!("Threshold `{}` is not in range 0 to 100", s))
    } else {
        Ok(threshold)
    }
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create Foldseek database from amino acid sequences
    #[clap(arg_required_else_help = true, allow_hyphen_values = true, verbatim_doc_comment)]
    Createdb {
        /// Input directory with fasta files or a single fasta file
        input: PathBuf,
        /// Output foldseek database
        output: PathBuf,
        /// ProstT5 model
        model: PathBuf,
        /// Keep intermediate files
        #[arg(short, long, default_value="false")]
        keep: bool,
        /// Force overwrite output database
        #[arg(short, long, default_value="false")]
        overwrite: bool,
        /// Set maximum sequence length threshold
        #[arg(long)]
        max_len: Option<usize>,
/* TODO: Implement optional arguments
        /// Custom foldseek binary
        #[arg(long)]
        foldseek: Option<PathBuf>,
        /// Custom foldseek options
        #[arg(long)]
        foldseek_options: Option<String>,
 */
    },
    /// Search Foldseek database against reference database
    #[clap(arg_required_else_help = true, allow_hyphen_values = true)]
    Search {
        /// Input database
        input: PathBuf,
        /// Target database to search against
        target: PathBuf,
        /// Output prefix; the result will be saved as OUTPUT.m8
        output: PathBuf,
        /// Temp directory
        tmp: PathBuf,
        /// Keep intermediate Foldseek alignment database
        #[arg(short, long, default_value="false")]
        keep_aln_db: bool,
        /// Arguments for foldseek options in string e.g. -s "-c 0.8"
        #[arg(short, long, default_value="-c 0.8")]
        search_options: String,
    },
    /// Create core structures from Foldseek database
    #[clap(arg_required_else_help = true)]
    Profile {
        /// Input database (createdb output)
        input_db: PathBuf,
        /// Input m8 file (search output)
        input_m8: PathBuf,
        /// Output directory
        output: PathBuf,
        /// Coverage threshold for core structures. [0 - 100]
        #[arg(short, long, default_value="80", value_parser = threshold_in_range)]
        threshold: usize,
        /// Generate tsv with copy number statistics
        #[arg(short, long, default_value="true")]
        print_copiness: bool,
    },
    /// Infer phylogenetic tree from core structures
    #[clap(arg_required_else_help = true, allow_hyphen_values = true)]
    Tree {
        /// Proteome database
        proteome_db: PathBuf,
        /// Input directory containing core structures
        input: PathBuf,
        /// Output directory
        output: PathBuf,
        /// Alignment method
        #[arg(short, long, default_value="foldmason")]
        aligner: String,
        /// Tree method
        #[arg(short, long, default_value="iqtree")]
        tree_method: String,
        /// Options for aligner
        #[arg(short='o', long, default_value="")]
        aligner_options: String,
        /// Options for tree method
        #[arg(short='p', long, default_value="-m JTT+F+I+G -B 1000")]
        tree_options: String,
        /// Threshold for MSA filtering
        #[arg(short='d', long, default_value="0.5", value_parser = threshold_in_range)]
        threshold: f32,
    },
}