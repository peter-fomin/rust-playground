#[derive(Debug, PartialEq)]
pub struct DNA;

#[derive(Debug, PartialEq)]
pub struct RNA;

#[derive(Debug, PartialEq)]
pub struct NucleicAcid {
    sequence: Vec<char>,
    acid: Acid,
}

#[derive(Debug, PartialEq)]
pub enum Acid {
    DNA,
    RNA,
}

impl DNA {
    pub fn new(sequence: &str) -> Result<NucleicAcid, usize> {
        NucleicAcid::new(sequence, Acid::DNA)
    }
}

impl RNA {
    pub fn new(sequence: &str) -> Result<NucleicAcid, usize> {
        NucleicAcid::new(sequence, Acid::RNA)
    }
}

impl Acid {
    fn nucleotides(&self) -> &'static [char] {
        // arrays have to be in transcription order
        match self {
            Self::DNA => &['A', 'C', 'T', 'G'],
            // transcribes: v    v    v    v
            Self::RNA => &['U', 'G', 'A', 'C'],
        }
    }
}

impl NucleicAcid {
    pub fn new(sequence: &str, acid: Acid) -> Result<Self, usize> {
        match sequence
            .chars()
            .position(|c| !acid.nucleotides().contains(&c))
        {
            Some(position) => Err(position),
            None => Ok(Self {
                sequence: sequence.chars().collect(),
                acid,
            }),
        }
    }

    pub fn into_rna(mut self) -> Self {
        for nucleotide in self.sequence.iter_mut() {
            Self::nucleotide_to_rna(nucleotide);
        }
        self.acid = Acid::RNA;
        self
    }

    fn nucleotide_to_rna(nuc: &mut char) {
        *nuc = Acid::RNA.nucleotides()[Acid::DNA
            .nucleotides()
            .iter()
            .position(|c| c == nuc)
            .unwrap()];
    }
}
