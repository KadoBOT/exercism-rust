use std::collections::HashMap;
use std::str;

pub struct CodonsInfo<'a> {
    pairs: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &'a str) -> Option<&'a str> {
        if let Some(&s) = self.pairs.get(codon) {
            return Some(s);
        }
        None
    }

    pub fn of_rna(&self, rna: &'a str) -> Option<Vec<&'a str>> {
        let subs = rna
            .as_bytes()
            .chunks(3)
            .map(str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap();

        let translations = subs
            .iter()
            .take_while(|c| self.name_for(c) != Some(&"stop codon"))
            .map(|c| self.name_for(c))
            .collect::<Vec<_>>();

        if translations.contains(&None) {
            return None;
        }

        Some(translations.iter().map(|&c| c.unwrap()).collect())
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    let pairs = pairs
        .iter()
        .map(|(a, b)| (*a, *b))
        .collect::<HashMap<_, _>>();
    CodonsInfo { pairs }
}
