// After
pub fn square(s: u32) -> u64 {
    match s {
        1..=64 => 1u64 << (s - 1),
        _ => panic!("Square must be between 1 and 64"),
    }
}

pub fn total() -> u64 {
    u64::max_value()
}

// Before
fn get_square_values(s: u32) -> Vec<u64> {
    let mut squares: Vec<u64> = vec![1];
    (1..s).enumerate().for_each(|(idx, _)| {
        let prev = squares[idx];
        squares.push(prev * 2);
    });

    squares
}

pub fn _square(s: u32) -> u64 {
    if s == 0 || s > 64 {
        panic!("Square must be between 1 and 64")
    }

    let squares = get_square_values(s);
    *squares.last().unwrap()
}

pub fn _total() -> u64 {
    let squares = get_square_values(64);
    squares.iter().sum()
}
