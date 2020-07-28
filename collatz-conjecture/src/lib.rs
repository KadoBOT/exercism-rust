pub fn collatz(n: u64) -> Option<u64> {
    match (n, n % 2) {
        (0, _) => None,
        (1, _) => Some(0),
        (_, 0) => collatz(n / 2).map(|x| x + 1),
        _ => collatz(n * 3 + 1).map(|x| x + 1),
    }
}
