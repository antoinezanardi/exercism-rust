const BOTTLES: [&str; 11] = [
    "No green bottles",
    "One green bottle",
    "Two green bottles",
    "Three green bottles",
    "Four green bottles",
    "Five green bottles",
    "Six green bottles",
    "Seven green bottles",
    "Eight green bottles",
    "Nine green bottles",
    "Ten green bottles",
];

fn get_verse(verse: u32) -> String {
    let current_verse_bottles = BOTTLES[verse as usize];
    let next_verse_bottles = BOTTLES[verse.saturating_sub(1) as usize].to_lowercase();

    format!(
        "{current_verse_bottles} hanging on the wall,\n\
        {current_verse_bottles} hanging on the wall,\n\
        And if one green bottle should accidentally fall,\n\
        There'll be {next_verse_bottles} hanging on the wall.\n"
    )
}

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let start_bottles = start_bottles.clamp(1, 10);
    let ending_lyric = start_bottles.saturating_sub(take_down).clamp(0, start_bottles);
    (ending_lyric..start_bottles)
        .rev()
        .map(|idx| get_verse(idx + 1))
        .collect::<Vec<_>>()
        .join("\n")
}
