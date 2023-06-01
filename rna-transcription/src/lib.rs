#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        for (i, c) in dna.chars().enumerate() {
            if "AGCT".contains(c) == false {
                return Err(i);
            }
        }
        Ok(Dna(dna.to_string()))
    }

    pub fn into_rna(self) -> Rna {
        Rna::new(self.0.chars().map(|c| match c {
            'G' => 'C',
            'C' => 'G',
            'T' => 'A',
            'A' => 'U',
            _ => ' ',
        }).collect::<String>().as_str()).unwrap()
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        for (i, c) in rna.chars().enumerate() {
            if "AGCU".contains(c) == false {
                return Err(i);
            }
        }
        Ok(Rna(rna.to_string()))
    }
}
