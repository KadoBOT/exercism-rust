use std::collections::HashMap;

const VALID_DNAS: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    nucleotide_counts(dna).and_then(|m| m.get(&nucleotide).copied().ok_or(nucleotide))
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut hashmap = HashMap::new();
    hashmap.insert('A', 0);
    hashmap.insert('C', 0);
    hashmap.insert('G', 0);
    hashmap.insert('T', 0);
    for ch in dna.chars() {
        if !VALID_DNAS.contains(&ch) {
            return Err(ch);
        }
        hashmap.entry(ch).and_modify(|v| *v += 1);
    }
    Ok(hashmap)
}
