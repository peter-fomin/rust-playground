#[derive(Debug, PartialEq)]
pub struct DNA(String);

#[derive(Debug, PartialEq)]
pub struct RNA(String);

impl DNA {
    const NUCLEOTIDES: &'static str = "ACTG";

    pub fn new(sequence: &str) -> Result<Self, usize> {
        Ok(Self(check_sequence(sequence, Self::NUCLEOTIDES)?))
    }

    pub fn into_rna(self) -> RNA {
        RNA(self
            .0
            .chars()
            .map(|nuc| {
                RNA::NUCLEOTIDES
                    .chars()
                    .nth(Self::NUCLEOTIDES.find(nuc).unwrap())
                    .unwrap()
            })
            .collect())
    }
}

impl RNA {
    const NUCLEOTIDES: &'static str = "UGAC";

    pub fn new(sequence: &str) -> Result<Self, usize> {
        Ok(Self(check_sequence(sequence, Self::NUCLEOTIDES)?))
    }
}

fn check_sequence(sequence: &str, nucleotides: &str) -> Result<String, usize> {
    match sequence.chars().position(|c| !nucleotides.contains(c)) {
        Some(position) => Err(position),
        None => Ok(sequence.to_string()),
    }
}
