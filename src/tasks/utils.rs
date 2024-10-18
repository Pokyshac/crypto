use std::{collections::HashSet, i64};

pub fn modulo(a: i64, b: i64) -> (i64, i64) {
    let signum = i64::signum(a) * i64::signum(b);
    let mut q: i64;
    let mut r: i64;
    q = a / b;
    if a < q * b {
        q += signum;
    }
    r = a - q * b;
    
    (q, r)
}

pub fn get_prime_factors(n: i64) -> Result<HashSet<i64>, String>{
    if n <= 0 {
        return Err(String::from("Argument must be greater than zero"))
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

pub fn euclidean_algorithm(a: i64, b: i64) -> (i64, i64) {
    let mut a = a;
    let mut b = b;
    let mut x1 = 0;
    let mut x2 = 1;
    let mut y1 = 1;
    let mut y2 = 0;

    while b > 0 {
        let (q, r) = modulo(a, b);     
        let x = x2 - q * x1;
        let y = y2 - q * y1;
        a = b;
        b = r;
        x2 = x1;
        x1 = x;
        y2 = y1;
        y1 = y;
    }

    return (x2, y2)
}
