use rand::{thread_rng, Rng};

fn is_invalid(s: &str) -> bool {
    s.is_empty() || s.chars().any(|ch| ch.is_uppercase() || !ch.is_alphabetic())
}

pub fn rotate(key: &str, s: &str, f: fn(&mut Vec<char>, usize) -> ()) -> String {
    key.bytes()
        .cycle()
        .zip(s.bytes())
        .map(|(b, s_byte)| {
            let distance = b - b'a';
            let mut alphabet = (b'a'..=b'z').map(|b| b as char).collect::<Vec<_>>();
            f(&mut alphabet, distance as usize);
            let s_distance = s_byte - b'a';
            *alphabet.get(s_distance as usize).unwrap()
        })
        .collect::<String>()
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    if is_invalid(key) || is_invalid(s) {
        return None;
    }

    let result = rotate(key, s, |chs, num| chs.rotate_left(num));
    Some(result)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if is_invalid(key) || is_invalid(s) {
        return None;
    }

    let result = rotate(key, s, |chs, num| chs.rotate_right(num));
    Some(result)
}

pub fn encode_random(s: &str) -> (String, String) {
    let key = thread_rng()
        .gen_iter()
        .take(100)
        .map(|num: u32| ((num % 26) as u8 + b'a') as u8 as char)
        .collect::<String>();
    let cipher = encode(&key, s).unwrap();
    (key, cipher)
}
