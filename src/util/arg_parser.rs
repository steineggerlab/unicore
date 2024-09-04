use std::path::PathBuf;
use clap::{Parser, Subcommand};
use crate::util::arg_parser::Commands::*;

#[derive(Parser)]
#[clap(disable_version_flag = true, arg_required_else_help = true)]
pub struct ClapArgs {
    #[command(subcommand)]
    pub command: Option<Commands>,
    /// Print version and information
    #[arg(short, long)]
    pub version: bool,
}

// Check if the threshold is in range
fn threshold_in_range(s: &str) -> Result<usize, String> {
    let threshold: usize = s.parse().map_err(|_| "Not a number".to_string())?;
    if threshold > 100 {
        Err(format!("Threshold `{}` is not in range 0 to 100", s))
    } else {
        Ok(threshold)
    }
}
fn _threshold_in_range_f64(s: &str) -> Result<f64, String> {
    let threshold: f64 = s.parse().map_err(|_| "Not a number".to_string())?;
    if threshold < 0.0 || threshold > 1.0 {
        Err(format!("Threshold `{}` is not in range 0.0 to 1.0", s))
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
        // Use python script instead. hidden option
        #[arg(long, default_value="false", hide = true)]
        use_python: bool,
/* TODO: Implement optional arguments
        /// Custom foldseek binary
        #[arg(long)]
        foldseek: Option<PathBuf>,
        /// Custom foldseek options
        #[arg(long)]
        foldseek_options: Option<String>,
 */
    },
    /// Cluster Foldseek database
    #[clap(arg_required_else_help = true, allow_hyphen_values = true)]
    Cluster {
        /// Input database (createdb output)
        input: PathBuf,
        /// Output prefix; the result will be saved as OUTPUT.tsv
        output: PathBuf,
        /// Temp directory
        tmp: PathBuf,
        /// Keep intermediate Foldseek cluster database
        #[arg(short, long, default_value="false")]
        keep_cluster_db: bool,
        /// Coverage threshold for core structures. [0 - 100]
        #[arg(short, long, default_value="-c 0.8")]
        cluster_options: String,
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
        /// Input m8 file (cluster or search output)
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
        /// Input database (createdb output)
        db: PathBuf,
        /// Input directory containing core structures (profile output)
        input: PathBuf,
        /// Output directory
        output: PathBuf,
        /// Multiple sequence aligner [foldmason, mafft-linsi, mafft]
        #[arg(short, long, default_value="foldmason")]
        aligner: String,
        /// Phylogenetic tree builder [iqtree, fasttree (under development), raxml (under development)]
        #[arg(short, long, default_value="iqtree")]
        tree_builder: String,
        /// Options for sequence aligner
        #[arg(short='o', long)]
        aligner_options: Option<String>,
        /// Options for tree builder; please adjust if using different tree method
        #[arg(short='p', long, default_value="-m JTT+F+I+G -B 1000")]
        tree_options: String,
        /// Gap threshold for multiple sequence alignment [0 - 100]
        #[arg(short='d', long, default_value="50", value_parser = threshold_in_range)]
        threshold: usize,
        /// Number of threads to use
        #[arg(short='c', long, default_value="0")]
        threads: usize,
    },
}

pub struct Args {
    pub command: Option<Commands>,
    pub version: bool,

    pub createdb_input: Option<String>,
    pub createdb_output: Option<String>,
    pub createdb_model: Option<String>,
    pub createdb_keep: Option<bool>,
    pub createdb_overwrite: Option<bool>,
    pub createdb_max_len: Option<Option<usize>>,
    pub createdb_use_python: Option<bool>,

    pub profile_input_db: Option<String>,
    pub profile_input_m8: Option<String>,
    pub profile_output: Option<String>,
    pub profile_threshold: Option<usize>,
    pub profile_print_copiness: Option<bool>,

    pub search_input: Option<String>,
    pub search_target: Option<String>,
    pub search_output: Option<String>,
    pub search_tmp: Option<String>,
    pub search_keep_aln_db: Option<bool>,
    pub search_search_options: Option<String>,

    pub cluster_input: Option<String>,
    pub cluster_output: Option<String>,
    pub cluster_tmp: Option<String>,
    pub cluster_keep_cluster_db: Option<bool>,
    pub cluster_cluster_options: Option<String>,

