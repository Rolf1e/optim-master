use rand::prelude::*;
use optim::knapsack::Knapsack;
use optim::Solution;

use crate::solution::KnapsackSolution;

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


fn generate_solution(length: usize) -> Vec<bool> {
    let mut vec = Vec::new();
    for _ in 0.. length {
       vec.push(rand::random()); 
    }
    vec
}


pub fn walk_execution(knapsack: &Knapsack, number_execution: i32, fitness: f32, length: usize) -> Vec<(f32, Vec<bool>, f32)> {
    let mut result = Vec::new();

    let mut generated_solution = generate_solution(length);

    for _ in 0..number_execution {
        let exec = execute_random_walk(length, fitness, knapsack, &mut generated_solution);
        result.push((exec.0, exec.1.to_vec(), exec.2));
    }

    result
}

fn execute_random_walk<'a>(length: usize, fitness: f32, knapsack: &Knapsack, generated_solution: &'a mut [bool]) -> (f32, &'a [bool], f32) {
    let generated_solution = generate_walk_solution(generated_solution, length);
    let solution = KnapsackSolution::new(&generated_solution, fitness);
    let sum_weight = knapsack.sum_weight(&generated_solution);
    (solution.evaluate(knapsack, &sum_weight), generated_solution, sum_weight)
}

fn generate_walk_solution<'a>(generated_solution: &'a mut [bool], length: usize) -> &'a [bool] {
    let rand :usize = rand::thread_rng()
        .gen_range(0, length);

    if let true = generated_solution[rand] {
        generated_solution[rand] = false;
    } else {
        generated_solution[rand] = true;
    }

    generated_solution
}


#[test]
fn should_generate_vec_of_bool() {
    let gen = generate_solution(4);
    assert_eq!(4, gen.len());
}




