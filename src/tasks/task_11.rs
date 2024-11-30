use crate::utils::{is_prime, gcd, is_valid_comparison};

pub fn find_pseudoprime(b: i64) -> i64 {
    let mut n = 4;
    loop {
        if !is_prime(n) {
            if is_valid_comparison(b, n-1, 1, n) {
                return n;
            }
        }

        n += 1;
    }
}
