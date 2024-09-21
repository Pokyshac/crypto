use std::i64;

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
