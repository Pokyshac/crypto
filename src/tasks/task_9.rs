use num::PrimInt;

use crate::{tasks::utils::is_valid_comparison, utils};

// pub fn find_base(n: i64) -> i64 {
//     let prime_factors = utils::get_prime_factors(n).unwrap();
//     let remainders: Vec<i64> = prime_factors.iter()
//                                             .map(|x| utils::modulo(n - 1, *x).1)
//                                             .collect();

//     let k = prime_factors.len();
//     let mut b = 1;
//     'b_iter: loop {
//         b += 1;
//         for prime_factor in prime_factors.iter() {
//             if utils::gcd(b, *prime_factor) != 1 {
//                 continue 'b_iter;    
//             }        
//         }

//         for remainder in remainders.iter() {
//             if !is_valid_comparison(b, *remainder, 1, n) {
//                 continue 'b_iter;
//             }
//         }    

//         return b;
//     }
// }

pub fn find_base(n: i64) -> i64 {
    let mut b = 2;
    
    while b < n {
        if is_valid_comparison(b, n-1, 1, n) {
            break;
        }
        
        b += 1;
    }

    b
}
