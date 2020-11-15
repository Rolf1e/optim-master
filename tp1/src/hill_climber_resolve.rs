use optim::knapsack::Knapsack;
use optim::resolver::Resolver;
use optim::solution::Solution;

use crate::random_resolve::generate_solution;
use crate::solution::KnapsackSolution;

pub type BestSolution = (f32, Vec<bool>, f32);

pub struct HillClimberResolver<'a> {
    knapsack: &'a Knapsack,
    length: usize,
    fitness: f32,
}

impl<'a> HillClimberResolver<'a> {
    pub fn new(knapsack: &'a Knapsack, length: usize, fitness: f32) -> Self {
        HillClimberResolver {
            knapsack,
            length,
            fitness,
        }
    }
}

impl<'a> Resolver<BestSolution> for HillClimberResolver<'a> {
    fn resolve(&mut self) -> BestSolution {
        let mut best_solution = generate_solution(self.length);
        let solution = KnapsackSolution::new(self.knapsack, self.fitness);

        for i in 0..self.length {
            let eval = solution.evaluate(&best_solution);
            let new_solution = generate_hill_climber_solution(&mut best_solution, i as usize);

            let eval2 = solution.evaluate(&new_solution);
            //println!("eval1: {} | eval2 {}", eval, eval2);

            if eval < eval2 {
                best_solution = new_solution.to_vec();
                //println!("update eval1: {}", solution.evaluate(&mut best_solution));
            }
        }

        (
            0.0,
            best_solution.clone(),
            self.knapsack.sum_weight(&best_solution),
        )
    }

    #[warn(unused_variables)]
    fn multiple_resolve(&mut self, _number_execution: i32) -> Vec<BestSolution> {
        unimplemented!("Does not make real sense to implement");
    }
}

pub fn generate_hill_climber_solution<'a>(
    generated_solution: &'a mut [bool],
    index: usize,
) -> Vec<bool> {
    let mut new = Vec::from(generated_solution);
    new[index] = !new[index];
    new
}
