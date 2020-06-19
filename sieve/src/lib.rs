pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut non_primes = (2..upper_bound + 1).collect::<Vec<u64>>();
    let mut result = Vec::new();

    while let Some(prime) = non_primes.to_owned().first() {
        result.push(*prime);
        non_primes.retain(|n| n % prime != 0);
    }

    result
}
