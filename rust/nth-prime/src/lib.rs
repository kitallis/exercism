fn is_prime(n: u32) -> bool {
    for j in 2..n {
        if n % j == 0 {
            return false;
        }
    }

    true
}

pub fn nth(n: u32) -> Option<u32> {
    if n < 1 {
        return None;
    }

    let mut primes = 0;
    let mut nth_prime: u32 = 1;

    while primes < n {
        nth_prime += 1;

        if is_prime(nth_prime) {
            primes += 1;
        }
    }

    Some(nth_prime)
}
