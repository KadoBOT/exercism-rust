/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    (b'A'..=b'Z').all(|b| sentence.to_ascii_uppercase().contains(char::from(b)))
}
