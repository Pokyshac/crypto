use std::{collections::HashSet, i64};
use num::Integer;

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
