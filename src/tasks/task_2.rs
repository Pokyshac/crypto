use std::{io::{stdout, Write}, process::Stdio};
use crate::tasks::utils;

pub fn get_moved_deduction_system(a: i64, b: i64, m: i64) -> Result<Vec<i64>, String> {
    if m <= 0 {
        return Err(String::from("Argument 'm' can not be lower or equal zero"));
    }
    
    let mut result = Vec::with_capacity(m as usize);
    for i in 0..m {
        result.push(utils::modulo(a * i + b, m).1);
    }

    Ok(result)
}

pub fn print_deduction_system(system: &Vec<i64>) {
    print!("[");
    for i in 0..system.len() - 1 {
        print!("{}, ", i64::abs(system[i]));
    }
    print!("{}]", i64::abs(system.last().unwrap().clone()));
    stdout().flush().unwrap();
}
