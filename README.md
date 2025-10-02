Telsearch works on raw sequencing reads of any kind (Illumina, PacBio HiFi, ONT, etc.) 

This is a rookie project to practice Rust, and as of now, I've implemented it to run well on a cluster with a lot of memory (>1TB). So use it with caution. 

The default mode that currently runs without breaking uses exact string matching to detect a telomere sequence (and its reverse complement). That's a horrible way to detect biological sequences, I plan to include better pairwise alignment algorithms in the future. 

Returns the frequency of a canonical telomere seqeunce in each read. 

Example run: (The `fastq` file should be `gz` compressed)
```shell
# with Cargo
## clone this reposity
git clone https://github.com/artorias111/telsearch.git
## run with cargo
cargo run -- --reads path/to/fastq.gz --telomere TTAGGC
```

example output:  
```shell
sequence  telomere_count
SEQ004  6
SEQ009  5
SEQ002  5
.
.
.
```
