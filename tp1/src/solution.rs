use optim::knapsack::Knapsack;
use optim::solution::Solution;

pub struct KnapsackSolution<'a> {
    fitness: f32, //total bag's weight
    knapsack: &'a Knapsack,
}

impl<'a> KnapsackSolution<'a> {
    pub fn new(knapsack: &'a Knapsack, fitness: f32) -> Self {
        KnapsackSolution { knapsack, fitness }
    }

    pub fn get_fitness(&self) -> f32 {
        self.fitness
    }
}

impl<'a> Solution<'a, f32> for KnapsackSolution<'a> {
    fn evaluate(&self, choosed_items: &[bool]) -> f32 {
        evaluate_solution(
            self.knapsack,
            &choosed_items,
            &self.fitness,
            &self.knapsack.sum_weight(choosed_items),
        )
    }
}

pub fn evaluate_solution(
    knapsack: &Knapsack,
    choosed_items: &[bool],
    fitness: &f32,
    sum_weight: &f32,
) -> f32 {
    let sum_profit = knapsack.sum_profit(choosed_items);

    if sum_weight <= fitness {
        return sum_profit;
    }

    sum_profit - knapsack.get_beta() * (sum_weight - fitness)
}

#[test]
fn should_evaluate_solution() {
    use optim::knapsack::Item;

    let mut items = Vec::new();
    items.push(Item::new(String::from("Grelloc"), 70.0, 5.0));
    items.push(Item::new(String::from("Nekoshiro"), 45.0, 2.0));
    items.push(Item::new(String::from("Rolfie"), 80.0, 6.0));
    items.push(Item::new(String::from("Ephrimes"), 60.0, 4.0));

    let sack = Knapsack::new(0.0, items);
    let choosed_items = vec![false, true, true, false];
    assert_eq!(
        8.0,
        evaluate_solution(
            &sack,
            &choosed_items,
            &200.0,
            &sack.sum_weight(&choosed_items)
        )
    );

    let choosed_items = vec![true, true, false, true];
    assert_eq!(
        11.0,
        evaluate_solution(
            &sack,
            &choosed_items,
            &200.0,
            &sack.sum_weight(&choosed_items)
        )
    );
}

