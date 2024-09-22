use std::collections::{HashMap, HashSet};

use num::{iter::Range, Rational64};

use super::utils::get_prime_factors;

pub fn calculate_eulers_function(n: i64) -> Result<i64, String> {
    if n <= 0 {
        return Err(String::from("Argument must be greater than zero"))
    }
    
    let prime_factors = get_prime_factors(n).unwrap();
    let mut result = Rational64::new(1, 1);
    for value in prime_factors.iter() {
        result *= Rational64::new(-1, *value) + 1;
    }

    Ok(n * result.numer() / result.denom()) 
}
