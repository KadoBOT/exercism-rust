/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .rev()
        .filter(|ch| !ch.is_whitespace())
        .enumerate()
        .try_fold(0, |acc, (idx, ch)| {
            ch.to_digit(10).map(|num| match (idx + 1) % 2 {
                0 if num * 2 > 9 => acc + (num * 2 - 9),
                0 => acc + (num * 2),
                _ => acc + num,
            })
        })
        .map_or(false, |sum| code.trim().len() > 1 && sum % 10 == 0)
}
