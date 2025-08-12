#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn is_sublist_of(candidate: &[i32], list: &[i32]) -> bool {
    candidate.is_empty() || list.windows(candidate.len()).any(|window| window == candidate)
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }
    if is_sublist_of(first_list, second_list) {
        return Comparison::Sublist;
    }
    if is_sublist_of(second_list, first_list) {
        return Comparison::Superlist;
    }
    Comparison::Unequal
}
