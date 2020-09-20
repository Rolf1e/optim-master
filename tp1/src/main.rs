pub mod parsing;
pub mod solution;
pub mod knapsack;
pub mod resolve; 
pub mod py_bindings;

fn main() {
    let content = py_bindings::execute_multiple_time(100);
    println!("{:?}", content);
}
