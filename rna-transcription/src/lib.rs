trait Strand {
    fn unwrap(&self) -> Option<usize> {
        let valid_chars = self
            .strand()
            .chars()
            .take_while(|ch| self.nucleotides().contains(&ch))
            .collect::<Vec<char>>();
        if valid_chars.len() == self.strand().len() {
            return None;
        }
        Some(valid_chars.len())
    }

    fn nucleotides(&self) -> [char; 4];
    fn strand(&self) -> String;
}

macro_rules! impl_strand(
    ($t:ty) => (
        impl Strand for $t {
            fn nucleotides(&self) -> [char; 4] {
                self.0
            }
            fn strand(&self) -> String {
                self.1.to_owned()
            }
        }
    );
);

#[derive(Debug, PartialEq)]
pub struct DNA([char; 4], String);

#[derive(Debug, PartialEq)]
pub struct RNA([char; 4], String);

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        let p = DNA(['A', 'C', 'G', 'T'], dna.to_string());
        if let Some(x) = p.unwrap() {
            return Err(x);
        }
        Ok(p)
    }

    pub fn into_rna(self) -> RNA {
        let rna = self
            .1
            .chars()
            .map(|ch| match ch {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => unreachable!("DNA should contain only GCTA!"),
            })
            .collect::<String>();
        RNA::new(&rna).unwrap()
    }
}

impl_strand!(DNA);

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        let p = RNA(['C', 'G', 'A', 'U'], rna.to_string());
        if let Some(x) = p.unwrap() {
            return Err(x);
        }
        Ok(p)
    }
}

impl_strand!(RNA);
