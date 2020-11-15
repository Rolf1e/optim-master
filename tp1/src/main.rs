#![feature(test)]
pub mod hill_climber_resolve;
pub mod parsing;
pub mod py_bindings;
pub mod random_resolve;
pub mod solution;
pub mod walk_resolve;

use std::sync::mpsc;
use std::thread;
use std::time::Instant;

use crate::hill_climber_resolve::*;
use crate::random_resolve::*;
use crate::walk_resolve::*;
use optim::knapsack::Knapsack;
use optim::resolver::*;

fn main() {
    let now = Instant::now();
    let file_name = "data1000.txt";

    for _ in 1..20 {
        println!("Random");
        let res1 = one_thread_random(file_name, 10, 1000);
        println!("{}", res1);
        //let t1 = now.elapsed().as_millis();
        //println!("One thread: time {} ms", t1);
        //two_read_random(file_name, 10, 100000);
        //let t2 = now.elapsed().as_millis() - t1;
        //println!("Tow threads with two reads: time {} ms", t2);

        //println!("Walk");
        //one_thread_walk(file_name, 10, 100000);
        //let t3 = now.elapsed().as_millis() - t1 - t2;
        //println!("One thread: time {} ms", t3);
        //two_read_walk(file_name, 10, 100000);
        //let t4 = now.elapsed().as_millis() - t1 - t2 - t3;
        //println!("Tow threads with two reads: time {} ms", t4);

        println!("Hill climber best improvement");
        let res5 = one_thread_hill_climber(file_name);
        println!("{}", res5.2);
    }
}

fn one_thread_random(file_name: &str, number_execution: i32, iterations: i32) -> f32 {
    println!("execute on  {} iterations", number_execution * iterations);
    let mut res: Vec<random_resolve::BestSolution> = Vec::with_capacity(number_execution as usize);
    let file_content = parsing::create_knapsack_from_file(file_name);
    res.append(&mut random_resolve_for_threads(
        number_execution,
        iterations,
        file_content.0,
        file_content.1,
        file_content.2,
    ));

    res.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let (profit, _sol, _weight, _attempt) = res.first().ok_or(0).unwrap();

    *profit
}

fn one_thread_walk(file_name: &str, number_execution: i32, iterations: i32) {
    println!("execute on {} iterations", number_execution * iterations);
    let mut res: Vec<walk_resolve::BestSolution> = Vec::with_capacity(number_execution as usize);
    let file_content = parsing::create_knapsack_from_file(file_name);
    res.append(&mut walk_resolve_for_threads(
        number_execution,
        file_content.0,
        file_content.1,
        file_content.2,
    ));
}

fn one_thread_hill_climber(file_name: &str) -> hill_climber_resolve::BestSolution {
    let file_content = parsing::create_knapsack_from_file(file_name);
    hill_climber_resolver_for_threads(file_content.0, file_content.1, file_content.2)
}

fn two_read_random(file_name: &'static str, number_execution: i32, iterations: i32) {
    println!("execute on  {} iterations", number_execution * iterations);
    let mut res: Vec<random_resolve::BestSolution> = Vec::with_capacity(number_execution as usize);

    let (tx, rx) = mpsc::channel();

    let half = (number_execution / 2) as i32;

    let handle = thread::spawn(move || {
        let half = half.clone();
        let mut res_sub: Vec<random_resolve::BestSolution> = Vec::with_capacity(half as usize);
        let file_content = parsing::create_knapsack_from_file(file_name);

        res_sub.append(&mut random_resolve_for_threads(
            half,
            iterations,
            file_content.0,
            file_content.1,
            file_content.2,
        ));
        tx.send(res_sub).unwrap();
    });

    let file_content = parsing::create_knapsack_from_file(file_name);
    res.append(&mut random_resolve_for_threads(
        half,
        iterations,
        file_content.0,
        file_content.1,
        file_content.2,
    ));

    res.append(&mut rx.recv().unwrap());

    handle.join().unwrap();
}

fn two_read_walk(file_name: &'static str, number_execution: i32, iterations: i32) -> f32 {
    println!("execute on  {} iterations", number_execution * iterations);
    let mut res: Vec<walk_resolve::BestSolution> = Vec::with_capacity(number_execution as usize);

    let (tx, rx) = mpsc::channel();

    let half = (number_execution / 2) as i32;

    let handle = thread::spawn(move || {
        let half = half.clone();
        let mut res_sub: Vec<walk_resolve::BestSolution> = Vec::with_capacity(half as usize);
        let file_content = parsing::create_knapsack_from_file(file_name);

        res_sub.append(&mut walk_resolve_for_threads(
            half,
            file_content.0,
            file_content.1,
            file_content.2,
        ));
        tx.send(res_sub).unwrap();
    });

    let file_content = parsing::create_knapsack_from_file(file_name);
    res.append(&mut walk_resolve_for_threads(
        half,
        file_content.0,
        file_content.1,
        file_content.2,
    ));

    res.append(&mut rx.recv().unwrap());

    handle.join().unwrap();

    res.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    res.first().unwrap().0
}

fn random_resolve_for_threads(
    number_execution: i32,
    iterations: i32,
    knapsack: Knapsack,
    fitness: f32,
    length: usize,
) -> Vec<random_resolve::BestSolution> {
    RandomResolver::new(&knapsack, fitness, length, iterations).multiple_resolve(number_execution)
}

fn walk_resolve_for_threads(
    number_execution: i32,
    knapsack: Knapsack,
    fitness: f32,
    length: usize,
) -> Vec<walk_resolve::BestSolution> {
    WalkResolver::new(
        fitness,
        &knapsack,
        length,
        random_resolve::generate_solution(length),
    )
    .multiple_resolve(number_execution)
}

fn hill_climber_resolver_for_threads(
    knapsack: Knapsack,
    fitness: f32,
    length: usize,
) -> hill_climber_resolve::BestSolution {
    HillClimberResolver::new(&knapsack, length, fitness).resolve()
}
