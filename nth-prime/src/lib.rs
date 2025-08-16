pub fn is_prime(n: u32, primes: &Vec<u32>) -> bool {
    let floored_square_root = (n as f64).sqrt().floor() as u32;
    if floored_square_root == 1 {
        return true
    }

    primes.iter()
        .take_while(|&&p| p <= floored_square_root)
        .all(|&p| n % p != 0)
}

pub fn nth(nth: u32) -> u32 {
    let mut primes: Vec<u32> = vec![];
    let mut candidate = 2;
    let ending_num= (nth + 1) as usize;

    while primes.len() != ending_num {
        if is_prime(candidate, &primes) {
            primes.push(candidate);
        }
        candidate += 1;
    }

    *primes.last().unwrap()
}
