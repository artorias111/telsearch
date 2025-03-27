Telsearch works on raw sequencing reads of any kind (Illumina, PacBio HiFi, ONT, etc.) 

Returns the frequency of a canonical telomere seqeunce in each read. 

Example run: (Ensure the `fastq` file is `gz` compressed)
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
