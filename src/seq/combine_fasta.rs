use std::fs::File;
use std::path::PathBuf;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use crate::envs::error_handler as err;

// Function that loops until a line that starts with '>' is found
fn skip_to_next_fasta_line(reader: &mut BufReader<File>, sequence: &mut String, line: &mut String, add_this_tmp: &mut usize, eof: &mut bool) -> io::Result<()> {
    loop {
        line.clear();
        match reader.read_line(line) {
            Ok(0) => {
                *eof = true;
                break
            },
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        if line.starts_with('>') {
            break;
        }
        // let line = *line.trim();
        sequence.push_str(line.trim());
        *add_this_tmp += line.trim().len();
    }
    Ok(())
}
pub fn combine_fasta(fasta_files: &Vec<String>, output_file: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut names: Vec<String> = Vec::new();
    let mut sequences: Vec<String> = Vec::new();
    let mut prev_len = 0;

    // Process each FASTA file
    for fasta_path in fasta_files {
        let file = File::open(fasta_path.trim())?;
        let mut reader = BufReader::new(file);
        let mut line = String::new();
        let mut add_this = 0;

        match reader.read_line(&mut line) {
            Ok(0) => continue,
            Ok(_) => (),
            Err(_) => err::error(err::ERR_GENERAL, Some("Problem in reading fasta file".to_string())),
        }

        let mut eof = false;
        while !eof {
            if line.starts_with('>') {
                let name = line[1..].trim().to_string();
                // If the name is already in the list, append the sequence
                if let Some(pos) = names.iter().position(|n| n == &name) {
                    // Check if the sequence[pos] has the same length to the previous sequences
                    if sequences[pos].len() != prev_len {
                        // Pad the sequence with '-' if shorter than prev_len
                        let curr_len = sequences[pos].len();
                        // print prev_len and curr_len
                        println!("prev len: {} curr len: {}, pos: {}, name: {}, fasta: {}" , prev_len, curr_len, pos, names[pos], fasta_path.trim());
                        sequences[pos].push_str(&"-".repeat(prev_len - curr_len));
                    }
                    let mut sequence = String::new();
                    let mut add_this_tmp = 0;
                    skip_to_next_fasta_line(&mut reader, &mut sequence, &mut line, &mut add_this_tmp, &mut eof)?;
                    sequences[pos].push_str(&sequence);
                    add_this = add_this_tmp;
                } else {
                    names.push(name);
                    let mut sequence = "-".repeat(prev_len);
                    let mut add_this_tmp = 0;
                    skip_to_next_fasta_line(&mut reader, &mut sequence, &mut line, &mut add_this_tmp, &mut eof)?;
                    sequences.push(sequence);
                    add_this = add_this_tmp;
                }
            } else {
                line.clear();
                match reader.read_line(&mut line) {
                    Ok(0) => break,
                    Ok(_) => (),
                    Err(_) => err::error(err::ERR_GENERAL, Some("Problem in reading fasta file".to_string()))
                }
            }
        }

        prev_len += add_this;
        // Pad sequences if shorter than prev_len
        for seq in sequences.iter_mut() {
            if seq.len() < prev_len {
                seq.push_str(&"-".repeat(prev_len - seq.len()));
            }
        }
    }

    // Write to output file
    let mut output = BufWriter::new(File::create(output_file)?);
    for (name, sequence) in names.iter().zip(sequences.iter()) {
        writeln!(output, ">{}\n{}", name, sequence)?;
    }

    Ok(())
}
