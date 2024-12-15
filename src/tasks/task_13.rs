use crate::task_3::calculate_eulers_function;
use crate::utils::gcd;
use crate::utils::is_valid_comparison;
use crate::utils::get_prime_factors;

pub fn find_primitive_residues(m: i64) -> Vec<i64> {
    let phi = calculate_eulers_function(m).unwrap();
    let mut prime_factors = get_prime_factors(phi).unwrap();
    let mut result = Vec::<i64>::with_capacity(phi as usize);

    'g_iter: for g in 2..m {
        if gcd(m, g) == 1 {
            if !is_valid_comparison(g, phi / 2, 1, m) {
                for p in prime_factors.iter() {
                    if is_valid_comparison(g, phi / p, 1, m) {
                        continue 'g_iter;
                    }
                }

                let mut g = g;
                let mut k = 1;
                while k < phi && g < m {
                    if gcd(phi, k) == 1 {
                        result.push(g);
                    }
                    
                    g *= g;
                    k += 1;
                }

                break;
            }
        }
    }
    
    result
}
