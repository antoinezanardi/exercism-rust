struct Square<'a> {
    top_line: Option<&'a [char]>,
    middle_line: &'a [char],
    bottom_line: Option<&'a [char]>,
}

fn get_flowers_count_in_range(tiles: Option<&[char]>, range: (usize, usize)) -> u8 {
    tiles
        .map(|t| t[range.0..=range.1].iter().filter(|&&c| c == '*').count() as u8)
        .unwrap_or(0)
}

fn get_annotated_tile(sub_garden: &Square, column_index: usize) -> char {
    let current_tile = sub_garden.middle_line[column_index];
    if current_tile != ' ' {
        return current_tile;
    }
    let line_len = sub_garden.middle_line.len();
    let low_bound = column_index.saturating_sub(1);
    let high_bound = (column_index + 1).min(line_len - 1);
    let flowers_count =
        get_flowers_count_in_range(Some(sub_garden.middle_line), (low_bound, high_bound))
            + get_flowers_count_in_range(sub_garden.top_line, (low_bound, high_bound))
            + get_flowers_count_in_range(sub_garden.bottom_line, (low_bound, high_bound));

    if flowers_count == 0 {
        ' '
    } else {
        char::from_digit(flowers_count as u32, 10).unwrap_or(current_tile)
    }
}

fn get_annotated_line(sub_garden: &Square) -> String {
    (0..sub_garden.middle_line.len())
        .map(|col| get_annotated_tile(sub_garden, col))
        .collect()
}

pub fn annotate(garden: &[&str]) -> Vec<String> {
    let parsed: Vec<Vec<char>> = garden.iter().map(|line| line.chars().collect()).collect();

    parsed
        .iter()
        .enumerate()
        .map(|(i, middle)| {
            let square = Square {
                top_line: if i > 0 { Some(&parsed[i - 1]) } else { None },
                middle_line: middle.as_slice(),
                bottom_line: parsed.get(i + 1).map(|v| v.as_slice()),
            };
            get_annotated_line(&square)
        })
        .collect()
}
