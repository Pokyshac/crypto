use crate::tasks::utils::modulo;
use crate::tasks::task_3::calculate_eulers_function;

pub fn remainder(a: i64, k: i64, m: i64) -> Result<i64, String> {
    let phi = match calculate_eulers_function(m) {
        Ok(x) => x,
        Err(error) => return Err(error) 
    };
    let mut d = modulo(a, m).1;
    let mut c = modulo(k, phi).1;
    let mut t = 1;
    
    while c > 1 {
        if c % 2 != 0 {
            c -= 1;
            t *= d;
            t = modulo(t, m).1;
        }
        
        d = modulo(d * d, m).1;
        
        c /= 2;
    }
    let result = modulo(t * d, m).1;

    Ok(result)
}
