import sys

# Program to combine multiple MSA fasta files into one file
# Usage: python combine_fasta.py /list/of/fasta/files.txt /output/file.fasta
fasta_files = []
with open(sys.argv[1]) as f:
    for line in f:
        fasta_files.append(line[:-1])

# Open fasta files and combine into one file
names = []
sequences = []
prev_len = 0
for fasta in fasta_files:
    add_this = 0
    with open(fasta) as f:
        line = f.readline()
        while line:
            if line[0] == ">":
                # If name is not in list, add it
                if line.strip()[1:] not in names:
                    names.append(line.strip()[1:])
                    sequences.append("".join(["-"]*prev_len))
                    line = f.readline()
                    add_this_tmp = 0
                    while line and line[0] != ">":
                        sequences[-1] += line[:-1]
                        add_this_tmp += len(line[:-1])
                        line = f.readline()
                    add_this = add_this_tmp
                else:
                    # Find index of name in list
                    index = names.index(line.strip()[1:])
                    line = f.readline()
                    # Check if sequence length is the same as previous sequences
                    if len(sequences[index]) != prev_len:
                        sequences[index] += "".join(["-"]*(prev_len-len(sequences[index])))
                    add_this_tmp = 0
                    while line and line[0] != ">":
                        sequences[index] += line[:-1]
                        add_this_tmp += len(line[:-1])
                        line = f.readline()
                    add_this = add_this_tmp
            #line = f.readline()
            else:
                line = f.readline()
    prev_len += add_this
    # Pad sequences with "-" if they are shorter than the previous sequences
    for i in range(len(sequences)):
        if len(sequences[i]) < prev_len:
            sequences[i] += "".join(["-"]*(prev_len-len(sequences[i])))
assert len(sequences) == len(names)
# Write to output file
with open(sys.argv[2], "w") as f:
    for i in range(len(names)):
        f.write(">%s\n%s\n"%(names[i], sequences[i]))
