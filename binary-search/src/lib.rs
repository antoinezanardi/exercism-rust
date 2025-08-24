use std::cmp::Ordering::{Equal, Greater, Less};
use std::ops::RangeInclusive;

fn get_range_middle_idx(range: RangeInclusive<usize>) -> usize {
    let (start, end) = (*range.start(), *range.end());

    start + (end - start) / 2
}

fn find_recursively<T>(collection: &[T], key: &T, range: RangeInclusive<usize>) -> Option<usize>
where
    T: Ord,
{
    let (range_start, range_end) = (*range.start(), *range.end());
    let middle_idx = get_range_middle_idx(range);
    let middle_value = &collection[middle_idx];

    match middle_value.cmp(key) {
        Equal => Some(middle_idx),
        Greater if range_start < middle_idx => {
            find_recursively(collection, key, range_start..=middle_idx - 1)
        }
        Less if middle_idx < range_end => {
            find_recursively(collection, key, middle_idx + 1..=range_end)
        }
        _ => None,
    }
}

pub fn find<T, C>(collection: C, key: T) -> Option<usize>
where
    T: Ord,
    C: AsRef<[T]>,
{
    match collection.as_ref().len() {
        0 => None,
        len => find_recursively(collection.as_ref(), &key, 0..=len - 1),
    }
}
