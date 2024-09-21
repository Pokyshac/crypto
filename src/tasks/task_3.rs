use std::collections::{HashMap, HashSet};

use num::{iter::Range, Rational64};

pub fn calculate_eulers_function(n: i64) -> Result<i64, String> {
    if n <= 0 {
        return Err(String::from("Argument can not be lower or equal zero"))
    }
    
    let prime_factors = get_prime_factors(n).unwrap();
    let mut result = Rational64::new(1, 1);
    for value in prime_factors.iter() {
        result *= Rational64::new(-1, *value) + 1;
    }

    Ok(n * result.numer() / result.denom()) 
}

pub fn get_prime_factors(n: i64) -> Result<HashSet<i64>, String>{
    if n <= 0 {
        return Err(String::from("Argument can not be lower or equal zero"))
    }
        
    let mut result: HashSet<i64> = HashSet::with_capacity(n as usize);
    let mut n = n;
    let mut delimiter = 2;
    while delimiter * delimiter <= n {
        if n % delimiter == 0 {
            result.insert(delimiter);
            n /= delimiter;
        }
        else {
            delimiter += 1;
        }
    }
    if n > 1 {
        result.insert(n);
    }

    Ok(result)
}
