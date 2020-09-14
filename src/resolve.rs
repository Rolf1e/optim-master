extern crate test;

use crate::solution::Solution;
use crate::parsing;

pub fn execute_random_research(nb_attempt: i32, file_name: &str) -> (Vec<bool>, f32, i32) {
    let mut best_attempt :Vec<bool> = Vec::new();
    let mut best_weight :f32 = 0.0;
    let mut best_attempt_number: i32 = 0;

    for attempt in 0..100 {
        let extracted_content = parsing::create_knapsack_from_file(file_name);
        let generated_solution = generate_solution(extracted_content.2);
        let knapsack = extracted_content.0;
        let solution = Solution::new(&generated_solution, extracted_content.1);

        //println!("{}", solution.display());

        if solution.is_working(&knapsack) {
            println!("attempt {} is working", attempt);
            let sum_weight = knapsack.sum_weight(&generated_solution);
            if best_weight < sum_weight {
                best_weight =  sum_weight;
                best_attempt = generated_solution;
                best_attempt_number = attempt;
            }
        }
        else {
            println!("attempt {} is not working", attempt);
        }
    }

    (best_attempt, best_weight, best_attempt_number)
}

#[test]
#[ignore = "to give a sample of how it works"]
fn should_execute_random_research() {
    let result = execute_random_research(10, "test2.txt");
    println!("{:?}", result);
}


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

