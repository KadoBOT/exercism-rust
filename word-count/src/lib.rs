use regex::Regex;
use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let re = Regex::new(r"(\\\w)?(\w+('\w)?)").unwrap();
    re.captures_iter(words)
        .fold(HashMap::new(), |mut acc, cap| {
            *acc.entry(cap[2].to_lowercase()).or_insert(0) += 1;
            acc
        })
}
