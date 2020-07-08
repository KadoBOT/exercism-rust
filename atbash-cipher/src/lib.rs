use std::collections::HashMap;

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let alphabet = (b'a'..=b'z')
        .zip((b'a'..=b'z').rev())
        .map(|(a, b)| (a as char, b as char))
        .collect::<HashMap<char, char>>();
    plain
        .chars()
        .fold((String::new(), 0), |(mut acc, count), ch| {
            if ch.is_alphanumeric() {
                if count % 5 == 0 && count > 0 {
                    acc += " "
                }
                let encoded_ch = alphabet.get(&ch.to_ascii_lowercase()).unwrap_or(&ch);
                return (format!("{}{}", acc, encoded_ch), count + 1);
            }
            (acc, count)
        })
        .0
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    let alphabet = (b'a'..=b'z')
        .rev()
        .zip(b'a'..=b'z')
        .map(|(a, b)| (a as char, b as char))
        .collect::<HashMap<char, char>>();
    cipher
        .chars()
        .filter_map(|ch| {
            if ch.is_whitespace() {
                return None;
            }
            Some(*alphabet.get(&ch.to_ascii_lowercase()).unwrap_or(&ch))
        })
        .collect()
}
