#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }
    if let Some(&n) = number.iter().find(|&&n| n >= from_base) {
        return Err(Error::InvalidDigit(n));
    }
    let mut decimal = number
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (idx, n)| acc + n * from_base.pow(idx as u32));

    if decimal == 0 {
        return Ok(vec![0]);
    }
    let mut result = vec![];
    while decimal > 0 {
        result.push(decimal % to_base);
        decimal /= to_base;
    }
    result.reverse();
    Ok(result)
}
