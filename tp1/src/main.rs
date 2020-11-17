#![feature(test)]
pub mod hill_climber_resolve;
pub mod parsing;
pub mod py_bindings;
pub mod random_resolve;
pub mod solution;
pub mod walk_resolve;
pub mod executor;

use crate::executor::*;

fn main() {
    let file_name = "data1000.txt";

    let number_execution = 10;
    let iterations = 1000;
    println!("execute on  {} iterations", number_execution * iterations);
    for _ in 0..5 {
        let res1 = one_thread_random(file_name, number_execution.clone(), iterations.clone());
        println!("OT - Random => P:{} W:{} T:{}ms", res1.0, res1.1, res1.2);

        let res3 = one_thread_walk(file_name, number_execution.clone());
        println!("OT - Walk => P:{} W:{} T:{}ms", res3.0, res3.1, res3.2);

        let res5 = one_thread_hill_climber(file_name, number_execution.clone());
        println!(
            "OT - Hill Climber BE => P:{} W:{} T:{}ms",
            res5.0, res5.1, res5.2
        );
        print!("\n");
    }
}

