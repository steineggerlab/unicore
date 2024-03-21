import sys
import os

# python print_single_copy_genes.py <gene_to_spe_list> <m8_file> <output_dir> <threshold>

gene_to_spe_list = sys.argv[1]
m8_file = sys.argv[2]
output_dir = sys.argv[3]
if len(sys.argv) > 4:
    threshold = float(sys.argv[4])
else:
    threshold = 0.9

gene_to_spe = {}

species_set = set()
# Read in the gene to species list
with open(gene_to_spe_list, 'r') as f:
    for line in f:
        line = line.strip()
        af_gene, spe = line.split()
        if af_gene[:3] == 'AF-':
            gene = af_gene.split('-')[1]
        else:
            gene = af_gene
        if gene not in gene_to_spe:
            gene_to_spe[gene] = set()
        gene_to_spe[gene].add(spe)
        species_set.add(spe)

species_count = len(species_set)

# If there is no output_dir, create it
if not os.path.exists(output_dir):
    os.makedirs(output_dir)

# Read in the m8 file
# and output the gene, multiple copy percent, and single copy percent

with open(m8_file, 'r') as f:
    curr_query = None
    spe_dict = {}
    for line in f:
        line = line.strip()
        query, target = line.split()[:2]
        if target[:3] == 'AF-':
            target = target.split('-')[1]
        if query == curr_query:
            species = gene_to_spe[target]
            for spe in species:
                if spe not in spe_dict:
                    spe_dict[spe] = set()
                spe_dict[spe].add(target)
        else:
            if curr_query is not None:
                single_copy = sum([1 for spe in spe_dict if len(spe_dict[spe]) == 1])
                single_copy_percent = single_copy / species_count
                if single_copy_percent >= threshold:
                    if curr_query[:3] == 'AF-':
                        curr_query = curr_query.split('-')[1]
                    with open(os.path.join(output_dir, curr_query + '.txt'), 'w') as out:
                        for spe in spe_dict:
                            if len(spe_dict[spe]) == 1:
                                spe_tar = list(spe_dict[spe])[0]
                                out.write(spe_tar + '\t' + spe + '\n')
            curr_query = query
            spe_dict = {}
            species = gene_to_spe[target]
            for spe in species:
                if spe not in spe_dict:
                    spe_dict[spe] = set()
                spe_dict[spe].add(target)
    single_copy = sum([1 for spe in spe_dict if len(spe_dict[spe]) == 1])
    single_copy_percent = single_copy / species_count
    if single_copy_percent >= threshold:
        if curr_query[:3] == 'AF-':
            curr_query = curr_query.split('-')[1]
        with open(os.path.join(output_dir, curr_query + '.txt'), 'w') as out:
            for spe in spe_dict:
                if len(spe_dict[spe]) == 1:
                    spe_tar = list(spe_dict[spe])[0]
                    out.write(spe_tar + '\t' + spe + '\n')
