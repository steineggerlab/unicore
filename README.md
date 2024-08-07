# Unicore
Unicore is a method for scalable and accurate phylogenetic reconstruction with structural core genes, universally applicable to any given set of taxa.

## Installation
### Quick Start with Conda
We will provide a Conda package soon. Please wait for the release.

### Manual Installation
#### Minimum requirements
* [Cargo](https://www.rust-lang.org/tools/install) (Rust)
* [Foldseek](https://foldseek.com)
* [ProstT5 model weight](https://huggingface.co/Rostlab/ProstT5)
* [Foldmason](https://foldmason.foldseek.com)
* [IQ-TREE](http://www.iqtree.org/)

#### Optional requirements
* AFDB/Swiss-Prot database
  * For now, please use `foldseek databases` to download
* [MAFFT](https://mafft.cbrc.jp/alignment/software/)
* [Fasttree](http://www.microbesonline.org/fasttree/)

#### Guide
> Note: We will provide an easier way to download these as a `download` module soon.
> Until then, please follow the steps below.

Please install the latest version of Rust from [here](https://www.rust-lang.org/tools/install).

To run `createdb` module, you have to pre-download the model weight of the ProstT5.
Please download the model from [here](https://huggingface.co/Rostlab/ProstT5).

Foldseek can be installed from [here](https://foldseek.com).

Default tool for MSA is Foldmason and for phylogeny inference is IQ-TREE.
You can download Foldmason from [here](https://foldmason.foldseek.com) and IQ-TREE from [here](http://www.iqtree.org/).

With these tools installed, you can install and run `unicore` by:
```
git clone https://github.com/steineggerlab/unicore.git
cd unicore
cargo build --release
target/release/unicore help
```

## Modules
Unicore has four main modules:
* `createdb` - Create 3Di structural alphabet database from input species
* `search` - Search structures against given reference database
* `profile` - Taxonomic profiling and core gene identification
* `tree` - Phylogenetic inference using structural core genes

Run each module with `unicore <module> help` to see the detailed usage.

### createdb
`createdb` module takes a folder with input species and outputs 3Di structural alphabets predicted with ProstT5.
This module runs much faster with GPU. Please install `cuda` for GPU acceleration.

> Note. Currently, proteomes in `.fasta` format are only supported as an input. Eventually, we will support more types and formats that can represent species' protein space.

Example dataset:
```
data/
└─┬ Proteome1.fasta
  ├ Proteome2.fasta
  ...
  └ ProteomeN.fasta

```
To run the module, please use the following command:
```
unicore createdb data db/proteome_db /path/to/prostt5/model
```
This will create a Foldseek database in the `db` folder.

### search
`search` module takes a `createdb` output database, searches them against the given reference database, and outputs the alignment results in BLAST format.
On default, alignments with 80% bidirectional coverage with E-value < $10^{-3}$ will be reported.

Example command:
```
unicore search db/proteome_db /path/to/reference/db search/result tmp
```
This will create a `result.m8` output file in the `search` folder.

### profile
`profile` module takes the database (`createdb` output) and alignment results (`search` output) to find structural core genes.

On default, the module will report the genes that are present in 80% of the species as a single copy. You can change this threshold by `-t` option.

Example command:
```
// 85% coverage
unicore profile -t 85 db/proteome_db search/result.m8 result
```
This will create a `result` folder with the core genes and their occurrence in the species.

### tree
`tree` module takes the core genes and the species proteomes to infer the phylogenetic tree using the alignments of the structural core genes.

On default, alignment will be generated by `foldmason` and truncated by 50% gap filtering, followed by phylogenetic inference using `iqtree`.

Example command:
```
unicore tree db/proteome_db result tree
```

This will create a `tree` folder with the phylogenetic tree in Newick format.