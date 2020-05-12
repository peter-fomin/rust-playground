#[derive(Debug, PartialEq)]
pub struct DNA {
    dna: String,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    rna: String,
}

impl DNA {
    const NUCLEOTIDES: &'static str = "ACTG";

    pub fn new(dna: &str) -> Result<Self, usize> {
        match dna.chars().position(|c| !Self::NUCLEOTIDES.contains(c)) {
            Some(position) => Err(position),
            None => Ok(Self {
                dna: dna.to_string(),
            }),
        }
    }

    pub fn into_rna(self) -> RNA {
        RNA {
            rna: self
                .dna
                .chars()
                .map(|c| match c {
                    'G' => 'C',
                    'C' => 'G',
                    'T' => 'A',
                    'A' => 'U',
                    _ => '?',
                })
                .collect(),
        }
    }
}

impl RNA {
    const NUCLEOTIDES: &'static str = "UGAC";
    pub fn new(rna: &str) -> Result<Self, usize> {
        match rna.chars().position(|c| !Self::NUCLEOTIDES.contains(c)) {
            Some(position) => Err(position),
            None => Ok(Self {
                rna: rna.to_string(),
            }),
        }
    }
}
