pub struct Knapsack {
    content : Vec<Item>,
}

pub struct Item {
    name: String,
    weight: f32,
    profit: f32,
}

impl Knapsack {
   
    pub fn new() -> Self {
        Knapsack {
            content : Vec::new(),
        } 
    }

    pub fn create(content :Vec<Item>) -> Self {
        Knapsack {
            content,
        } 
    }

    pub fn push(&mut self, item : Item) {
        self.content.push(item); 
    }

    pub fn pop(&mut self, item: Item) {
        self.content
            .retain(|x| x.name.eq(&item.name));
    }

    pub fn get_content(&self) -> &Vec<Item> {
        &self.content
    }

    pub fn sum_weight(&self, choosed_items :&[bool]) -> f32 {
        self.content
            .iter()
            .zip(choosed_items)
            .filter(|(_, &taken)| taken)
            .map(|(item, _)| item.get_weight())
            .sum::<f32>()
    }
}

impl Item {
    
    pub fn new(name: String, weight: f32, profit: f32) -> Self {
        Item { name, weight, profit,}
    }

    pub fn get_weight(&self) -> f32 {
       self.weight
    }

    pub fn get_profit(&self) -> f32 {
        self.profit
    }
}

