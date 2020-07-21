use std::fmt::Display;

pub struct Luhn(String);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        self.0
            .chars()
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
            .map_or(false, |sum| self.0.trim().len() > 1 && sum % 10 == 0)
    }
}

impl<T: Display> From<T> for Luhn {
    fn from(input: T) -> Self {
        Luhn(input.to_string())
    }
}
