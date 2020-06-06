pub fn factors(n: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let mut num = 2;
    let mut cur = n;
    while cur > 1 {
        match cur % num {
            0 => {
                result.push(num);
                cur /= num;
            }
            _ => num += 1,
        }
    }

    result
}
