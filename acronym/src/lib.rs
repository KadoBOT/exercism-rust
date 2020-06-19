pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(&[' ', '_', '-'][..])
        .flat_map(|w| {
            w.chars().take(1).chain(
                w.chars()
                    .skip_while(|ch| ch.is_uppercase())
                    .filter(|ch| ch.is_uppercase()),
            )
        })
        .collect::<String>()
        .to_ascii_uppercase()
}
