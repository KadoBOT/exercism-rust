use std::collections::HashMap;
use std::str;

pub struct CodonsInfo<'a> {
    pairs: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &'a str) -> Option<&'a str> {
        self.pairs.get(codon).cloned()
    }

    pub fn of_rna(&self, rna: &'a str) -> Option<Vec<&'a str>> {
        rna.as_bytes()
            .chunks(3)
            .map(str::from_utf8)
            .map(|c| self.name_for(c.unwrap()))
            .take_while(|&c| c != Some("stop codon"))
            .collect()
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo {
        pairs: pairs.into_iter().collect(),
    }
}
