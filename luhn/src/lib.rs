/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = code.trim();
    let is_valid = code.chars().all(|ch| ch.is_numeric() || ch.is_whitespace());
    if code.len() <= 1 || !is_valid {
        return false;
    }

    code.chars()
        .rev()
        .filter(|ch| ch.is_numeric())
        .enumerate()
        .fold(0, |acc, (idx, ch)| {
            let num = ch.to_digit(10).unwrap();
            match (idx + 1) % 2 {
                0 if num * 2 > 9 => acc + (num * 2 - 9),
                0 => acc + (num * 2),
                _ => acc + num,
            }
        })
        .rem_euclid(10)
        .eq(&0)
}
