pub fn encrypt(input: &str) -> String {
    let input = input
        .to_lowercase()
        .chars()
        .filter(|ch| ch.is_alphanumeric())
        .collect::<String>();
    let size = (input.len() as f32).sqrt();
    let rows_vec = vec![" ".repeat(size.floor() as usize); size.ceil() as usize];
    input
        .chars()
        .fold((rows_vec, (0, 0)), |(mut res, (x, y)), ch| {
            res[y].replace_range(x..=x, &ch.to_string());
            if y == res.len() - 1 {
                return (res, (x + 1, 0));
            }
            (res, (x, y + 1))
        })
        .0
        .join(" ")
}
