use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;

use optim::knapsack::{Knapsack, Item};


// return (knapsack, fitness, nb_items)
pub fn create_knapsack_from_file(file_name: &str) -> (Knapsack, f32, usize) {
    let content = get_file_content(file_name);
    let content :Vec<_> = content
        .split('\n')
        .collect();

    let fitness = content
        .get(3)
        .unwrap_or_else(|| panic!("Failed to parse fitness"))
        .parse::<f32>();

    let nb_items = content
        .get(0)
        .unwrap_or_else(|| panic!("Failed to parse nb_items"))
        .parse::<usize>();

    (create_knapsack(&content), fitness.unwrap(), nb_items.unwrap())
}

pub fn create_knapsack(content :&[&str]) -> Knapsack {
    let items_profits = extract_from_vector(&content, 1);
    let items_weights = extract_from_vector(&content, 2);
    
    let items :Vec<Item> = items_weights
        .iter()
        .zip(items_profits)
        .map(|(weight, profit)| Item::new(weight.to_string(), *weight, profit))
        .collect();

    Knapsack::create(items)
}

#[test]
pub fn should_create_knapsack() {
   let extracted_content = create_knapsack_from_file("test2.txt"); 
   assert_eq!(100.0, extracted_content.1);
   assert_eq!(5, extracted_content.0.get_content().len());
}

fn extract_from_vector(content :&[&str], index :usize) -> Vec<f32> {
    let items_profits = content
        .get(index)
        .unwrap_or_else(|| panic!("Failed to parse items vector index: {}", index));
    parse_content(items_profits)
}

pub fn get_file_content(file_name :&str) -> String {
    fs::read_to_string(file_name)
        .unwrap_or_else(|_| panic!("Failed to read file {} ", file_name))
}

#[test]
fn should_read() {
    assert_eq!("20 10 40 70 5\n", get_file_content("test.txt"));
}

pub fn parse_content(input: &str) -> Vec<f32> {
    let x = input
        .split(' ')
        .collect();
   get_as_u32(x) 
}

#[test]
fn should_get_str_as_f32() {
    assert_eq!(vec![20.0, 10.0, 40.0, 70.0, 5.0], parse_content("20 10 40 70 5"));
}

pub fn get_as_u32(input: Vec<&str>) -> Vec<f32> {
    input.iter()
        .map(|x| parse_f32(x))
        .collect()
}

#[test]
fn should_parse_to_f32() {
    assert_eq!(vec![20.0, 10.0, 40.0, 70.0, 5.0], get_as_u32(vec!["20", "10", "40", "70", "5"]));
}

fn parse_f32(string :&str) -> f32 {
   string.parse::<f32>()
       .unwrap_or_else(|_| panic!("Failed to parse {}", string))
}


#[test]
fn should_parse_f32() {
    assert_eq!(16.0, parse_f32("16"));
}

pub fn write_into_file(file_name: &str, to_write: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_name)
        .unwrap();
    
    if let Err(e) = writeln!(file, "{}", to_write) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

