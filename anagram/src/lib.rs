use std::collections::HashSet;

fn is_anagram(word: &str, candidate: &str) -> bool {
    let lowered_word = word.to_lowercase();
    let lowered_candidate = candidate.to_lowercase();
    if lowered_word == lowered_candidate || lowered_word.len() != lowered_candidate.len() {
        return false;
    }
    let mut word_characters: Vec<char> = lowered_word.chars().collect();
    let mut candidate_characters: Vec<char> = lowered_candidate.chars().collect();
    word_characters.sort_unstable();
    candidate_characters.sort_unstable();

    word_characters == candidate_characters
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams
        .iter()
        .filter(|&&candidate| is_anagram(word, candidate))
        .copied()
        .collect()
}
