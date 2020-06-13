use regex::Regex;

// 1) Using regex to replace brackets
pub fn brackets_are_balanced(string: &str) -> bool {
    let re1 = Regex::new(r"[^\(\)\[\]\{\}]").unwrap();
    let mut result = String::from(re1.replace_all(string, ""));
    let re2 = Regex::new(r"\(\)|\{\}|\[\]").unwrap();
    while re2.is_match(&result) {
        result = String::from(re2.replace_all(&result, ""));
    }

    result.is_empty()
}

// 2) Using a vector to hold closing brackets
pub fn _brackets_are_balanced(string: &str) -> bool {
    let mut brackets = Vec::new();

    for ch in string.chars() {
        match ch {
            '{' => brackets.push('}'),
            '[' => brackets.push(']'),
            '(' => brackets.push(')'),
            '}' | ']' | ')' if brackets.pop() != Some(ch) => return false,
            _ => (),
        }
    }

    brackets.is_empty()
}
