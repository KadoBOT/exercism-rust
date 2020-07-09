pub fn rotate(input: &str, key: i8) -> String {
    let mut alphabet = (b'a'..=b'z').map(|b| b as char).collect::<Vec<_>>();
    if key > 0 {
        alphabet.rotate_left(key as usize);
    } else {
        alphabet.rotate_right(key.abs() as usize);
    }
    input
        .bytes()
        .map(|b: u8| match b {
            b'a'..=b'z' => alphabet[(b - b'a') as usize],
            b'A'..=b'Z' => alphabet[(b - b'A') as usize].to_ascii_uppercase(),
            _ => b as char,
        })
        .collect()
}
