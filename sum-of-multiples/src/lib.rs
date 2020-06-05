pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut iter = factors.iter();
    (1..limit)
        .filter(|num| iter.any(|&x| x != 0 && num % x == 0))
        .sum()
}
