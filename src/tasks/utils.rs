use std::{collections::{HashMap, HashSet}, i64};
use num::Integer;

use crate::task_5::remainder;

pub fn is_prime(n: i64) -> bool {
    let range = num::integer::sqrt(n) + 1;
    for i in 2..range {
        if n % i == 0 {
            return false;
        }
    }
    
    true
}

pub fn is_valid_comparison(a: i64, k: i64, b: i64, n: i64) -> bool {
    check_comparison(a, k, b, n, 1)
}

fn check_comparison(a: i64, k: i64, b: i64, n: i64, c: i64) -> bool {
    if k == 1 {
        return modulo(a * c - b, n).1 == 0;
    }
    
    let mut k = k;
    if k % 2 == 0 {
        let d = modulo(a * a, n).1;
        return check_comparison(d, k / 2, b, n, c);
    } else {
        k -= 1;
        let d = modulo(a * a, n).1;
        let v = modulo(c * a, n).1;
        return check_comparison(d, k / 2, b, n, v);
    }
}

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

pub fn get_prime_factors_with_degrees(n: i64) -> HashMap<i64, i64> {
    let mut result = HashMap::<i64, i64>::with_capacity(n as usize);
    let mut n = n;
    let mut delimiter = 2;
    while delimiter * delimiter <= n {
        if n % delimiter == 0 {
            if result.contains_key(&delimiter) {
                *result.get_mut(&delimiter).unwrap() += 1;
            } else {
                result.insert(delimiter, 1);
            }
            n /= delimiter;
        }
        else {
            delimiter += 1;
        }
    }
    if n > 1 {
        if result.contains_key(&n) {
            *result.get_mut(&n).unwrap() += 1;
        } else {
            result.insert(n, 1);
        }
    }

    result
}

pub fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a;
    let mut b = b;
    let mut temp: i64;
    while b != 0 {
        temp = modulo(a, b).1;

        a = b;
        b = temp;
    }

    a
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

pub fn table_to_string<T: Integer + Clone + ToString>(table: &Vec<T>, rows: usize, cols: usize) -> Result<String, &str> {
    if rows == 0 || cols == 0 {
        return Err("Rows and columns count can not be zero");
    }
    
    let cell_size = digits_count(
            table.iter()
                .max()
                .unwrap()
                .clone()
        ) + 1;
    
    let mut result = String::with_capacity((cell_size * cols + rows) * 2);
    let mut k = 0;
    let mut row = String::with_capacity((cell_size + 1) * cols);
    for i in 0..rows * 2 {
        row.clear();
        if i % 2 == 0 {
            for j in 0..cols {
                let string_value = table[k].to_string();
                let spaces_count = cell_size - string_value.len();
                let spaces = vec![" "; spaces_count].join("");
                let cell = string_value + &spaces + "|";
                
                row.push_str(&cell);
                
                k += 1;
            }
        }
        else {
            for j in 0..cols {
                let cell = vec!["-"; cell_size].join("") + "|";
                
                row.push_str(&cell);
            }
        }
        result.push_str(&row);
        result.push_str("\n");
    }
    
    Ok(result)
}

fn digits_count<T: Integer + Clone + ToString>(value: T) -> usize {
    value.to_string().len()
}
