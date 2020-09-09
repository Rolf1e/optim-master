#![feature(test)]
mod parsing;
mod solution;
mod knapsack;
mod resolve; 
mod bench;

use std::env;

fn main() {
    let args: Vec<String> = env::args()
        .collect();

    if args.len() <= 1 {
        panic!("==> Please give the number of attempt ");
    }

    let number_execution = args[1]
        .parse::<i32>()
        .unwrap(); 
    launch(number_execution);
}

pub fn launch(number_execution :i32) {
    for attempt in 0..number_execution {
        let random_execution = resolve::execute_random_research(attempt, "data1000.txt");
        //println!("Best weight {} for solution : {:?}", random_execution.1, random_execution.0);
        println!("Best weight {} for num : solution : {}", random_execution.1, attempt);
        let to_append = format!("{};{};", attempt.to_string(), random_execution.1.to_string());
        parsing::write_into_file(format!("output/output{}.csv", attempt.to_string()).as_str(), to_append.as_str());
    }
}


