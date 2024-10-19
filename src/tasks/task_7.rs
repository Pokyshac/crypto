use std::{collections::HashSet, thread::available_parallelism};

pub fn solve_comparison_system(m: i64, n: i64) -> Vec<i64> {
    let p = m * n;
    let mut solving_table = Vec::<i64>::with_capacity(p as usize);
    let mut available_values: HashSet<i64> = (0..m * n).collect();
    
    for i in 0..n {
        solving_table.push(
            find_suitable_value(0, i, m, n, &mut available_values)
        );
    }

    for i in 1..m {
        solving_table.push(
            find_suitable_value(i, 0, m, n, &mut available_values)
        );
        for j in 1..n {
            let index = (i - 1) * n + j - 1;
            solving_table.push(
                solving_table[index as usize] + 1
            );
        }
    }

    solving_table
}

fn find_suitable_value(a: i64, b: i64, m: i64, n: i64, available_values: &mut HashSet<i64>) -> i64 {
    let mut found_value = -1;
    for value in available_values.iter() {
        if (value - a) % m == 0 && (value - b) % n == 0 {
            found_value = *value;
            break;
        } 
    }
    available_values.remove(&found_value);
    
    found_value
}
