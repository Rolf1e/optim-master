use crate::knapsack::Item;

#[derive(Debug)]
pub enum ChainedList<T> {
    Cons(T, Box<ChainedList<T>>),
    Nil,
}



impl ChainedList<Item> {
    
   pub fn get(&self) -> Item {
        
   }

}

