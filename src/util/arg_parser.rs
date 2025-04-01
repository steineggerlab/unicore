use std::path::PathBuf;
use clap::{Parser, Subcommand};
use color_print::cstr;
use crate::util::arg_parser::Commands::*;

#[derive(Parser)]
#[clap(disable_version_flag = true, arg_required_else_help = true)]
#[command(subcommand_value_name = "MODULE")]
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

// Extra help messages
const PROFILE_HELP: &str = cstr!(r#"<bold><underline>Example:</underline></bold>
  # Define core genes above 85% coverage threshold
  <bold>unicore profile -t 85 example/db/proteome_db example/out/clu.tsv result</bold>
"#);
const GENETREE_HELP: &str = cstr!(r#"<bold><underline>Example:</underline></bold>
  # Create a list of hashed gene names
  <bold>awk -F'\t' 'NR==FNR {a[$1];next} ($3 in a) {print $1}' /path/to/original/gene/names db/proteome_db.map > /path/to/hashed/gene/names</bold>
  # Run gene-tree with the list of hashed gene names; use --realign option to recompute the alignment with custom --threshold option for MSA gap threshold
  <bold>unicore gene-tree --realign --threshold 30 --name /path/to/hashed/gene/names example/tree</bold>
"#);
#[derive(Subcommand)]
#[command(subcommand_help_heading = "Modules")]
pub enum Commands {
    /// Easy core gene phylogeny workflow, from fasta files to phylogenetic tree
    #[clap(arg_required_else_help = true, allow_hyphen_values = true)]
    EasyCore {
        /// Input directory with fasta files or a single fasta file
        input: PathBuf,
        /// Output directory where all results will be saved
        output: PathBuf,
        /// ProstT5 model
        model: PathBuf,
        /// tmp directory
        tmp: PathBuf,
        /// Keep intermediate files
        #[arg(short, long, default_value="false")]
        keep: bool,
        /// Force overwrite output database
        #[arg(short='w', long, default_value="false")]
        overwrite: bool,
        /// Set maximum sequence length threshold
        #[arg(long)]
        max_len: Option<usize>,
        /// Use GPU for foldseek createdb
        #[arg(short, long, default_value="false")]
        gpu: bool,
        /// Use AFDB lookup for foldseek createdb. Useful for large databases
        #[arg(long)]
        afdb_lookup: Option<PathBuf>,
        /// Use custom lookup database, accepts any Foldseek database to reference against
        #[arg(long)]
        custom_lookup: Option<PathBuf>,
        /// Arguments for foldseek options in string e.g. -c "-c 0.8"
        #[arg(short, long, default_value="-c 0.8")]
        cluster_options: String,
        /// Coverage threshold for core structures. [0 - 100]
        #[arg(short='C', long, default_value="80", value_parser = threshold_in_range)]
        core_threshold: usize,
        /// Generate tsv with copy number statistics
        #[arg(short, long, default_value="true")]
        print_copiness: bool,
        /// Multiple sequence aligner [foldmason, mafft-linsi, mafft]
        #[arg(short='A', long, default_value="foldmason")]
        aligner: String,
        /// Phylogenetic tree builder [iqtree, fasttree (under development), raxml (under development)]
        #[arg(short='T', long, default_value="iqtree")]
        tree_builder: String,
        /// Options for sequence aligner
        #[arg(short, long)]
        aligner_options: Option<String>,
        /// Options for tree builder; please adjust if using different tree method
        #[arg(short, long, default_value="-m JTT+F+I+G -B 1000")]
        tree_options: String,
        /// Gap threshold for multiple sequence alignment [0 - 100]
        #[arg(short='G', long, default_value="50", value_parser = threshold_in_range)]
        gap_threshold: usize,
        /// Number of threads to use; 0 to use all
        #[arg(long, default_value="0")]
        threads: usize,
        /// Verbosity (0: quiet, 1: +errors, 2: +warnings, 3: +info, 4: +debug)
        #[arg(short='v', long, default_value="3")]
        verbosity: u8,
    },
    /// Easy search workflow, from fasta files to phylogenetic tree
    #[clap(arg_required_else_help = true, allow_hyphen_values = true, hide = true)]
    EasySearch {
        /// Input directory with fasta files or a single fasta file
        input: PathBuf,
        /// Target database to search against
        target: PathBuf,
        /// Output directory where all results will be saved
        output: PathBuf,
        /// ProstT5 model
        model: PathBuf,
        /// tmp directory
        tmp: PathBuf,
        /// Keep intermediate files
        #[arg(short, long, default_value="false")]
        keep: bool,
        /// Force overwrite output database
        #[arg(short='w', long, default_value="false")]
        overwrite: bool,
        /// Set maximum sequence length threshold
        #[arg(long)]
        max_len: Option<usize>,
        /// Use GPU for foldseek createdb
        #[arg(short, long, default_value="false")]
        gpu: bool,
        /// Use AFDB lookup for foldseek createdb. Useful for large databases
        #[arg(long)]
        afdb_lookup: Option<PathBuf>,
        /// Use custom lookup database, accepts any Foldseek database to reference against
        #[arg(long)]
        custom_lookup: Option<PathBuf>,
        /// Arguments for foldseek options in string e.g. -s "-c 0.8"
        #[arg(short, long, default_value="-c 0.8")]
        search_options: String,
        /// Coverage threshold for core structures. [0 - 100]
        #[arg(short='C', long, default_value="80", value_parser = threshold_in_range)]
        core_threshold: usize,
        /// Generate tsv with copy number statistics
        #[arg(short, long, default_value="true")]
        print_copiness: bool,
        /// Multiple sequence aligner [foldmason, mafft-linsi, mafft]
        #[arg(short='A', long, default_value="foldmason")]
        aligner: String,
        /// Phylogenetic tree builder [iqtree, fasttree (under development), raxml (under development)]
        #[arg(short='T', long, default_value="iqtree")]
        tree_builder: String,
        /// Options for sequence aligner
        #[arg(short, long)]
        aligner_options: Option<String>,
        /// Options for tree builder; please adjust if using different tree method
        #[arg(short, long, default_value="-m JTT+F+I+G -B 1000")]
        tree_options: String,
        /// Gap threshold for multiple sequence alignment [0 - 100]
        #[arg(short='G', long, default_value="50", value_parser = threshold_in_range)]
        gap_threshold: usize,
        /// Number of threads to use; 0 to use all
        #[arg(long, default_value="0")]
        threads: usize,
        /// Verbosity (0: quiet, 1: +errors, 2: +warnings, 3: +info, 4: +debug)
        #[arg(short='v', long, default_value="3")]
        verbosity: u8,
    },
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
        /// Use GPU for foldseek createdb
        #[arg(short, long, default_value="false")]
        gpu: bool,
        /// Use AFDB lookup for foldseek createdb. Useful for large databases
        #[arg(long)]
        afdb_lookup: Option<PathBuf>,
        /// Use custom lookup database, accepts any Foldseek database to reference against
        #[arg(long)]
        custom_lookup: Option<PathBuf>,
        /// Number of threads to use; 0 to use all
        #[arg(long, default_value="0")]
        threads: usize,
        /// Verbosity (0: quiet, 1: +errors, 2: +warnings, 3: +info, 4: +debug)
        #[arg(short='v', long, default_value="3")]
        verbosity: u8,
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
        /// Arguments for foldseek options in string e.g. -c "-c 0.8"
        #[arg(short, long, default_value="-c 0.8")]
        cluster_options: String,
        /// Number of threads to use; 0 to use all
        #[arg(long, default_value="0")]
        threads: usize,
        /// Verbosity (0: quiet, 1: +errors, 2: +warnings, 3: +info, 4: +debug)
        #[arg(short='v', long, default_value="3")]
        verbosity: u8,
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
        /// Number of threads to use; 0 to use all
        #[arg(long, default_value="0")]
        threads: usize,
        /// Verbosity (0: quiet, 1: +errors, 2: +warnings, 3: +info, 4: +debug)
        #[arg(short='v', long, default_value="3")]
        verbosity: u8,
    },
    /// Create core structures from Foldseek database
    #[clap(arg_required_else_help = true)]
    #[command(after_help=PROFILE_HELP)]
    Profile {
        /// Input database (createdb output)
        input_db: PathBuf,
        /// Input tsv file (cluster or search output)
        input_tsv: PathBuf,
        /// Output directory
        output: PathBuf,
        /// Coverage threshold for core structures. [0 - 100]
        #[arg(short, long, default_value="80", value_parser = threshold_in_range)]
        threshold: usize,
        /// Generate tsv with copy number statistics
        #[arg(short, long, default_value="true")]
        print_copiness: bool,
        /// Number of threads to use; 0 to use all
        #[arg(long, default_value="0")]
        threads: usize,
        /// Verbosity (0: quiet, 1: +errors, 2: +warnings, 3: +info, 4: +debug)
        #[arg(short='v', long, default_value="3")]
        verbosity: u8,
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
        /// Number of threads to use; 0 to use all
        #[arg(short='c', long, default_value="0")]
        threads: usize,
        /// Verbosity (0: quiet, 1: +errors, 2: +warnings, 3: +info, 4: +debug)
        #[arg(short='v', long, default_value="3")]
        verbosity: u8,
    },
    /// Infer phylogenetic tree of each core structures
    #[clap(arg_required_else_help = true, allow_hyphen_values = true)]
    #[command(after_help=GENETREE_HELP)]
    GeneTree {
        /// Input directory containing species phylogenetic tree (Output of the Tree module)
        input: PathBuf,
        /// File containing core structures for computing phylogenetic tree. If not provided, all core structures will be used
        #[arg(short='n', long, default_value="")]
        names: String,
        /// Phylogenetic tree builder [iqtree, fasttree (under development), raxml (under development)]
        #[arg(short='T', long, default_value="iqtree")]
        tree_builder: String,
        /// Options for tree builder; please adjust if using different tree method
        #[arg(short='p', long, default_value="-m JTT+F+I+G -B 1000")]
        tree_options: String,
        /// Compute the Multiple sequence alignment again. This will overwrite the previous alignment
        #[arg(short='f', long, default_value="false")]
        realign: bool,
        /// Multiple sequence aligner [foldmason, mafft-linsi, mafft]
        #[arg(short, long, default_value="foldmason")]
        aligner: String,
        /// Options for sequence aligner
        #[arg(short='o', long)]
        aligner_options: Option<String>,
        /// Gap threshold for multiple sequence alignment [0 - 100]
        #[arg(short='d', long, default_value="50", value_parser = threshold_in_range)]
        threshold: usize,
        /// Number of threads to use; 0 to use all
        #[arg(short='c', long, default_value="0")]
        threads: usize,
        /// Verbosity (0: quiet, 1: +errors, 2: +warnings, 3: +info, 4: +debug)
        #[arg(short='v', long, default_value="3")]
        verbosity: u8,
    },
    /// Runtime environment configuration
    #[clap(arg_required_else_help = true, allow_hyphen_values = true)]
    Config {
        /// Check current environment configuration
        #[arg(short='c', long)]
        check: bool,
        /// Set mmseqs binary path
        #[arg(long)]
        set_mmseqs: Option<PathBuf>,
        /// Set foldseek binary path
        #[arg(long)]
        set_foldseek: Option<PathBuf>,
        /// Set foldmason binary path
        #[arg(long)]
        set_foldmason: Option<PathBuf>,
        /// Set mafft binary path
        #[arg(long)]
        set_mafft: Option<PathBuf>,
        /// Set mafft-linsi binary path
        #[arg(long)]
        set_mafft_linsi: Option<PathBuf>,
        /// Set iqtree binary path
        #[arg(long)]
        set_iqtree: Option<PathBuf>,
        /// Set fasttree binary path
        #[arg(long)]
        set_fasttree: Option<PathBuf>,
        /// Set raxml binary path
        #[arg(long)]
        set_raxml: Option<PathBuf>,
        /// Verbosity (0: quiet, 1: +errors, 2: +warnings, 3: +info, 4: +debug)
        #[arg(short='v', long, default_value="3")]
        verbosity: u8,
    },
}

#[derive(Default)]
pub struct Args {
    pub command: Option<Commands>,
    pub version: bool,
    pub threads: usize,
    pub verbosity: u8,

    pub createdb_input: Option<String>,
    pub createdb_output: Option<String>,
    pub createdb_model: Option<String>,
    pub createdb_keep: Option<bool>,
    pub createdb_overwrite: Option<bool>,
    pub createdb_max_len: Option<Option<usize>>,
    pub createdb_gpu: Option<bool>,
    pub createdb_afdb_lookup: Option<Option<String>>,
    pub createdb_custom_lookup: Option<Option<String>>,

    pub profile_input_db: Option<String>,
    pub profile_input_tsv: Option<String>,
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

    pub genetree_input: Option<String>,
    pub genetree_names: Option<String>,
    pub genetree_tree_builder: Option<String>,
    pub genetree_tree_options: Option<String>,
    pub genetree_threshold: Option<usize>,
    pub genetree_realign: Option<bool>,
    pub genetree_aligner: Option<String>,
    pub genetree_aligner_options: Option<Option<String>>,

    pub config_check: Option<bool>,
    pub config_set_mmseqs: Option<String>,
    pub config_set_foldseek: Option<String>,
    pub config_set_foldmason: Option<String>,
    pub config_set_mafft: Option<String>,
    pub config_set_mafft_linsi: Option<String>,
    pub config_set_iqtree: Option<String>,
    pub config_set_fasttree: Option<String>,
    pub config_set_raxml: Option<String>,
}
fn own(path: &PathBuf) -> String { path.clone().to_string_lossy().into_owned() }
impl Args {
    pub fn parse() -> Self {
        let args = ClapArgs::parse();
        let verbosity = match &args.command {
            Some(Createdb { verbosity, .. }) => *verbosity,
            Some(Profile { verbosity, .. }) => *verbosity,
            Some(Search { verbosity, .. }) => *verbosity,
            Some(Cluster { verbosity, .. }) => *verbosity,
            Some(Tree { verbosity, .. }) => *verbosity,
            Some(GeneTree { verbosity, .. }) => *verbosity,
            Some(EasyCore { verbosity, .. }) => *verbosity,
            Some(EasySearch { verbosity, .. }) => *verbosity,
            Some(Config { verbosity, .. }) => *verbosity,
            _ => 3,
        };
        let threads = match &args.command {
            Some(Createdb { threads, .. }) => *threads,
            Some(Profile { threads, .. }) => *threads,
            Some(Search { threads, .. }) => *threads,
            Some(Cluster { threads, .. }) => *threads,
            Some(Tree { threads, .. }) => *threads,
            Some(GeneTree { threads, .. }) => *threads,
            Some(EasyCore { threads, .. }) => *threads,
            Some(EasySearch { threads, .. }) => *threads,
            _ => 0,
        };

        let createdb_input = match &args.command {
            Some(Createdb { input, .. }) => Some(own(input)),
            Some(EasyCore { input, .. }) => Some(own(input)),
            Some(EasySearch { input, .. }) => Some(own(input)), _ => None,
        };
        let createdb_output = match &args.command {
            Some(Createdb { output, .. }) => Some(own(output)),
            Some(EasyCore { output, .. }) => Some(format!("{}/proteome/proteome_db", own(output))),
            Some(EasySearch { output, .. }) => Some(format!("{}/proteome/proteome_db", own(output))), _ => None,
        };
        let createdb_model = match &args.command {
            Some(Createdb { model, .. }) => Some(own(model)),
            Some(EasyCore { model, .. }) => Some(own(model)),
            Some(EasySearch { model, .. }) => Some(own(model)), _ => None,
        };
        let createdb_keep = match &args.command {
            Some(Createdb { keep, .. }) => Some(*keep),
            Some(EasyCore { keep, .. }) => Some(*keep),
            Some(EasySearch { keep, .. }) => Some(*keep), _ => None,
        };
        let createdb_overwrite = match &args.command {
            Some(Createdb { overwrite, .. }) => Some(*overwrite),
            Some(EasyCore { overwrite, .. }) => Some(*overwrite),
            Some(EasySearch { overwrite, .. }) => Some(*overwrite), _ => None,
        };
        let createdb_max_len = match &args.command {
            Some(Createdb { max_len, .. }) => Some(max_len.clone()),
            Some(EasyCore { max_len, .. }) => Some(max_len.clone()),
            Some(EasySearch { max_len, .. }) => Some(max_len.clone()), _ => None,
        };
        let createdb_gpu = match &args.command {
            Some(Createdb { gpu, .. }) => Some(*gpu),
            Some(EasyCore { gpu, .. }) => Some(*gpu),
            Some(EasySearch { gpu, .. }) => Some(*gpu), _ => None,
        };
        let createdb_afdb_lookup = match &args.command {
            Some(Createdb { afdb_lookup, .. }) => match afdb_lookup { Some(p) => Some(Some(own(p))), _none => Some(None) },
            Some(EasyCore { afdb_lookup, .. }) => match afdb_lookup { Some(p) => Some(Some(own(p))), _none => Some(None) },
            Some(EasySearch { afdb_lookup, .. }) => match afdb_lookup { Some(p) => Some(Some(own(p))), _none => Some(None) }, _ => None,
        };
        let createdb_custom_lookup = match &args.command {
            Some(Createdb { custom_lookup, .. }) => match custom_lookup { Some(p) => Some(Some(own(p))), _none => Some(None) },
            Some(EasyCore { custom_lookup, .. }) => match custom_lookup { Some(p) => Some(Some(own(p))), _none => Some(None) },
            Some(EasySearch { custom_lookup, .. }) => match custom_lookup { Some(p) => Some(Some(own(p))), _none => Some(None) }, _ => None,
        };

        let profile_input_db = match &args.command {
            Some(Profile { input_db, .. }) => Some(own(input_db)),
            Some(EasyCore { output, .. }) => Some(format!("{}/proteome/proteome_db", own(output))),
            Some(EasySearch { output, .. }) => Some(format!("{}/proteome/proteome_db", own(output))), _ => None,
        };
        let profile_input_tsv = match &args.command {
            Some(Profile { input_tsv, .. }) => Some(own(input_tsv)),
            Some(EasyCore { output, .. }) => Some(format!("{}/cluster/clust.tsv", own(output))),
            Some(EasySearch { output, .. }) => Some(format!("{}/search/search.m8", own(output))), _ => None,
        };
        let profile_output = match &args.command {
            Some(Profile { output, .. }) => Some(own(output)),
            Some(EasyCore { output, .. }) => Some(format!("{}/profile", own(output))),
            Some(EasySearch { output, .. }) => Some(format!("{}/profile", own(output))), _ => None,
        };
        let profile_threshold = match &args.command {
            Some(Profile { threshold, .. }) => Some(*threshold),
            Some(EasyCore { core_threshold, .. }) => Some(*core_threshold),
            Some(EasySearch { core_threshold, .. }) => Some(*core_threshold), _ => None,
        };
        let profile_print_copiness = match &args.command {
            Some(Profile { print_copiness, .. }) => Some(*print_copiness),
            Some(EasyCore { print_copiness, .. }) => Some(*print_copiness),
            Some(EasySearch { print_copiness, .. }) => Some(*print_copiness), _ => None,
        };

        let search_input = match &args.command {
            Some(Search { input, .. }) => Some(own(input)),
            Some(EasySearch { output, .. }) => Some(format!("{}/proteome/proteome_db", own(output))), _ => None,
        };
        let search_target = match &args.command {
            Some(Search { target, .. }) => Some(own(target)),
            Some(EasySearch { target, .. }) => Some(own(target)), _ => None,
        };
        let search_output = match &args.command {
            Some(Search { output, .. }) => Some(own(output)),
            Some(EasySearch { output, .. }) => Some(format!("{}/search/search", own(output))), _ => None,
        };
        let search_tmp = match &args.command {
            Some(Search { tmp, .. }) => Some(own(tmp)),
            Some(EasySearch { tmp, .. }) => Some(own(tmp)), _ => None,
        };
        let search_keep_aln_db = match &args.command {
            Some(Search { keep_aln_db, .. }) => Some(*keep_aln_db),
            Some(EasySearch { keep, .. }) => Some(*keep), _ => None,
        };
        let search_search_options = match &args.command {
            Some(Search { search_options, .. }) => Some(search_options.clone()),
            Some(EasySearch { search_options, .. }) => Some(search_options.clone()), _ => None,
        };

        let cluster_input = match &args.command {
            Some(Cluster { input, .. }) => Some(own(input)),
            Some(EasyCore { output, .. }) => Some(format!("{}/proteome/proteome_db", own(output))), _ => None,
        };
        let cluster_output = match &args.command {
            Some(Cluster { output, .. }) => Some(own(output)),
            Some(EasyCore { output, .. }) => Some(format!("{}/cluster/clust", own(output))), _ => None,
        };
        let cluster_tmp = match &args.command {
            Some(Cluster { tmp, .. }) => Some(own(tmp)),
            Some(EasyCore { tmp, .. }) => Some(own(tmp)), _ => None,
        };
        let cluster_keep_cluster_db = match &args.command {
            Some(Cluster { keep_cluster_db, .. }) => Some(*keep_cluster_db),
            Some(EasyCore { keep, .. }) => Some(*keep), _ => None,
        };
        let cluster_cluster_options = match &args.command {
            Some(Cluster { cluster_options, .. }) => Some(cluster_options.clone()),
            Some(EasyCore { cluster_options, .. }) => Some(cluster_options.clone()), _ => None,
        };

        let tree_db = match &args.command {
            Some(Tree { db, .. }) => Some(own(db)),
            Some(EasyCore { output, .. }) => Some(format!("{}/proteome/proteome_db", own(output))),
            Some(EasySearch { output, .. }) => Some(format!("{}/proteome/proteome_db", own(output))), _ => None,
        };
        let tree_input = match &args.command {
            Some(Tree { input, .. }) => Some(own(input)),
            Some(EasyCore { output, .. }) => Some(format!("{}/profile", own(output))),
            Some(EasySearch { output, .. }) => Some(format!("{}/profile", own(output))), _ => None,
        };
        let tree_output = match &args.command {
            Some(Tree { output, .. }) => Some(own(output)),
            Some(EasyCore { output, .. }) => Some(format!("{}/tree", own(output))),
            Some(EasySearch { output, .. }) => Some(format!("{}/tree", own(output))), _ => None,
        };
        let tree_aligner = match &args.command {
            Some(Tree { aligner, .. }) => Some(aligner.clone()),
            Some(EasyCore { aligner, .. }) => Some(aligner.clone()),
            Some(EasySearch { aligner, .. }) => Some(aligner.clone()), _ => None,
        };
        let tree_tree_builder = match &args.command {
            Some(Tree { tree_builder, .. }) => Some(tree_builder.clone()),
            Some(EasyCore { tree_builder, .. }) => Some(tree_builder.clone()),
            Some(EasySearch { tree_builder, .. }) => Some(tree_builder.clone()), _ => None,
        };
        let tree_aligner_options = match &args.command {
            Some(Tree { aligner_options, .. }) => Some(aligner_options.clone()),
            Some(EasyCore { aligner_options, .. }) => Some(aligner_options.clone()),
            Some(EasySearch { aligner_options, .. }) => Some(aligner_options.clone()), _ => None,
        };
        let tree_tree_options = match &args.command {
            Some(Tree { tree_options, .. }) => Some(tree_options.clone()),
            Some(EasyCore { tree_options, .. }) => Some(tree_options.clone()),
            Some(EasySearch { tree_options, .. }) => Some(tree_options.clone()), _ => None,
        };
        let tree_threshold = match &args.command {
            Some(Tree { threshold, .. }) => Some(*threshold),
            Some(EasyCore { gap_threshold, .. }) => Some(*gap_threshold),
            Some(EasySearch { gap_threshold, .. }) => Some(*gap_threshold), _ => None,
        };

        let genetree_input = match &args.command {
            Some(GeneTree { input, .. }) => Some(own(input)), _ => None,
        };
        let genetree_names = match &args.command {
            Some(GeneTree { names, .. }) => Some(names.clone()), _ => None,
        };
        let genetree_tree_builder = match &args.command {
            Some(GeneTree { tree_builder, .. }) => Some(tree_builder.clone()), _ => None,
        };
        let genetree_tree_options = match &args.command {
            Some(GeneTree { tree_options, .. }) => Some(tree_options.clone()), _ => None,
        };
        let genetree_realign = match &args.command {
            Some(GeneTree { realign, .. }) => Some(*realign), _ => None,
        };
        let genetree_aligner = match &args.command {
            Some(GeneTree { aligner, .. }) => Some(aligner.clone()), _ => None,
        };
        let genetree_aligner_options = match &args.command {
            Some(GeneTree { aligner_options, .. }) => Some(aligner_options.clone()), _ => None,
        };
        let genetree_threshold = match &args.command {
            Some(GeneTree { threshold, .. }) => Some(*threshold), _ => None,
        };

        let config_check = match &args.command {
            Some(Config { check, .. }) => Some(*check), _ => None,
        };
        let config_set_mmseqs = match &args.command {
            Some(Config { set_mmseqs, .. }) => match set_mmseqs { Some(p) => Some(own(p)), _ => None }, _ => None,
        };
        let config_set_foldseek = match &args.command {
            Some(Config { set_foldseek, .. }) => match set_foldseek { Some(p) => Some(own(p)), _ => None }, _ => None,
        };
        let config_set_foldmason = match &args.command {
            Some(Config { set_foldmason, .. }) => match set_foldmason { Some(p) => Some(own(p)), _ => None }, _ => None,
        };
        let config_set_mafft = match &args.command {
            Some(Config { set_mafft, .. }) => match set_mafft { Some(p) => Some(own(p)), _ => None }, _ => None,
        };
        let config_set_mafft_linsi = match &args.command {
            Some(Config { set_mafft_linsi, .. }) => match set_mafft_linsi { Some(p) => Some(own(p)), _ => None }, _ => None,
        };
        let config_set_iqtree = match &args.command {
            Some(Config { set_iqtree, .. }) => match set_iqtree { Some(p) => Some(own(p)), _ => None }, _ => None,
        };
        let config_set_fasttree = match &args.command {
            Some(Config { set_fasttree, .. }) => match set_fasttree { Some(p) => Some(own(p)), _ => None }, _ => None,
        };
        let config_set_raxml = match &args.command {
            Some(Config { set_raxml, .. }) => match set_raxml { Some(p) => Some(own(p)), _ => None }, _ => None,
        };

        Args {
            command: args.command, version: args.version, threads, verbosity,
            createdb_input, createdb_output, createdb_model, createdb_keep, createdb_overwrite, createdb_max_len, createdb_gpu, createdb_afdb_lookup, createdb_custom_lookup,
            profile_input_db, profile_input_tsv, profile_output, profile_threshold, profile_print_copiness,
            search_input, search_target, search_output, search_tmp, search_keep_aln_db, search_search_options,
            cluster_input, cluster_output, cluster_tmp, cluster_keep_cluster_db, cluster_cluster_options,
            tree_db, tree_input, tree_output, tree_aligner, tree_tree_builder, tree_aligner_options, tree_tree_options, tree_threshold,
            genetree_input, genetree_names, genetree_tree_builder, genetree_tree_options, genetree_realign, genetree_aligner, genetree_aligner_options, genetree_threshold,
            config_check, config_set_mmseqs, config_set_foldseek, config_set_foldmason, config_set_mafft, config_set_mafft_linsi, config_set_iqtree, config_set_fasttree, config_set_raxml,
        }
    }
}