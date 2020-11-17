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
            solution.evaluate(&best_solution),
            best_solution.clone(),
            self.knapsack.sum_weight(&best_solution),
        )
    }

    fn multiple_resolve(&mut self, number_execution: i32) -> Vec<BestSolution> {
        let mut res = Vec::with_capacity(number_execution as usize);

        for _ in 0..number_execution {
            res.push(self.resolve());
        }

        res
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

#[test]
fn should_generate_hill_climber_solution() {
    let mut vec = vec![true, false, true, false];

    assert_eq!(
        vec![true, false, false, false],
        generate_hill_climber_solution(&mut vec, 2)
    );
    assert_eq!(
        vec![false, false, true, false],
        generate_hill_climber_solution(&mut vec, 0)
    );
}

