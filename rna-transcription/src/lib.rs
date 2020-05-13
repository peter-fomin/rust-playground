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
        // Arrays have to be in transcription order.
        match self {
            Self::DNA => &['A', 'C', 'T', 'G'],
            // Transcribes: v    v    v    v
            Self::RNA => &['U', 'G', 'A', 'C'],
        }
    }

    fn nucleotide_to_rna(nuc: &mut char) {
        // Transcribes one nucleotide according to the order in nucleotides() fn.
        *nuc = Acid::RNA.nucleotides()[Acid::DNA
            .nucleotides()
            .iter()
            .position(|c| c == nuc)
            .unwrap()];
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
        self.sequence.iter_mut().for_each(Acid::nucleotide_to_rna);
        self.acid = Acid::RNA;
        self
    }
}
