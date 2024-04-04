use std::path::PathBuf;
use clap::{Parser, Subcommand};
use crate::envs::error_handler as err;

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
fn threshold_in_range(s: &str) -> Result<f32, String> {
    let threshold: f32 = s.parse().map_err(|_| format!("Threshold `{}` is weird, please check", s))?;
    if threshold < 0.0 || threshold > 1.0 {
        Err(format!("Threshold `{}` is not in range 0 to 1", s))
    } else {
        Ok(threshold)
    }
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create Foldseek database from amino acid sequences
    /// combined_aa.fasta is a fasta file of all sequences in the input directory
    /// combined_3di.fasta is a fasta file of all sequences in the input directory converted into 3Di sequences
    #[clap(arg_required_else_help = true, allow_hyphen_values = true, verbatim_doc_comment)]
    Createdb {
        /// Input directory with fasta files
        input: PathBuf,
        /// Output foldseek database
        output: PathBuf,
        /// ProstT5 model
        model: PathBuf,
        /// Delete combined fasta files (combined_aa.fasta and combined_3di.fasta)
        #[arg(short, long, default_value="false")]
        delete_fasta: bool,
/* TODO: Implement optional arguments
        /// Custom foldseek binary
        #[arg(long)]
        foldseek: Option<PathBuf>,
        /// Custom foldseek options
        #[arg(long)]
        foldseek_options: Option<String>,
 */
    },
    /// Search Foldseek database against reference database.
    /// Name of the output m8 file will be OUTPUT_DB.m8
    #[clap(arg_required_else_help = true, allow_hyphen_values = true)]
    Search {
        /// Input db
        input_db: PathBuf,
        /// Database to search against
        target_db: PathBuf,
        /// Output db
        output_db: PathBuf,
        /// tmp directory
        tmp: PathBuf,
        /// Delete tmp directory
        #[arg(short, long, default_value="true")]
        delete_tmp: bool,
        /// Arguments for foldseek options in string i.e. -s "-c 0.8"
        #[arg(short, long, default_value="-c 0.8")]
        search_options: String,
    },
    /// Create core structures from Foldseek database
    #[clap(arg_required_else_help = true)]
    Profile {
        /// Input m8 file
        input: PathBuf,
        /// Gene to Species mapping tsv file
        mapping: PathBuf,
        /// Output directory
        output: PathBuf,
        /// Coverage threshold for core structures. Ranging from 0 to 1
        #[arg(short, long, default_value="0.8", value_parser = threshold_in_range)]
        threshold: f32,
        /// Output copiness tsv
        #[arg(short, long, default_value="true")]
        print_copiness: bool,
    },
    /// Infer phylogenetic tree from core structures
    #[clap(arg_required_else_help = true, allow_hyphen_values = true)]
    Tree {
        /// Input directory
        #[arg(short, long)]
        input: PathBuf,
        /// Output directory
        #[arg(short, long)]
        output: PathBuf,
        /// Alignment method
        #[arg(short, long, default_value="foldmason")]
        aligner: String,
        /// Tree method
        #[arg(short, long, default_value="iqtree")]
        tree_method: String,
    },
}