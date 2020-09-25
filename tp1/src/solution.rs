use crate::knapsack::{Knapsack, Item};

pub struct Solution<'a> {
    choosed_items : &'a [bool], // bool vector of choosed items : True -> choosed False -> Non choosed 
    fitness : f32, //total bag's weight
}

impl<'a> Solution<'a> {

    pub fn new(choosed_items: &'a [bool], fitness: f32) -> Self {
        Solution {
            choosed_items, 
            fitness, 
        }
    }

    pub fn evaluate(&self, knapsack :&Knapsack, sum_weight : &f32) -> f32 {
        evaluate_solution(knapsack, self.choosed_items, &self.fitness, sum_weight)
    }

    pub fn display(&self) -> String {
        format!("Fitness : {}, solution -> {:?}", self.fitness, self.choosed_items)
    }
}


fn evaluate_solution(knapsack :&Knapsack, choosed_items: &[bool], fitness: &f32, sum_weight : &f32) -> f32 {
    if sum_weight <= fitness {
        return knapsack.sum_profit(choosed_items);
    }

    let beta = calculate_beta(knapsack.get_content(), choosed_items);

    knapsack.sum_profit(choosed_items) - beta * (sum_weight - fitness)
}

#[test]
fn should_evaluate_solution() {
    use crate::knapsack::Item;

    let mut sack = Knapsack::new();
    sack.push(Item::new(String::from("Grelloc"), 70.0, 5.0));
    sack.push(Item::new(String::from("Nekoshiro"), 45.0, 2.0));
    sack.push(Item::new(String::from("Rolfie"), 80.0, 6.0));
    sack.push(Item::new(String::from("Ephrimes"), 60.0, 4.0));

    let choosed_items = vec![false, true, true, false];
    assert_eq!(8.0, evaluate_solution(&sack, &choosed_items, &200.0, &sack.sum_weight(&choosed_items)));

    let choosed_items = vec![true, true, false, true];
    assert_eq!(11.0, evaluate_solution(&sack, &choosed_items, &200.0, &sack.sum_weight(&choosed_items)));
}

#[test]
fn should_display() {
    let expect = String::from("Fitness : 120, solution -> [true, false, true, true]");
    let choosed_items = vec![true, false, true, true];
    let solution = Solution::new(&choosed_items, 120.0);

    assert_eq!(expect, solution.display());
}

fn calculate_beta(items: &[Item], choosed_items: &[bool]) -> f32{
     items.
        iter()
        .zip(choosed_items)
        .filter(|(_, &taken)| taken)
        .map(|(item, _)| item.get_profit() / item.get_weight())
        .sum::<f32>()
}

#[test]
fn should_calculate_beta() {
    let choosed_items = vec![true, true, true];
    let items = vec![Item::new(String::from("1"), 20.0, 10.0), Item::new(String::from("2"), 20.0, 11.0), Item::new(String::from("3"), 20.0, 15.0)];
    assert_eq!(1.8, calculate_beta(&items, &choosed_items));    
}

