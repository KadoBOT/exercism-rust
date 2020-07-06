const VOWEL_SOUNDS: [&str; 7] = ["a", "e", "i", "o", "u", "xr", "yt"];
const CONSONANT_SOUNDS: [&str; 6] = ["ch", "qu", "thr", "th", "sch", "rh"];

pub fn translate(input: &str) -> String {
    input
        .split_ascii_whitespace()
        .map(|word| {
            let vowel_sound = VOWEL_SOUNDS.iter().find(|&pat| word.starts_with(pat));
            if vowel_sound.is_some() {
                return format!("{}ay", input);
            }

            let consonant_sound = CONSONANT_SOUNDS.iter().find(|&pat| word.starts_with(pat));
            if let Some(prefix) = consonant_sound {
                return format!("{}{}ay", &word[prefix.len()..], prefix);
            }

            let consonant_qu = &word[1..=2] == "qu";
            if consonant_qu {
                return format!("{}{}ay", &word[3..], &word[0..=2]);
            }

            format!("{}{}ay", &word[1..], &word[0..1])
        })
        .collect::<Vec<_>>()
        .join(" ")
}
