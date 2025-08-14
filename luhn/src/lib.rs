fn get_code_without_spaces(code: &str) -> String {
    code.chars().filter(|c| !c.is_whitespace()).collect()
}

fn is_numeric_code_and_long_enough(code: &str) -> bool {
    code.len() >= 2 && code.chars().all(|char| char.is_ascii_digit())
}

fn get_numeric_to_add(char: char, index: usize) -> u32 {
    let mut numeric = char.to_digit(10).unwrap_or(0);
    if index % 2 == 0 {
        return numeric;
    }
    numeric *= 2;
    if numeric > 9 {
        numeric -= 9;
    }
    numeric
}

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = get_code_without_spaces(code);
    if !is_numeric_code_and_long_enough(&code) {
        return false;
    }
    let sum = code
        .chars()
        .filter(|c| !c.is_whitespace())
        .rev()
        .enumerate()
        .fold(0, |sum, (idx, char)| sum + get_numeric_to_add(char, idx));

    sum % 10 == 0
}
