# Unicore
Unicore is a method for scalable and accurate phylogenetic reconstruction with structural core genes using Foldseek and ProstT5, universally applicable to any given set of taxa.

## Publications
Kim, D., Park, S., & Steinegger, M. (2024). Unicore enables scalable and accurate phylogenetic reconstruction with structural core genes. _bioRxiv_. [https://doi.org/10.1101/2024.12.22.629535](https://www.biorxiv.org/content/10.1101/2024.12.22.629535v1)

## Table of Contents
- [Unicore](#unicore)
- [Quick Start with Conda](#quick-start-with-conda)
  - [GPU acceleration with CUDA](#gpu-acceleration-with-cuda)
- [Manual](#manual)
  - [Preparing input](#preparing-input)
  - [Main modules](#main-modules)
  - [Additional modules](#additional-modules)
- [Build from Source](#build-from-source)
  - [Minimum requirements](#minimum-requirements)
  - [Optional requirements](#optional-requirements)
  - [Installation guide](#installation-guide)

## Quick Start with Conda
```
conda install -c bioconda unicore
unicore -v
```

### GPU acceleration with CUDA
`createdb` module can be greatly acclerated with ProstT5-GPU.
If you have a Linux machine with CUDA-compatible GPU, please install this additional package:
```
conda install -c conda-forge pytorch-gpu
```


## Manual
Unicore has four main modules, which can be run sequentially to infer the phylogenetic tree of the given species.
* `createdb` - Create 3Di structural alphabet database from input species
* `cluster` - Cluster Foldseek database
* `profile` - Taxonomic profiling and core gene identification
* `tree` - Phylogenetic inference using structural core genes

We also provide an easy workflow module that automatically runs the four modules in order.
* `easy-core` - Easy core gene phylogeny workflow, from fasta files to phylogenetic tree

Run each module with `unicore <module> help` to see the detailed usage.

### Preparing input
Unicore requires a set of proteomes as input to infer the phylogenetic tree. Please prepare the input proteomes in a folder.

> Note. Currently, proteomes in `.fasta` format are only supported as an input. We will try to support more types and formats that can represent species.

Example dataset:
```
data/
└─┬ Proteome1.fasta
  ├ Proteome2.fasta
  ...
  └ ProteomeN.fasta

```

### Main modules
#### createdb
`createdb` module takes a folder with input species and outputs 3Di structural alphabets predicted with ProstT5.

This module runs much faster with GPU. Please install `cuda` for GPU acceleration.

To run the module, please use the following command:
```
// Download ProstT5 weights as below if you haven't already
// foldseek databases ProstT5 /path/to/prostt5/weights tmp
unicore createdb data db/proteome_db /path/to/prostt5/weights
```
This will create a Foldseek database in the `db` folder.

If you have foldseek installed with CUDA, you can run the ProstT5 in the module with foldseek by adding `--use-foldseek` option.

#### cluster
`cluster` module takes a `createdb` output database, runs Foldseek clustering, and outputs the cluster results.

On default, clustering will be done with 80% bidirectional coverage (-c 0.8).<br>
You can feed custom clustering parameters for Foldseek via `--cluster-options` option.

Example command:
```
unicore cluster db/proteome_db out/clu tmp
```
This will create a `clu.tsv` output file in the `out` folder.

#### profile
`profile` module takes the database (`createdb` output) and cluster results (`cluster` output) to find structural core genes.

On default, the module will report the genes that are present in 80% of the species as a single copy. You can change this threshold by `-t` option.

Example command:
```
// 85% coverage
unicore profile -t 85 db/proteome_db out/clu.tsv result
```
This will create a `result` folder with the core genes and their occurrences in the species.

#### tree
`tree` module takes the core genes and the species proteomes to infer the phylogenetic tree using the alignments of the structural core genes.

On default, alignment will be generated by `foldmason` and truncated by 50% gap filtering, followed by phylogenetic inference using `iqtree`.

Example command:
```
unicore tree db/proteome_db result tree
```

This will create a `tree` folder with the resulting phylogenetic trees in Newick format.

#### easy-core
`easy-core` module orchestrates the four modules in order, processes all the way from the input proteomes to the phylogenetic tree.

Example command:
```
// Download ProstT5 weights as below if you haven't already
// foldseek databases ProstT5 /path/to/prostt5/weights tmp
unicore easy-core data tree /path/to/prostt5/weights tmp
```

This will create a `tree` folder with phylogenetic trees built with the structural core genes identified from the input proteomes.

### Additional modules
#### search
`search` module takes a `createdb` output database, searches them against the given reference database, and outputs the alignment results in BLAST format.

On default, alignments with 80% bidirectional coverage with E-value < $10^{-3}$ will be reported.

Example command:
```
unicore search db/proteome_db /path/to/reference/db search/result tmp
```
This will create a `result.m8` output file in the `search` folder, which can be used as an input for the `profile` module instead of the `cluster` output.

## Build from Source
### Minimum requirements
* [Cargo](https://www.rust-lang.org/tools/install) (Rust)
* [Foldseek](https://foldseek.com) (version ≥ 9)
* [Foldmason](https://foldmason.foldseek.com)
* [IQ-TREE](http://www.iqtree.org/)
* pytorch, transformers, sentencepiece, protobuf
  - These are required for users who cannot build foldseek with CUDA. Please install them with `pip install torch transformers sentencepiece protobuf`.
### Optional requirements
* [MAFFT](https://mafft.cbrc.jp/alignment/software/)
* [Fasttree](http://www.microbesonline.org/fasttree/) or [RAxML](https://cme.h-its.org/exelixis/web/software/raxml/)

### Installation guide
Please install the latest version of Rust from [here](https://www.rust-lang.org/tools/install).

Foldseek can be installed from [here](https://foldseek.com).

You have to pre-download the model weights of the ProstT5. Run `foldseek databases ProstT5 <dir> tmp` to download the weights on `<dir>`. If this doesn't work, make sure you have the latest version of Foldseek.

Foldmason and IQ-TREE is designated as default tools for alignment and phylogenetic inference. You can download Foldmason from [here](https://foldmason.foldseek.com) and IQ-TREE from [here](http://www.iqtree.org/).

With these tools installed, you can install and run `unicore` by:
```
git clone https://github.com/steineggerlab/unicore.git
cd unicore
cargo build --release
bin/unicore help
```
