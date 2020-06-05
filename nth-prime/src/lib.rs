pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = Vec::new();
    let mut count: u32 = 1;

    while primes.len() as u32 != (n + 1) {
        count += 1;
        match primes.iter().find(|&&x| count % x == 0) {
            Some(_) => continue,
            None => primes.push(count),
        }
    }

    return *primes.last().unwrap_or(&0);
}
