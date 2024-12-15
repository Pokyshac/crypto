#![allow(
    unused_imports,
    unused_variables,
    unused_mut,
    dead_code
)]

use stopwatch::Stopwatch;

use std::io::Write;
use std::process::Stdio;

use num::iter::Range;
use tasks::task_1;
use tasks::task_2;
use tasks::task_3;
use tasks::task_4;
use tasks::task_5;
use tasks::task_6;
use tasks::task_7;
use tasks::task_8;
use tasks::task_9;
use tasks::task_10;
use tasks::task_11;
use tasks::task_12;
use tasks::task_13;
use tasks::utils;

mod tasks;

fn main() {
    // let m = 5;
    // let mult_table = task_1::get_multiplication_table(m);
    // let add_table = task_1::get_adding_table(m);

    // let output_table = task_1::table_to_string(&mult_table, m as usize).unwrap();
    // println!("{}", output_table);

    // let a = 12;
    // let b = 3;
    // let m = 19;
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

    // let mut alphabet = task_6::Alphabet::new();
    // for i in 97..123 {
    //     // println!("{}",char::from_u32(i as u32).unwrap());
    //     alphabet.insert(char::from_u32(i as u32).unwrap(), i - 96);
    // }   
    // alphabet.insert(' ', 123);

    // let text = String::from("da");
    // println!("{}", text);
    // println!("-----------------");

    // let (open_key, hidden_key) = task_6::get_rsa_keys(2153, 3779);

    // let encoded_text = task_6::rsa_encode(&open_key, &text, &alphabet);
    // let decoded_text = task_6::rsa_decode(&hidden_key, &encoded_text, &alphabet);

    // println!("{:?}", encoded_text);
    // println!("{:?}", decoded_text);

    // let s = task_7::solve_comparison_system(3, 11);
    // println!("{}", utils::table_to_string(&s, 3, 11).unwrap());

    // let n = 30;
    // println!("{}", task_8::is_mersenn_prime(n));

    // let n = 1000;
    // println!("{}", task_10::find_witness(n));

    // let n = 15;
    // println!("{}", task_9::find_base(n));

    // let b = 5;
    // println!("{}", task_11::find_pseudoprime(b));

    // let n = 103;
    // println!("{}", task_12::test_ferma(n));

    let m = 89;
    let r = task_13::find_primitive_residues(m);
    for v in r.iter() {
        println!("{}", v);
    }
}
