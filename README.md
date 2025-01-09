# Unicore
Unicore is a method for scalable and accurate phylogenetic reconstruction with structural core genes using Foldseek and ProstT5, universally applicable to any given set of taxa.

## Publications
Kim, D., Park, S., & Steinegger, M. (2024). Unicore enables scalable and accurate phylogenetic reconstruction with structural core genes. _bioRxiv_, 2024.12.22.629535. [doi.org/10.1101/2024.12.22.629535](https://doi.org/10.1101/2024.12.22.629535)

## Table of Contents
- [Unicore](#unicore)
- [Quick Start with Conda](#quick-start-with-conda)
  - [GPU acceleration with CUDA](#gpu-acceleration-with-cuda)
  - [GPU acceleration with Foldseek-ProstT5 (beta)](#gpu-acceleration-with-foldseek-prostt5-beta)
- [Tutorial](#tutorial)
- [Manual](#manual)
  - [Input](#input)
  - [easy-core workflow](#easy-core-workflow)
  - [Modules](#modules)
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

### GPU acceleration with Foldseek-ProstT5 (beta)
> Note. This feature is under development and may not work in some environments. We will provide an update after the stable release of Foldseek-ProstT5.

Foldseek provides a GPU-compatible static binary for ProstT5 prediction (requires Linux with AVX2 support, glibc>=2.29, and nvidia-driver>=525.60.13)<br>
To use it, please install it by running the following command:
```
wget https://mmseqs.com/foldseek/foldseek-linux-gpu.tar.gz; tar xvfz foldseek-linux-gpu.tar.gz; export PATH=$(pwd)/foldseek/bin/:$PATH
```
Then, add `--use-foldseek` and `--gpu` options to either `easy-core` or `createdb` module to use Foldseek implementation of ProstT5-GPU:
```
unicore easy-core --use-foldseek --gpu <INPUT> <OUTPUT> <MODEL> <TMP>
```

<hr>

## Tutorial
### Download sample data
If you are using the conda package, you can download the example dataset from the following link:
```
wget https://unicore.steineggerlab.workers.dev/unicore_example.zip
unzip unicore_example.zip
```
If you cloned the repository, you can find the example dataset in the `example/data` folder.

### Download ProstT5 weights
You need to first download the ProstT5 weights to run the `createdb` module.
```
foldseek databases ProstT5 weights tmp
```

### Run the easy-core module
The `easy-core` module processes all the way from the input proteomes to build the phylogenetic tree based on their structural core genes.
Use the following command to run the easy-core module:
```
unicore easy-core example/data example/results weights tmp
```

If you have a CUDA-compatible GPU, add `--gpu` flag to run ProstT5 with GPU acceleration.

### Check the results
After running the `easy-core` module, you can find the results in the `example/results` folder.
 * `proteome` folder contains the proteome information parsed from the input files.
 * `cluster` folder contains the Foldseek clustering result (clust.tsv).
 * `profile` folder contains the taxonomic profiling results and metadata of defined structural core genes.
 * `tree` folder contains the results from the phylogenetic inference.

#### Phylogenetic tree
`example/results/tree/iqtree.treefile` is the concatenated structural core gene tree represented in Newick format. <br>
Each node in the tree represents a species, labeled with their input proteome file name.

#### Structural core genes
`example/results/tree/fasta` folder contains subfolders named after the defined structural core genes. <br>
Each subfolder contains the amino acid sequences (aa.fasta) and their 3Di representations (3di.fasta) of the core genes.

<hr>

## Manual
We provide an easy workflow module that automatically runs the all modules in order.
* `easy-core` - Easy core gene phylogeny workflow, from fasta files to phylogenetic tree

Unicore has four main modules, which can be run sequentially to infer the phylogenetic tree of the given species.
* `createdb` - Create 3Di structural alphabet database from input species
* `cluster` - Cluster Foldseek database
* `profile` - Taxonomic profiling and core gene identification
* `tree` - Phylogenetic inference using structural core genes

Run each module with `unicore <module> help` to see the detailed usage.

### Input
Unicore requires a set of proteomes as input to infer the phylogenetic tree. Please prepare the input proteomes in a folder.\
You can also refer to the example dataset in the `example/data` folder or download it from [here](https://unicore.steineggerlab.workers.dev/unicore_example.zip).

> Note. Currently, proteomes in `.fasta` format are only supported as an input. We will try to support more types and formats that can represent species.

Example dataset:
```
data/
└─┬ Proteome1.fasta
  ├ Proteome2.fasta
  ...
  └ ProteomeN.fasta

```

### easy-core workflow
`easy-core` workflow module orchestrates four modules in order, processes all the way from the input proteomes to the phylogenetic tree.

Example command:
```
// Download ProstT5 weights as below if you haven't already
// foldseek databases ProstT5 /path/to/prostt5/weights tmp
unicore easy-core data results /path/to/prostt5/weights tmp
```

This will create a `results/tree` folder with phylogenetic trees built with the structural core genes identified from the input proteomes.

The `easy-core` module will also create folders named `results/proteome`, `results/cluster`, and `results/profile` with intermediate results for `createdb`, `cluster`, and `profile` module, respectively.

### Modules
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

<hr>

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
