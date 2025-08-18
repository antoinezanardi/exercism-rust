pub fn factors(mut n: u64) -> Vec<u64> {
    let mut prime_factors: Vec<u64> = vec![];
    if n < 2 {
        return prime_factors;
    }
    while n % 2 == 0 {
        prime_factors.push(2);
        n /= 2;
    }
    let mut factor = 3;
    while factor * factor <= n {
        while n % factor == 0 {
            prime_factors.push(factor);
            n /= factor;
        }
        factor += 2;
    }
    if n > 1 {
        prime_factors.push(n);
    }

    prime_factors
}
