#![allow(
    unused_imports,
    unused_variables,
    unused_mut,
    dead_code
)]

use tasks::task_1;
use tasks::task_2;
use tasks::task_3;
use tasks::task_4;
use tasks::task_5;
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

    // let a = 7;
    // let k = 2024;
    // let m = 13;
    // let result =  task_5::remainder(a, k, m);
    // match result {
    //     Ok(x) => println!("{}", x),
    //     Err(error) => println!("{}", error)
    // }

    // let m = 48;
    // println!("{}", task_4::find_eulers_function_arguments(m).unwrap().len());

    for i in 2..128 {
        let v = task_4::find_eulers_function_arguments(i).unwrap().len();

        let mut cnt = 0;
        let mut n = 1;
        while n <= 10000 {
            let phi_n = task_3::calculate_eulers_function(n).unwrap();
            if phi_n == i {
                cnt += 1;
            }

            n += 1;
        }

        if v != cnt {
            println!("govno v {}", i);
            panic!("{} != {}", v, cnt);
        }
    }
}
