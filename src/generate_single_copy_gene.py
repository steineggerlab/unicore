import sys

# python generate_single_copy_gene.py <gene_to_spe_list> <m8_file> <output_file>

gene_to_spe_list = sys.argv[1]
m8_file = sys.argv[2]
output_file = sys.argv[3]

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

# Read in the m8 file
# and output the gene, multiple copy percent, and single copy percent

with open(m8_file, 'r') as f, open(output_file, 'w') as out:
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
                    spe_dict[spe] = 0
                spe_dict[spe] += 1
        else:
            if curr_query is not None:
                single_copy = sum([1 for spe in spe_dict if spe_dict[spe] == 1])
                single_copy_percent = single_copy / species_count
                multiple_copy = len(spe_dict)
                multiple_copy_percent = multiple_copy / species_count
                out.write(curr_query + '\t' + str(multiple_copy_percent) + '\t' + str(single_copy_percent) + '\n')
            curr_query = query
            spe_dict = {}
            species = gene_to_spe[target]
            for spe in species:
                if spe not in spe_dict:
                    spe_dict[spe] = 0
                spe_dict[spe] += 1
    single_copy = sum([1 for spe in spe_dict if spe_dict[spe] == 1])
    single_copy_percent = single_copy / species_count
    multiple_copy = len(spe_dict)
    multiple_copy_percent = multiple_copy / species_count
    out.write(curr_query + '\t' + str(multiple_copy_percent) + '\t' + str(single_copy_percent) + '\n')
