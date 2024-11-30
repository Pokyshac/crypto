use crate::utils::is_valid_comparison;

pub fn test_ferma(n: i64) -> bool {
    for i in 2..n {
        if !is_valid_comparison(i, n-1, 1, n) {
            return false;
        }
    }

    true
}
