#![allow(
    unused_imports,
    unused_variables,
    unused_mut,
    dead_code
)]

use tasks::task_1;
use tasks::task_2;

mod tasks;

fn main() {
    // let m = 5;
    // let mult_table = task_1::get_multiplication_table(m);
    // let add_table = task_1::get_adding_table(m);

    // let output_table = task_1::table_to_string(&mult_table, m as usize).unwrap();
    // println!("{}", output_table);

    let a = -10;
    let b = 1;
    let m = 9;
    let system = task_2::get_moved_deduction_system(a, b, m);
    task_2::print_deduction_system(&system);
}
