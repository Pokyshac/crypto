use num::{BigInt, FromPrimitive};

use crate::utils::is_prime;

pub fn is_mersenn_prime(n: i64) -> bool {
    if n == 2 {
        return true;
    }
    if is_prime(n) {
        let mut s: i64 = 4;    
        let m = i64::pow(2, n as u32) - 1;
        for k in 1..n - 1 {
            s = s * s - 2;
        }
        if s % m == 0 {
            return true;
        }
        else {
            return false;
        }
    }
    false
}
