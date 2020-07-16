use regex::Regex;

pub struct WordProblem;

pub fn answer(command: &str) -> Option<i32> {
    let r_str = r"^What is (-?\d+)( (plus|divided by|multiplied by|minus) (-?\d+)(.*))?\?$";
    let re = Regex::new(r_str).unwrap();
    let caps = re.captures(command)?;

    if caps.get(1).is_some() && caps.get(2).is_none() {
        return caps[1].parse().ok();
    }

    let (n1, n2): (i32, i32) = (caps[1].parse().unwrap(), caps[4].parse().unwrap());

    let result = match &caps[3] {
        "plus" => Some(n1 + n2),
        "divided by" => Some(n1 / n2),
        "multiplied by" => Some(n1 * n2),
        "minus" => Some(n1 - n2),
        _ => None,
    }?;

    let str = format!("What is {}{}?", result, &caps[5]);
    answer(str.as_str())
}
