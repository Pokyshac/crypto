use crate::task_3::calculate_eulers_function;
use crate::tasks::utils::get_prime_factors;

pub fn find_eulers_function_arguments(m: i64) -> Result<Vec<i64>, String> {
    if m <= 0 {
        return Err(String::from("Argument 'm' must be greater than zero"))
    }
    
    let mut result = Vec::with_capacity(2 * m as usize);
    let mut n = 2;
    while (f64::sqrt(n as f64 / 2.0) as i64) <= m {
        let phi = calculate_eulers_function(n).unwrap();
        if phi == m {
            result.push(n);
        }

        n += 1;
    } 

    Ok(result)
}

