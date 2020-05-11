use std::collections::HashMap;

const NUCLEOTIDES: &str = "ACTG";

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if NUCLEOTIDES.contains(nucleotide) {
        Ok(nucleotide_counts(dna)?[&nucleotide])
    } else {
        Err(nucleotide)
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts: HashMap<char, usize> = NUCLEOTIDES.chars().map(|c| (c, 0)).collect();
    for c in dna.chars() {
        match c {
            'A' | 'C' | 'T' | 'G' => *counts.entry(c).or_insert(0) += 1,
            _ => return Err(c),
        }
    }
    Ok(counts)
}
