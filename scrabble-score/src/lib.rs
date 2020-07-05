/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    word.to_ascii_uppercase()
        .chars()
        .fold(0, |acc, ch| match ch {
            'Q' | 'Z' => acc + 10,
            'J' | 'X' => acc + 8,
            'K' => acc + 5,
            'F' | 'H' | 'V' | 'W' | 'Y' => acc + 4,
            'B' | 'C' | 'M' | 'P' => acc + 3,
            'D' | 'G' => acc + 2,
            'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => acc + 1,
            _ => acc,
        })
}
