use std::collections::HashSet;

fn get_sorted_word(word: String) -> Vec<char> {
    let mut word_characters: Vec<char> = word.chars().collect();
    word_characters.sort_unstable();

    word_characters
}

fn is_anagram(word: &str, candidate: &str) -> bool {
    let lowered_word = word.to_lowercase();
    let lowered_candidate = candidate.to_lowercase();
    if lowered_word == lowered_candidate || lowered_word.len() != lowered_candidate.len() {
        return false;
    }
    let sorted_word = get_sorted_word(lowered_word);
    let sorted_candidate = get_sorted_word(lowered_candidate);

    sorted_word == sorted_candidate
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams
        .iter()
        .filter(|&&candidate| is_anagram(word, candidate))
        .copied()
        .collect()
}
