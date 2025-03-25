// Find the telomeric reads in a lsit of sequences, most probably a fastq file, figure out gz later
// inputs:
// path to fastq (test/test.fastq)
// repeat sequence (TTAGGG)

use std::collections::HashMap;
use std::str;
use std::fs::File;
use std::io::{BufReader, BufRead};
use clap::Parser;

#[derive(Parser)]
struct Args {
    // Path to the fastq file
    #[arg(short = 'r', long = "reads")]
    reads: String, // consider changing this to std::path::PathBuf,

    // telomere sequence to search (default: TTAGGG)
    #[arg(short = 't', long = "telomere", default_value = "TTAGGG")]
    telomere: String,
}


fn main() {
    let args = Args::parse();
    // println!("test path: {:?}, test telseq {:?}", args.reads, args.telomere);

    // command line args/user input args

    // let _fastq_file = "../test_data/test_fastq.fastq";
    let fastq_file = &args.reads;
    // let _telseq = "TAGAG"; 
    let telseq = &args.telomere;

    // ----------------------------


    let sequences = fastq2hashmap(fastq_file);
    // println!("{test_sequences:?}");
    for sequence in sequences.keys() {
        let t = telomere_number(&sequences[sequence], &telseq);
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


fn telomere_number (s: &str, telseq: &str) -> i32 {
    let mut telomeric_match = 0;
    let mut revcomp_telomeric_match = 0;
    let window_size = telseq.len();
    for window in s.as_bytes().windows(window_size) { 
        let window_str = str::from_utf8(window).unwrap();
        // println!("{window_str}");

        if window_str == telseq {
            telomeric_match +=1;
        } else if window_str == revcomp(telseq) {
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
