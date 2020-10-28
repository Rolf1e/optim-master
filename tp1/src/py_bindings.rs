use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use std::sync::mpsc;
use std::thread;

use crate::parsing;
use crate::random_resolve;
use crate::random_resolve::*;
use crate::walk_resolve;
use crate::walk_resolve::*;
use optim::knapsack::Knapsack;
use optim::resolver::*;


#[pyfunction]
fn resolve_random(
    number_execution: i32,
    iterations: i32,
) -> PyResult<Vec<random_resolve::BestSolution>> {
    Ok(naive_random_resolve(number_execution, iterations))
}

#[pyfunction]
fn resolve_random_multi_threaded(
    number_execution: i32,
    iterations: i32,
) -> PyResult<Vec<random_resolve::BestSolution>> {
    let mut res: Vec<random_resolve::BestSolution> = Vec::with_capacity(number_execution as usize);
    let half = (number_execution / 2) as i32;

    let (tx, rx) = mpsc::channel();


    let handle = thread::spawn(move || {
        let half = half.clone();
        let mut res_sub: Vec<random_resolve::BestSolution> = Vec::with_capacity(half as usize);
        let file_content = parsing::create_knapsack_from_file("data1000.txt");
        res_sub.append(&mut random_resolve_for_threads(half, iterations, file_content.0.clone(), file_content.1.clone(), file_content.2.clone()));
        tx.send(res_sub)
            .unwrap();
    });

    let file_content = parsing::create_knapsack_from_file("data1000.txt");
    res.append(&mut random_resolve_for_threads(half, iterations, file_content.0, file_content.1, file_content.2));

    res.append(&mut rx.recv().unwrap());

    handle.join().unwrap();

    Ok(res)
}

fn naive_random_resolve(number_execution: i32, iterations: i32) -> Vec<random_resolve::BestSolution> {
    let file_content = parsing::create_knapsack_from_file("data1000.txt");
    let mut resolver = RandomResolver::new(&file_content.0, file_content.1, file_content.2, iterations);

    resolver.multiple_resolve(number_execution)
}

fn random_resolve_for_threads(number_execution: i32, iterations: i32, knapsack: Knapsack, fitness: f32, length: usize) -> Vec<random_resolve::BestSolution> {
    let mut resolver = RandomResolver::new(&knapsack, fitness, length, iterations);

    resolver.multiple_resolve(number_execution)
}

#[pyfunction]
fn resolve_walk(number_execution: i32) -> PyResult<Vec<walk_resolve::BestSolution>> {
    let file_content = parsing::create_knapsack_from_file("data1000.txt");
    let generated_solution = random_resolve::generate_solution(file_content.2);
    let mut resolver = WalkResolver::new(
        file_content.1,
        &file_content.0,
        file_content.2,
        generated_solution,
    );

    Ok(resolver.multiple_resolve(number_execution))
}

#[pymodule]
fn knapsack(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(resolve_random))?;
    m.add_wrapped(wrap_pyfunction!(resolve_walk))?;
    m.add_wrapped(wrap_pyfunction!(resolve_random_multi_threaded))?;
    Ok(())
}
