pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        let str = self.to_string();
        str.chars()
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
        .map_or(false, |sum| str.trim().len() > 1 && sum % 10 == 0)
    }
}
