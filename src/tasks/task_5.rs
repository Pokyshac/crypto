use crate::tasks::utils::modulo;
use crate::tasks::task_3::calculate_eulers_function;

pub fn remainder(a: i64, k: i64, m: i64) -> Result<i64, String> {
    let phi = match calculate_eulers_function(m) {
        Ok(x) => x,
        Err(error) => return Err(String::from("Argument 'm' must be greater than zero"))
    };
    let mut d = modulo(a, m).1;
    let mut r = modulo(k, phi).1;    
    let mut result = 1;
    if r % 2 != 0 {
        r -= 1;
        result = d;
    }
    
    while r > 1 {
        d = modulo(d * d, m).1;
        
        r /= 2;
    }
    result = modulo(result * d, m).1;

    Ok(result)
}
