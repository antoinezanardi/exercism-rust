fn get_proverb_line(list: &[&str], idx: usize) -> String {
    if idx + 1 == list.len() {
        let first_item = list[0];
        return format!("And all for the want of a {first_item}.");
    }
    let current_item = list[idx];
    let next_item = list[idx + 1];

    format!("For want of a {current_item} the {next_item} was lost.\n")
}

pub fn build_proverb(list: &[&str]) -> String {
    (0..list.len())
        .map(|idx| get_proverb_line(list, idx))
        .collect()
}
