use std::{io::{stdout, Write}, process::Stdio};

pub fn get_moved_deduction_system(a: i64, b: i64, m: i64) -> Vec<i64> {
    let mut result = Vec::with_capacity(m as usize);
    for i in 0..m {
        result.push((a * i + b) % m);
    }

    result
}

pub fn print_deduction_system(system: &Vec<i64>) {
    print!("[");
    for i in 0..system.len() - 1 {
        print!("{}, ", i64::abs(system[i]));
    }
    print!("{}]", i64::abs(system.last().unwrap().clone()));
    stdout().flush().unwrap();
}
