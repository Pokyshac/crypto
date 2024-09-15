#![allow(
    unused_imports,
    unused_variables,
    unused_mut,
    dead_code
)]

use tasks::task_1;

mod tasks;

fn main() {
    let m = 5;
    let mult_table = task_1::get_multiplication_table(m);
    let add_table = task_1::get_adding_table(m);

    let output_table = task_1::table_to_string(&mult_table, m).unwrap();
    println!("{}", output_table);
}
