fn brackets_reducer(mut brackets: Vec<char>, char: char) -> Option<Vec<char>> {
    match char {
        '(' | '[' | '{' => brackets.push(char),
        ')' => if brackets.pop() != Some('(') { return None },
        ']' => if brackets.pop() != Some('[') { return None },
        '}' => if brackets.pop() != Some('{') { return None },
        _ => {}
    }

    Some(brackets)
}

pub fn brackets_are_balanced(string: &str) -> bool {
    string
        .chars()
        .try_fold(Vec::new(), brackets_reducer)
        .is_some_and(|vec| vec.is_empty())
}
