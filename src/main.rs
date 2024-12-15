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
use tasks::task_14;
use tasks::utils;

mod tasks;

fn main() {
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

    // let m = 89;
    // let r = task_13::find_primitive_residues(m);
    // for v in r.iter() {
    //     println!("{}", v);
    // }

    let m = 10;
    let r = task_14::get_deduction_orders(m);
    for v in r.iter() {
        println!("{}: {}", v.0, v.1);
    }
}
