pub fn is_prime(n: u64, primes: &[u64]) -> bool {
    let floored_square_root = (n as f64).sqrt().floor() as u64;
    if floored_square_root == 1 {
        return true;
    }

    primes.iter()
        .take_while(|&&p| p <= floored_square_root)
        .all(|&p| n % p != 0)
}

pub fn factors(n: u64) -> Vec<u64> {
    let primes: Vec<u64> = (2..=n).fold(vec![], |mut acc, x| {
        if is_prime(x, &acc) {
            acc.push(x);
        }
        acc
    });
    let mut factors: Vec<u64> = vec![];
    let mut n2 = n;
    while n2 != 1 {
        let highest_prime_factor = primes.iter().rev().find(|&&x| n2 % x == 0).unwrap_or(&1);
        factors.insert(0, *highest_prime_factor);
        n2 /= highest_prime_factor;
    }
    factors
}
