use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter().fold(BTreeMap::new(), |mut acc, (score, letters)| {
        letters.iter().for_each(|letter| {
            acc.insert(letter.to_ascii_lowercase(), *score);
        });
        acc
    })
}
