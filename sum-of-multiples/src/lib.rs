pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let factors_without_zero: Vec<u32> = factors
        .iter()
        .filter(|&&factor| factor != 0)
        .cloned()
        .collect();

    (1..limit)
        .filter(|n| factors_without_zero.iter().any(|factor| n % factor == 0))
        .sum()
}
