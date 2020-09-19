use crate::solution::Solution;
use crate::knapsack::Knapsack;
use crate::parsing;

 //number of time we want to test
 //return best solution and profit
pub fn random_execution(file_name: &str, number_execution: i32) -> (f32, Vec<bool>, f32, i32) {
    let file_content = parsing::create_knapsack_from_file(file_name);
    let mut best_solution :Vec<bool> = Vec::new();
    let mut best_profit :f32 = 0.0;
    let mut best_weight :f32 = 0.0;
    let mut best_attempt :i32 = 0;
    let knapsack = &file_content.0;

    for attempt in 0..number_execution {
        let result = execute(file_content.2, file_content.1, knapsack);
        if result.0 > best_profit {
            best_profit = result.0;
            best_solution = result.1;
            best_weight = result.2;
            best_attempt = attempt;
        }
    }

    (best_profit, best_solution, best_weight, best_attempt)    
}


fn execute(length: usize, fitness: f32, knapsack: &Knapsack) -> (f32, Vec<bool>, f32) {
    let generated_solution = generate_solution(length);
    let solution = Solution::new(&generated_solution, fitness);
    (solution.evaluate(knapsack, 10.0), generated_solution.clone(), knapsack.sum_weight(&generated_solution))
}

//TODO improve so solution is less random 
fn generate_solution(length: usize) -> Vec<bool> {
    let mut vec = Vec::new();
    for _ in 0.. length {
       vec.push(rand::random()); 
    }
    vec
}

#[test]
fn should_generate_vec_of_bool() {
    let gen = generate_solution(4);
    assert_eq!(4, gen.len());
}

