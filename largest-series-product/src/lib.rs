#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

fn get_product(string_digits: &str, span: usize) -> Result<u64, Error> {
    string_digits
        .as_bytes()
        .windows(span)
        .try_fold(0, |prd_max: u64, bytes| {
            let prod = bytes.iter().try_fold(1, |acc, &b| {
                (b as char)
                    .to_digit(10)
                    .map_or(Err(Error::InvalidDigit(b as char)), |x| Ok(acc * x as u64))
            });

            match prod {
                Ok(n) if n > prd_max => prod,
                Ok(_) => Ok(prd_max),
                _ => prod,
            }
        })
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    match span {
        0 => Ok(1),
        x if x > string_digits.len() => Err(Error::SpanTooLong),
        _ => get_product(string_digits, span),
    }
}
