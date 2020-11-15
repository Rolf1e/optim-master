use rand::prelude::*;

use optim::knapsack::Knapsack;
use optim::resolver::Resolver;
use optim::solution::Solution;

use crate::solution::KnapsackSolution;

pub type BestSolution = (f32, Vec<bool>, f32);

pub struct WalkResolver<'a> {
    knapsack: &'a Knapsack,
    choosed_items: Vec<bool>,
    length: usize,
    fitness: f32,
}

impl<'a> WalkResolver<'a> {
    pub fn new(
        fitness: f32,
        knapsack: &'a Knapsack,
        length: usize,
        choosed_items: Vec<bool>,
    ) -> Self {
        WalkResolver {
            knapsack,
            choosed_items,
            length,
            fitness,
        }
    }
}

impl<'a> Resolver<BestSolution> for WalkResolver<'a> {
    fn resolve(&mut self) -> (f32, Vec<bool>, f32) {
        let generated_solution = generate_walk_solution(self.length, &mut self.choosed_items);
        let sum_weight = self.knapsack.sum_weight(generated_solution);
        let solution = KnapsackSolution::new(&self.knapsack, self.fitness);
        (
            solution.evaluate(generated_solution),
            generated_solution.to_vec(),
            sum_weight,
        )
    }

    fn multiple_resolve(&mut self, number_execution: i32) -> Vec<BestSolution> {
        let mut result = Vec::with_capacity(number_execution as usize);

        for _ in 0..number_execution {
            let exec = self.resolve();
            result.push((exec.0, exec.1, exec.2));
        }

        result
    }
}

pub fn generate_walk_solution<'a>(length: usize, generated_solution: &'a mut [bool]) -> &'a [bool] {
    let rand: usize = rand::thread_rng().gen_range(0, length);

    generated_solution[rand] = !generated_solution[rand];

    generated_solution
}

#[test]
fn should_generate_vec_of_bool() {
    let mut array = [true, true, false, true];
    let gen = generate_walk_solution(4, &mut array);
    assert_eq!(4, gen.len());
}
