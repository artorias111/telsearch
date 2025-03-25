// Find the telomeric reads in a lsit of sequences, most probably a fastq file, figure out gz later
// inputs:
// path to fastq (test/test.fastq)
// repeat sequence (TTAGGG)

use std::collections::HashMap;
use std::str;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {

    let fastq_file = "../test_data/test_fastq.fastq";
    let test_sequences = fastq2hashmap(fastq_file);

    // println!("{test_sequences:?}");

    for sequence in test_sequences.keys() {
        let t = telomere_number(&test_sequences[sequence]);
        println!("{sequence} has {t} telomeric instances");
    }
}


fn fastq2hashmap (fastq_file: &str) -> HashMap<String, String> {
    let file = File::open(fastq_file).unwrap();
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let mut fastq_map = HashMap::new();

    while let (Some(header), Some(seq), Some(_plus), Some(_qscore)) = (
            lines.next(),
            lines.next(),
            lines.next(),
            lines.next(),) {
        let header_line = header.expect("header missing in the file");
        let sequence = seq.expect("Sequence missing in the file");
        fastq_map.insert(header_line, sequence);
    }
    fastq_map
}


fn telomere_number (s: &str) -> i32 {
    let mut telomeric_match = 0;
    let mut revcomp_telomeric_match = 0;
    for window in s.as_bytes().windows(6) {
        let window_str = str::from_utf8(window).unwrap();
        // println!("{window_str}");

        if window_str == "TTAGGG" {
            telomeric_match +=1;
        } else if window_str == revcomp("TTAGGG") {
            revcomp_telomeric_match +=1;
        }
    }
    let net_telomeric_match = telomeric_match + revcomp_telomeric_match;
    net_telomeric_match
}


fn revcomp (s: &str) -> String { //reverse complement a nucleotide
    s.chars()
        .rev()
        .map(|nucleotide| complement(nucleotide))
        .collect()
}

fn complement(nuc: char) -> char {
    match nuc {
        'A' => 'T',
        'T' => 'A',
        'G' => 'C',
        'C' => 'G',
        'a' => 't',
        't' => 'a',
        'g' => 'c',
        'c' => 'g',
        _   => 'N',  // N for any unknown or ambiguous nucleotide
        }
}
