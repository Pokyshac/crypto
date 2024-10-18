#![allow(
    unused_imports,
    unused_variables,
    unused_mut,
    dead_code
)]

use std::io::Write;
use std::process::Stdio;

use num::iter::Range;
use tasks::task_1;
use tasks::task_2;
use tasks::task_3;
use tasks::task_4;
use tasks::task_5;
use tasks::task_6;
use tasks::utils;

use crate::tasks::utils::get_prime_factors;

mod tasks;

fn main() {
    // let m = 5;
    // let mult_table = task_1::get_multiplication_table(m);
    // let add_table = task_1::get_adding_table(m);

    // let output_table = task_1::table_to_string(&mult_table, m as usize).unwrap();
    // println!("{}", output_table);

    // let a = 3;
    // let b = 1;
    // let m = 5;
    // let system = task_2::get_moved_deduction_system(a, b, m).unwrap();
    // task_2::print_deduction_system(&system);

    // let n = 5;
    // println!("{}", calculate_eulers_function(n).unwrap());

    // let a = 17;
    // let k = 2001;
    // let m = 1000;
    // let result =  task_5::remainder(a, k, m);
    // match result {
    //     Ok(x) => println!("{}", x),
    //     Err(error) => println!("{}", error)
    // }

    // let m = 48;
    // println!("{}", task_4::find_eulers_function_arguments(m).unwrap().len());

    let mut alphabet = task_6::Alphabet::new();
    for i in 97..110 {
        alphabet.insert(char::from_u32(i as u32).unwrap(), i - 96);
    }   
    alphabet.insert(' ', 15);

    let text = String::from("abd ca aba");
    println!("{}", text);
    println!("-----------------");

    let (open_key, hidden_key) = task_6::get_rsa_keys(19, 41);

    let encoded_text = task_6::rsa_encode(&open_key, &text, &alphabet);
    let decoded_text = task_6::rsa_decode(&hidden_key, &encoded_text, &alphabet);

    println!("{:?}", encoded_text);
    println!("{:?}", decoded_text);
}
