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
* [MAFFT](https://mafft.cbrc.jp/alignment/software/)
* [IQ-TREE](http://www.iqtree.org/)

#### Optional requirements
* AFDB/Swiss-Prot database
  * For now, please use `foldseek databases` to download
* [Foldmason](https://foldmason.foldseek.com)
* [Fasttree](http://www.microbesonline.org/fasttree/)

#### Guide
> Note: We will provide an easier way to download these as a `download` module soon.
> Until then, please follow the following steps.

Please install the latest version of Rust from [here](https://www.rust-lang.org/tools/install).

To run `createdb` module, you have to pre-download the model weight of the ProstT5.
Please download the model from [here](https://huggingface.co/Rostlab/ProstT5).

Foldseek can be installed from [here](https://foldseek.com).

Default tool for MSA is `MAFFT` and for phylogeny inference is `IQ-Tree`.
You can download `MAFFT` from [here](https://mafft.cbrc.jp/alignment/software/) and `IQ-Tree` from [here](http://www.iqtree.org/).

With these tools installed, you can install and run `unicore` by:
```
git clone https://github.com/steineggerlab/unicore.git
cd unicore
cargo build --release
target/release/unicore help
```

## Modules
Unicore has four modules:
* createdb
* search
* profile
* tree
### createdb
`createdb` takes a folder with proteomes in fasta files as input and outputs database for `foldseek` after predicting 3Di sequences, which are structural alphabets, with ProstT5. The job will be done faster if you utilie GPU.

Example dataset:
```
data
|
|___|-Species1_proteome.fasta
    |-Species2_proteome.fasta
    |-Species3_proteome.fasta
    ...
    |-Species20_proteom.fasta

```
Example command:
```
unicore createdb --max-len 3000 data db/proteome_db /to/prostt5/model
```
Example output:
```
db
|
|___|-proteome_db
    |-proteome_db.dbtype
    |-proteome_db.index
    |-proteome_db.lookup
    |-proteome_db.source
    |-proteome_db_ss
    ...
    |-proteome_db_ss.index
```
### search
The module `search` takes foldseek database, which is the output of the `createdb`, and outputs BLAST format output.\
It will on default search clustered SwissProt against the proteome database with 80% bidirectional coverage.

Example command:
```
unicore search db/proteome_db /to/swissprot/database search/result tmp
```
### profile
The module `profile` takes proteome database and BLAST format output from `createdb` and `search` and outputs structural core genes.\
On default, the module reports the core genes only when at least 80% of the species have single copy protein.\
To change the coverage, please use argument `-t`.

Example command:
```
# 85% coverage
unicore profile -t 0.85 db/proteome_db search/result.m8 core_genes
```
### tree
On default, `foldmason` is used for computing MSAs, over 50% gap will be removed from the MSAs, and IQ-Tree will be used for the phylogenetic inference.

Example command:
```
# 60% gap filtering
unicore tree -t 0.6 db/proteome_db core_genes tree
```