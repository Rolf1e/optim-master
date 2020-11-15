#[derive(Debug, Clone)]
pub struct Knapsack {
    beta: f32,
    content: Vec<Item>,
}

#[derive(Debug, Clone)]
pub struct Item {
    name: String,
    weight: f32,
    profit: f32,
}

impl Knapsack {
    pub fn new(beta: f32, content: Vec<Item>) -> Self {
        Knapsack { beta, content }
    }

    pub fn get_content(&self) -> &Vec<Item> {
        &self.content
    }

    pub fn get_beta(&self) -> &f32 {
        &self.beta
    }

    //W(x)
    pub fn sum_weight(&self, choosed_items: &[bool]) -> f32 {
        self.content
            .iter()
            .zip(choosed_items)
            .filter(|(_, &taken)| taken)
            .map(|(item, _)| item.get_weight())
            .sum::<f32>()
    }

    //P(x)
    pub fn sum_profit(&self, choosed_items: &[bool]) -> f32 {
        self.content
            .iter()
            .zip(choosed_items)
            .filter(|(_, &taken)| taken)
            .map(|(item, _)| item.get_profit())
            .sum::<f32>()
    }
}

impl Item {
    pub fn new(name: String, weight: f32, profit: f32) -> Self {
        Item {
            name,
            weight,
            profit,
        }
    }

    pub fn get_weight(&self) -> f32 {
        self.weight
    }

    pub fn get_profit(&self) -> f32 {
        self.profit
    }
}
