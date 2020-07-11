use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    let mut w_vec = word.to_lowercase().chars().collect::<Vec<_>>();

    possible_anagrams
        .iter()
        .filter_map(|&a| {
            let mut a_vec = a.to_lowercase().chars().collect::<Vec<_>>();
            if a_vec == w_vec {
                return None;
            }

            w_vec.sort();
            a_vec.sort();

            if a_vec == w_vec {
                return Some(a);
            }
            None
        })
        .collect()
}
