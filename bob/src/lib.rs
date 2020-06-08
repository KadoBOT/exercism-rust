pub fn reply(message: &str) -> &str {
    let message = message.trim();
    let has_letter = message.chars().any(char::is_alphabetic);
    let is_yell = has_letter && message.to_uppercase() == message;

    match message {
        _ if message.is_empty() => "Fine. Be that way!",
        _ if is_yell && message.ends_with('?') => "Calm down, I know what I'm doing!",
        _ if message.ends_with('?') => "Sure.",
        _ if is_yell => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
