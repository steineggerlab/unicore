import os
import sys

# python create_gene_specific_fasta.py <input_aa_fasta> <input_3di_fasta> <gene_dir>

# seqs having [aa, 3di] for each gene
seqs = {}
# Read amino acid fasta file
with open(sys.argv[1]) as f:
    line = f.readline()
    while line:
        if line[0] == ">":
            name = line.strip()[1:]
            line = f.readline()
            if name not in seqs:
                seqs[name] = []
            seqs[name].append(line.strip())
        line = f.readline()

# Read 3di fasta file
with open(sys.argv[2]) as f:
    line = f.readline()
    while line:
        if line[0] == ">":
            name = line.strip()[1:]
            line = f.readline()
            if name not in seqs:
                Raise(f"Error: 3di fasta file contains a sequence not in the aa fasta file {name}")
            seqs[name].append(line.strip())
        line = f.readline()

# From the directory, get the list of *.txt files
gene_dir = sys.argv[3]
gene_list = [f for f in os.listdir(gene_dir) if f.endswith('.txt')]

cnt = 0
for gene in gene_list:
    gene_name = gene[:-4]
    # If there is no directory, create one
    if not os.path.exists(os.path.join(gene_dir, gene_name)):
        os.makedirs(os.path.join(gene_dir, gene_name))
    with open(os.path.join(gene_dir, gene)) as f, open(os.path.join(gene_dir, gene_name, "aa.fasta"), 'w') as aa_f, open(os.path.join(gene_dir, gene_name, "3di.fasta"), 'w') as di_f:
        for line in f:
            line = line.strip().split()
            aa_f.write(f">{line[1]}\n{seqs[line[0]][0]}\n")
            di_f.write(f">{line[1]}\n{seqs[line[0]][1]}\n")
    cnt += 1
    print(f"\rCreated gene specific fasta files for {gene_dir}/{gene_name}, {cnt}/{len(gene_list)}", end="")
print(f"{gene_dir} done!")

