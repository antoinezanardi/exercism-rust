fn is_uppercase(message: &str) -> bool {
    let mut has_letter = false;
    for c in message.chars() {
        if c.is_ascii_alphabetic() {
            has_letter = true;
            if c.is_ascii_lowercase() {
                return false;
            }
        }
    }

    has_letter
}

fn is_question(message: &str) -> bool {
    message.ends_with('?')
}

pub fn reply(message: &str) -> &str {
    let trimmed = message.trim();

    match trimmed {
        _ if trimmed.is_empty() => "Fine. Be that way!",
        _ if is_question(trimmed) && is_uppercase(trimmed) => "Calm down, I know what I'm doing!",
        _ if is_question(trimmed) => "Sure.",
        _ if is_uppercase(trimmed) => "Whoa, chill out!",
        _ => "Whatever."
    }
}
