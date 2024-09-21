#![allow(
    unused_imports,
    unused_variables,
    unused_mut,
    dead_code
)]

use tasks::task_1;
use tasks::task_2;
use tasks::task_3;

use crate::tasks::task_3::calculate_eulers_function;

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
}
