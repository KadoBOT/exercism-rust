pub fn is_armstrong_number(num: u32) -> bool {
    let str_num = num.to_string();
    let sum: u32 = (&str_num)
        .chars()
        .map(|ch| ch.to_digit(10).unwrap().pow(str_num.len() as u32))
        .sum();
    sum.eq(&num)
}
