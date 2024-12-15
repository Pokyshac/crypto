use std::collections::HashMap;

use crate::task_3::calculate_eulers_function;

use crate::utils::gcd;
use crate::utils::is_valid_comparison;

pub fn get_deduction_orders(m: i64) -> Vec<(i64, i64)> {
    let phi = calculate_eulers_function(m).unwrap();
    let mut result = Vec::<(i64, i64)>::with_capacity(phi as usize);
    for a in 2..m {
        if gcd(m, a) == 1 {
            let mut l = 1;
            while !is_valid_comparison(a, l, 1, m) {
                l += 1;
            }

            result.push((a, l));
        }
    }

    result
}
