pub mod knapsack;

use knapsack::Knapsack;

pub trait Solution<'a, Y> {

    fn evaluate(&self, knapsack: &Knapsack, sum_weight: &f32) -> Y; 

}

