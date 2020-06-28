pub fn check(candidate: &str) -> bool {
    let mut filtered_str = candidate
        .to_ascii_uppercase()
        .chars()
        .filter(char::is_ascii_alphabetic)
        .collect::<Vec<_>>();
    let filtered_str_len = filtered_str.len();
    filtered_str.sort();
    filtered_str.dedup();
    filtered_str.len() == filtered_str_len
}
