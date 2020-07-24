use regex::Regex;

pub fn number(user_number: &str) -> Option<String> {
    let user_number = user_number.trim();
    let re = Regex::new(r"^((\+?1)?\s?\(?([2-9]{1}\d{2})\)?(\s*|-|\.)?([2-9]{1}\d{2})(\s*|-|\.)?(\d{4}))$").unwrap();
    let caps = re.captures(user_number)?;
    Some(format!("{}{}{}", &caps[3], &caps[5], &caps[7]))
}
