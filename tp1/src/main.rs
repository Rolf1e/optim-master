#![feature(test)]
pub mod parsing;
pub mod solution;
pub mod knapsack;
pub mod resolve; 

fn main() {
    let number_execution = 100000;
    println!("Execute {} times", number_execution);
    let file_content = parsing::create_knapsack_from_file("data1000.txt");
    let res = resolve::random_execution(&file_content.0, number_execution, file_content.1, file_content.2);
    println!("Profit: {}\nWeight: {}\nAttempt: {}\nSolution: {:?}", res.0, res.2, res.3, res.1);
}



