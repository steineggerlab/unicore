# Unicore
Fast and accurate universal core gene phylogeny with Foldseek and ProstT5
## Requirements
* ProstT5 weight
* foldseek
* foldmason or mafft
* IQ-Tree or Fasttree
* (Optional) Clustered SwissProt (`https://~~`)

Currently you need the weight of the ProstT5 to run the `createdb`. Please download the model from `https://~~~`.

Unicore also requires `foldseek` to be installed. Please go to `https://~~~` and install `foldseek`.

Unicore uses `foldmason` and `IQ-Tree` as a default. Please specify MSA tool and phylogeny inference tool if you want to use different tools. Please refer to the `Modules` for more detail.
## Installation
If you don't have Rust, please download the language from `https://~~`.\
To download the Unicore, please follow the following:
```
git clone <current github>
cd unicore
cargo build --release
```
You can find binary from `unicore/target/release`.
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