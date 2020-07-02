/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn = isbn.replace('-', "");
    let result = isbn.chars().fold((0, 10, true), |acc, ch| match ch {
        _ if acc.1 == 0 => (acc.0, acc.1, false),
        _ if ch.is_numeric() => (ch.to_digit(10).unwrap() * acc.1 + acc.0, acc.1 - 1, acc.2),
        'X' if acc.1 == 1 => (10 * acc.1 + acc.0, acc.1 - 1, acc.2),
        _ => (acc.0, acc.1, false),
    });

    result.0 % 11 == 0 && result.1 == 0 && result.2
}
