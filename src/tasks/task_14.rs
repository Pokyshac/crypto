use std::collections::HashMap;
use std::rc::Rc;

use crate::task_3::calculate_eulers_function;

use super::utils::gcd;
use super::utils::is_valid_comparison;
use super::utils::get_prime_factors;
use super::utils::get_prime_factors_with_degrees;

// pub fn get_deduction_orders(m: i64) -> Vec<(i64, i64)> {
//     let phi = calculate_eulers_function(m).unwrap();
//     let mut result = Vec::<(i64, i64)>::with_capacity(phi as usize);
//     for a in 2..m {
//         if gcd(m, a) == 1 {
//             let mut l = 1;
//             while !is_valid_comparison(a, l, 1, m) {
//                 l += 1;
//             }

//             result.push((a, l));
//         }
//     }

//     result
// }

pub fn get_deduction_orders(m: i64) -> Vec<(i64, i64)> {
    let phi = calculate_eulers_function(m).unwrap();
    let mut phi_prime_factors = get_prime_factors_with_degrees(phi).iter()
        .map(|x| (*x.0, *x.1))
        .collect::<Vec<(i64, i64)>>();
    phi_prime_factors.sort_by(|x, y| {x.0.cmp(&y.0)});
    let mut flatten_prime_factors = Vec::<i64>::with_capacity(phi_prime_factors.iter().fold(0, |s, x| {s + x.1}) as usize);
    for p in phi_prime_factors.iter() {
        flatten_prime_factors.append(&mut vec![p.0; p.1 as usize]);
    }
    let s = flatten_prime_factors.len();
    let mut result = Vec::<(i64, i64)>::with_capacity(phi as usize);

    let mut r;
    for a in 2..m {
        if gcd(m, a) == 1 {
            for d in 1..s+1 {
                r = f(1, &m, &(a as i64), d, 0, &flatten_prime_factors);
                match r {
                    Some(x) => {result.push((a, x)); break},
                    None => continue
                }
            }
        }
    }
    
    result
}

fn f(value: i64, m: &i64, a: &i64, d: usize, start_id: usize, prime_factors: &Vec<i64>) -> Option<i64> {
    if d == 1 {
        for k in start_id..prime_factors.len() - d + 1 {
            if is_valid_comparison(*a, value * prime_factors[k], 1, *m) {
                return Some(value * prime_factors[k]);
            }
        }
    } else {
        for k in start_id..prime_factors.len() - d + 1 {
            match f(value * prime_factors[k], &m, &a, d - 1, k + 1, &prime_factors) {
                Some(x) => return Some(x),
                None => continue
            }
        }
    }
    
    None
}
