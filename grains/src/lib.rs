pub fn square(s: u32) -> u64 {
    if s == 0 {
        panic!("Square must be positive");
    }

    2_u64.pow(s - 1)
}

pub fn total() -> u64 {
    (1..=64).map(square).sum()
}
