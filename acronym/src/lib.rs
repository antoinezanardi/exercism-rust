fn get_acronym_letters(word: &[char]) -> String {
    word.iter()
        .enumerate()
        .fold(String::new(), |mut acc, (idx, &char)| {
            if idx == 0 || char.is_uppercase() && word[idx - 1].is_lowercase() {
                acc.push(char.to_ascii_uppercase())
            }

            acc
        })
}

fn remove_all_non_letters(word: &str) -> Vec<char> {
    word.chars().filter(|c| c.is_alphabetic()).collect()
}

pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_whitespace() || c == '-')
        .map(remove_all_non_letters)
        .map(|word| get_acronym_letters(&word))
        .collect()
}
