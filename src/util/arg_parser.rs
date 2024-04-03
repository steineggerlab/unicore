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

#[derive(Subcommand)]
pub enum Commands {
    /// Create Foldseek database from amino acid sequences
    #[clap(arg_required_else_help = true)]
    Createdb {
        /// Input fasta file
        input: Option<PathBuf>,
        /// Output database
        output_db: Option<PathBuf>,
    },
    /// Search Foldseek database against reference database
    #[clap(arg_required_else_help = true)]
    Search {
        /// Input db
        #[arg(short, long)]
        input_db: PathBuf,
        /// Database to search against
        #[arg(short, long)]
        target_db: PathBuf,
        /// Output db
        #[arg(short, long)]
        output_db: PathBuf,
        /// tmp directory
        #[arg(short = 'm', long)]
        tmp: PathBuf,
        /// Delete tmp directory
        #[arg(short, long, default_value="true")]
        delete_tmp: bool,
    },
    /// Create core structures from Foldseek database
    #[clap(arg_required_else_help = true)]
    Profile {
        /// Input m8 file
        input: PathBuf,
        /// Output directory
        output: PathBuf,
        /// Output copiness tsv
        #[arg(short, long, default_value="true")]
        print_copiness: bool,
    },
    /// Infer phylogenetic tree from core structures
    #[clap(arg_required_else_help = true)]
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