const STUDENTS: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

fn letter_to_plant(letter: char) -> Option<&'static str> {
    match letter {
        'V' => Some("violets"),
        'R' => Some("radishes"),
        'C' => Some("clover"),
        'G' => Some("grass"),
        _ => None,
    }
}

fn get_plants_for_student(garden_line: &str, student: &str) -> Option<Vec<&'static str>> {
    let student_idx = STUDENTS.iter().position(|&name| name == student)?;
    let start = student_idx * 2;
    let end = start + 2;

    garden_line
        .get(start..end)?
        .chars()
        .map(letter_to_plant)
        .collect()
}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    diagram
        .lines()
        .filter_map(|line| get_plants_for_student(line, student))
        .flatten()
        .collect()
}