    pub tree_db: Option<String>,
    pub tree_input: Option<String>,
    pub tree_output: Option<String>,
    pub tree_aligner: Option<String>,
    pub tree_tree_builder: Option<String>,
    pub tree_aligner_options: Option<Option<String>>,
    pub tree_tree_options: Option<String>,
    pub tree_threshold: Option<usize>,
    pub tree_threads: Option<usize>,
}
fn own(path: &PathBuf) -> String { path.clone().to_string_lossy().into_owned() }
impl Args {
    pub fn parse() -> Self {
        let args = ClapArgs::parse();

        let createdb_input = match &args.command {
            Some(Createdb { input, .. }) => Some(own(input)), _ => None,
        };
        let createdb_output = match &args.command {
            Some(Createdb { output, .. }) => Some(own(output)), _ => None,
        };
        let createdb_model = match &args.command {
            Some(Createdb { model, .. }) => Some(own(model)), _ => None,
        };
        let createdb_keep = match &args.command {
            Some(Createdb { keep, .. }) => Some(*keep), _ => None,
        };
        let createdb_overwrite = match &args.command {
            Some(Createdb { overwrite, .. }) => Some(*overwrite), _ => None,
        };
        let createdb_max_len = match &args.command {
            Some(Createdb { max_len, .. }) => Some(max_len.clone()), _ => None,
        };
        let createdb_use_python = match &args.command {
            Some(Createdb { use_python, .. }) => Some(*use_python), _ => None,
        };

        let profile_input_db = match &args.command {
            Some(Profile { input_db, .. }) => Some(own(input_db)), _ => None,
        };
        let profile_input_m8 = match &args.command {
            Some(Profile { input_m8, .. }) => Some(own(input_m8)), _ => None,
        };
        let profile_output = match &args.command {
            Some(Profile { output, .. }) => Some(own(output)), _ => None,
        };
        let profile_threshold = match &args.command {
            Some(Profile { threshold, .. }) => Some(*threshold), _ => None,
        };
        let profile_print_copiness = match &args.command {
            Some(Profile { print_copiness, .. }) => Some(*print_copiness), _ => None,
        };

        let search_input = match &args.command {
            Some(Search { input, .. }) => Some(own(input)), _ => None,
        };
        let search_target = match &args.command {
            Some(Search { target, .. }) => Some(own(target)), _ => None,
        };
        let search_output = match &args.command {
            Some(Search { output, .. }) => Some(own(output)), _ => None,
        };
        let search_tmp = match &args.command {
            Some(Search { tmp, .. }) => Some(own(tmp)), _ => None,
        };
        let search_keep_aln_db = match &args.command {
            Some(Search { keep_aln_db, .. }) => Some(*keep_aln_db), _ => None,
        };
        let search_search_options = match &args.command {
            Some(Search { search_options, .. }) => Some(search_options.clone()), _ => None,
        };

        let cluster_input = match &args.command {
            Some(Cluster { input, .. }) => Some(own(input)), _ => None,
        };
        let cluster_output = match &args.command {
            Some(Cluster { output, .. }) => Some(own(output)), _ => None,
        };
        let cluster_tmp = match &args.command {
            Some(Cluster { tmp, .. }) => Some(own(tmp)), _ => None,
        };
        let cluster_keep_cluster_db = match &args.command {
            Some(Cluster { keep_cluster_db, .. }) => Some(*keep_cluster_db), _ => None,
        };
        let cluster_cluster_options = match &args.command {
            Some(Cluster { cluster_options, .. }) => Some(cluster_options.clone()), _ => None,
        };

        let tree_db = match &args.command {
            Some(Tree { db, .. }) => Some(own(db)), _ => None,
        };
        let tree_input = match &args.command {
            Some(Tree { input, .. }) => Some(own(input)), _ => None,
        };
        let tree_output = match &args.command {
            Some(Tree { output, .. }) => Some(own(output)), _ => None,
        };
        let tree_aligner = match &args.command {
            Some(Tree { aligner, .. }) => Some(aligner.clone()), _ => None,
        };
        let tree_tree_builder = match &args.command {
            Some(Tree { tree_builder, .. }) => Some(tree_builder.clone()), _ => None,
        };
        let tree_aligner_options = match &args.command {
            Some(Tree { aligner_options, .. }) => Some(aligner_options.clone()), _ => None,
        };
        let tree_tree_options = match &args.command {
            Some(Tree { tree_options, .. }) => Some(tree_options.clone()), _ => None,
        };
        let tree_threshold = match &args.command {
            Some(Tree { threshold, .. }) => Some(*threshold), _ => None,
        };
        let tree_threads = match &args.command {
            Some(Tree { threads, .. }) => Some(*threads), _ => None,
        };

        Args {
            command: args.command, version: args.version,
            createdb_input, createdb_output, createdb_model, createdb_keep, createdb_overwrite, createdb_max_len, createdb_use_python,
            profile_input_db, profile_input_m8, profile_output, profile_threshold, profile_print_copiness,
            search_input, search_target, search_output, search_tmp, search_keep_aln_db, search_search_options,
            cluster_input, cluster_output, cluster_tmp, cluster_keep_cluster_db, cluster_cluster_options,
            tree_db, tree_input, tree_output, tree_aligner, tree_tree_builder, tree_aligner_options, tree_tree_options, tree_threshold, tree_threads,
        }
    }
}