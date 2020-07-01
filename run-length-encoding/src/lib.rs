use std::str;

pub fn encode(source: &str) -> String {
    source
        .chars()
        .fold(vec![], |mut acc, ch| {
            match acc.pop() {
                Some((n, c)) if c == ch => {
                    acc.push((n + 1, c));
                }
                Some((n, c)) => {
                    acc.push((n, c));
                    acc.push((1, ch));
                }
                None => acc.push((1, ch)),
            };
            acc
        })
        .iter()
        .map(|(count, b)| match count {
            1 => format!("{}", b),
            _ => format!("{}{}", count, b),
        })
        .collect()
}

pub fn decode(source: &str) -> String {
    source
        .split(|ch: char| !ch.is_digit(10))
        .map(|n_str| n_str.parse::<usize>().unwrap_or(1))
        .zip(source.matches(|ch: char| !ch.is_digit(10)))
        .map(|(num, c)| c.repeat(num))
        .collect()
}
