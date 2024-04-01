use std::path::PathBuf;
use clap::{Parser, Subcommand};
use crate::var;

#[derive(Parser)]
#[clap(name = "unicore", version = var::VERSION, about = format!("Unicore v{}\nPhylogenetic inference with Universal core gene", var::VERSION))]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create foldseek database from amino acid sequences
    createdb {
        /// Input fasta file
        input: Option<PathBuf>,
        /// Output database
        output_db: Option<PathBuf>,
    },
    /// Search foldseek database against swissprot
    search {
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
        #[arg(long)]
        tmp: PathBuf,
        /// Delete tmp directory
        #[arg(short, long, default_value="true")]
        delete_tmp: bool,
    },
    /// Create core structures from
    profile {
        /// Input m8 file
        input: PathBuf,
        /// Output directory
        output: PathBuf,
        /// Output copiness tsv
        #[arg(short, long, default_value="true")]
        print_copiness: bool,
    },
    /// Inference phylogenetic tree from
    tree {
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
    }
}