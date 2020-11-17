use std::sync::mpsc;
use std::thread;
use std::time::Instant;

use crate::hill_climber_resolve::*;
use crate::random_resolve::*;
use crate::walk_resolve::*;

use crate::hill_climber_resolve;
use crate::parsing;
use crate::random_resolve;
use crate::walk_resolve;

use optim::knapsack::Knapsack;
use optim::resolver::*;

pub type ThreadSolution = (Vec<(f32, f32)>, u128);

pub fn one_thread_random(
    file_name: &str,
    number_execution: i32,
    iterations: i32,
) -> (f32, f32, u128) {
    let mut res: Vec<random_resolve::BestSolution> = Vec::with_capacity(number_execution as usize);
    let file_content = parsing::create_knapsack_from_file(file_name);

    let now = Instant::now();
    res.append(&mut random_resolve_for_threads(
        number_execution,
        iterations,
        file_content.0,
        file_content.1,
        file_content.2,
    ));

    let end = now.elapsed().as_millis();

    res.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let (profit, _sol, weight, _iteration) = res.last().ok_or(0).unwrap();

    (*profit, *weight, end)
}

pub fn one_thread_walk(file_name: &str, number_execution: i32) -> (f32, f32, u128) {
    let mut res: Vec<walk_resolve::BestSolution> = Vec::with_capacity(number_execution as usize);
    let file_content = parsing::create_knapsack_from_file(file_name);

    let now = Instant::now();
    res.append(&mut walk_resolve_for_threads(
        number_execution,
        file_content.0,
        file_content.1,
        file_content.2,
    ));

    let end = now.elapsed().as_millis();

    res.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let (profit, _sol, weight) = res.last().ok_or(0).unwrap();

    (*profit, *weight, end)
}

pub fn one_thread_hill_climber(file_name: &str, number_execution: i32) -> (f32, f32, u128) {
    let file_content = parsing::create_knapsack_from_file(file_name);

    let now = Instant::now();
    let mut res = hill_climber_resolver_for_threads(
        number_execution,
        file_content.0,
        file_content.1,
        file_content.2,
    );

    let end = now.elapsed().as_millis();

    res.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let (profit, _sol, weight) = res.last().ok_or(0).unwrap();

    (*profit, *weight, end)
}

pub fn two_read_random(
    file_name: &'static str,
    number_execution: i32,
    iterations: i32,
) -> (Vec<(f32, f32, i32)>, u128) {
    let mut res: Vec<random_resolve::BestSolution> = Vec::with_capacity(number_execution as usize);

    let (tx, rx) = mpsc::channel();

    let half = (number_execution / 2) as i32;

    let now = Instant::now();

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

    let end = now.elapsed().as_millis();

    res.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let parsed = res
        .iter()
        .map(|(profit, _sol, weight, iterations)| (*profit, *weight, *iterations))
        .collect();

    (parsed, end)
}

pub fn two_read_walk(file_name: &'static str, number_execution: i32) -> ThreadSolution {
    let mut res: Vec<walk_resolve::BestSolution> = Vec::with_capacity(number_execution as usize);

    let (tx, rx) = mpsc::channel();

    let half = (number_execution / 2) as i32;

    let now = Instant::now();
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

    let end = now.elapsed().as_millis();

    res.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let parsed = res
        .iter()
        .map(|(profit, _sol, weight)| (*profit, *weight))
        .collect();

    (parsed, end)
}

pub fn two_read_hill_climber(file_name: &'static str, number_execution: i32) -> ThreadSolution {
    let mut res: Vec<hill_climber_resolve::BestSolution> =
        Vec::with_capacity(number_execution as usize);

    let (tx, rx) = mpsc::channel();

    let half = (number_execution / 2) as i32;

    let now = Instant::now();
    let handle = thread::spawn(move || {
        let half = half.clone();
        let mut res_sub: Vec<hill_climber_resolve::BestSolution> =
            Vec::with_capacity(half as usize);
        let file_content = parsing::create_knapsack_from_file(file_name);

        res_sub.append(&mut hill_climber_resolver_for_threads(
            half,
            file_content.0,
            file_content.1,
            file_content.2,
        ));
        tx.send(res_sub).unwrap();
    });

    let file_content = parsing::create_knapsack_from_file(file_name);
    res.append(&mut hill_climber_resolver_for_threads(
        half,
        file_content.0,
        file_content.1,
        file_content.2,
    ));

    res.append(&mut rx.recv().unwrap());

    handle.join().unwrap();

    let end = now.elapsed().as_millis();

    res.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let parsed = res
        .iter()
        .map(|(profit, _sol, weight)| (*profit, *weight))
        .collect();

    (parsed, end)
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
    number_execution: i32,
    knapsack: Knapsack,
    fitness: f32,
    length: usize,
) -> Vec<hill_climber_resolve::BestSolution> {
    HillClimberResolver::new(&knapsack, length, fitness).multiple_resolve(number_execution)
}
