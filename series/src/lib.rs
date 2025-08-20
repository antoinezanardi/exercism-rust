pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec![];
    }

    digits
        .chars()
        .collect::<Vec<char>>()
        .windows(len)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect()
}
