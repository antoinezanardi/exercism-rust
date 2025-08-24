use std::collections::{HashMap, HashSet};

fn get_members(input: &str) -> Option<(Vec<&str>, &str)> {
    let (lhs, rhs) = input.split_once("==")?;
    let operation: Vec<&str> = lhs.split('+').map(str::trim).collect();
    let result = rhs.trim();

    Some((operation, result))
}

fn get_unique_letters(input: &str) -> Vec<char> {
    let mut unique_letters = HashSet::new();

    input
        .chars()
        .filter(|c| c.is_alphabetic() && unique_letters.insert(*c))
        .collect()
}

fn get_letters_not_eligible_to_zero(unique_letters: &[char], operands: &[&str]) -> HashSet<char> {
    unique_letters
        .iter()
        .copied()
        .filter(|&letter| operands.iter().copied().any(|str| str.starts_with(letter)))
        .collect()
}

fn parse_number_from_candidate(number: &str, candidate: &HashMap<char, u8>) -> Option<u64> {
    number.chars().try_fold(0u64, |acc, c| {
        let &digit = candidate.get(&c)?;
        Some(acc * 10 + digit as u64)
    })
}

fn is_solving_candidate(candidate: &HashMap<char, u8>, operation: &[&str], result: &str) -> bool {
    let operation_sum: u64 = operation
        .iter()
        .filter_map(|&number| parse_number_from_candidate(number, candidate))
        .sum();
    let Some(expected_result) = parse_number_from_candidate(result, candidate) else {
        return false;
    };

    operation_sum == expected_result
}

fn search_solution(
    letters: &[char],
    numbers: &mut Vec<u8>,
    letters_not_eligible_to_zero: &HashSet<char>,
    current_assignment: &mut HashMap<char, u8>,
    operation: &[&str],
    result: &str,
) -> Option<HashMap<char, u8>> {
    if letters.is_empty() {
        return if is_solving_candidate(current_assignment, operation, result) {
            Some(current_assignment.clone())
        } else {
            None
        };
    }

    let first_letter = letters[0];
    for i in 0..numbers.len() {
        let number = numbers[i];
        if number == 0 && letters_not_eligible_to_zero.contains(&first_letter) {
            continue;
        }

        current_assignment.insert(first_letter, number);
        let removed = numbers.remove(i);
        if let Some(solution) =
            search_solution(&letters[1..], numbers, letters_not_eligible_to_zero, current_assignment, operation, result)
        {
            return Some(solution);
        }
        numbers.insert(i, removed);
        current_assignment.remove(&first_letter);
    }

    None
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let (operation, result) = get_members(input)?;
    let unique_letters = get_unique_letters(input);
    let operands = [operation.clone(), vec![result]].concat();
    let letters_not_eligible_to_zero = get_letters_not_eligible_to_zero(&unique_letters, &operands);

    search_solution(
        &unique_letters,
        &mut vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        &letters_not_eligible_to_zero,
        &mut HashMap::new(),
        &operation,
        result,
    )
}
