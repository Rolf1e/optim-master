use crate::solution::KnapsackSolution;
use crate::knapsack::Knapsack;

 //number of time we want to test
 //return best solution and profit
pub fn random_execution(knapsack: &Knapsack, number_execution: i32, fitness: f32, length: usize) -> (f32, Vec<bool>, f32, i32) {
    let mut best_solution :Vec<bool> = Vec::new();
    let mut best_profit :f32 = 0.0;
    let mut best_weight :f32 = 0.0;
    let mut best_attempt :i32 = 0;

    for attempt in 0..number_execution {
        let result = execute(length, fitness, knapsack);
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
    let solution = KnapsackSolution::new(&generated_solution, fitness);
    let sum_weight = knapsack.sum_weight(&generated_solution);
    (solution.evaluate(knapsack, &sum_weight), generated_solution, sum_weight)
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




