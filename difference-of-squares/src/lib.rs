pub fn square_of_sum(n: u32) -> u32 {
    // (1..=n).sum::<u32>().pow(2)
    let sum: u32 = (1..=n).sum();
    return sum * sum;
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..=n).fold(0, |acc, num| num * num + acc)
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
