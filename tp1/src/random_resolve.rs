use optim::knapsack::Knapsack;
use optim::solution::Solution;
use optim::resolver::Resolver;

use crate::solution::KnapsackSolution;

pub type BestSolution = (f32, Vec<bool>, f32, i32);

pub struct RandomResolver<'a> {
    knapsack: &'a Knapsack,
    length: usize,
    fitness: f32,
}

impl<'a> RandomResolver<'a> {

    pub fn new(knapsack: &'a Knapsack, fitness: f32, length: usize) -> Self {
        RandomResolver {
            knapsack,
            fitness, 
            length,
        }
    }
    
    pub fn best_solution(&mut self, number_execution: i32) -> BestSolution {
        let mut best_solution :Vec<bool> = Vec::new();
        let mut best_profit :f32 = 0.0;
        let mut best_weight :f32 = 0.0;
        let mut best_attempt :i32 = 0;

        for attempt in 0..number_execution {
            let result = self.resolve();
            if result.0 > best_profit {
                best_profit = result.0;
                best_solution = result.1;
                best_weight = result.2;
                best_attempt = attempt;
            }
        }

        (best_profit, best_solution, best_weight, best_attempt)  
    }

}

impl<'a> Resolver<BestSolution> for RandomResolver<'a> {

    fn resolve(&mut self) -> (f32, Vec<bool>, f32) {
        execute(self.length, self.fitness, self.knapsack)
    }

    fn multiple_resolve(&mut self, number_execution: i32) -> Vec<BestSolution> {
        let mut res = Vec::new();

        for _ in 1..number_execution {
            res.push(self.best_solution(100000));
        }

        res
    }

}

fn execute(length: usize, fitness: f32, knapsack: &Knapsack) -> (f32, Vec<bool>, f32) {
    let generated_solution = generate_solution(length);
    let solution = KnapsackSolution::new(&generated_solution, fitness);
    let sum_weight = knapsack.sum_weight(&generated_solution);
    (solution.evaluate(knapsack, &sum_weight), generated_solution, sum_weight)
}

pub fn generate_solution(length: usize) -> Vec<bool> {
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

