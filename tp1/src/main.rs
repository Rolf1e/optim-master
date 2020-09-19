pub mod parsing;
pub mod solution;
pub mod knapsack;
pub mod resolve; 

fn main() {
    println!("Best : {:?}", resolve::random_execution("data1000.txt", 100));
}
