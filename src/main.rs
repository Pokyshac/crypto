#![allow(
    unused_imports,
    unused_variables,
    unused_mut,
    dead_code
)]

use tasks::task_1;

mod tasks;

fn main() {
    let m = 3;
    let mult_table = task_1::get_multiplication_table(m);
    let add_table = task_1::get_adding_table(m);

    for i in 0..mult_table.len() {
        println!("{} | ", mult_table[i]);
    }
}
