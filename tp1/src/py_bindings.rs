use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::parsing;
use crate::random_resolve;
use crate::random_resolve::*;
use crate::walk_resolve;
use crate::walk_resolve::*;
use optim::resolver::*;

#[pyfunction]
fn resolve_random(number_execution: i32) -> PyResult<Vec<random_resolve::BestSolution>> {
    let file_content = parsing::create_knapsack_from_file("data1000.txt");
    let mut resolver = RandomResolver::new(&file_content.0, file_content.1, file_content.2);

    Ok(resolver.multiple_resolve(number_execution))
}

#[pyfunction]
fn resolve_walk(number_execution: i32) -> PyResult<Vec<walk_resolve::BestSolution>> {
    let file_content = parsing::create_knapsack_from_file("data1000.txt");
    let generated_solution = random_resolve::generate_solution(file_content.2);
    let mut resolver = WalkResolver::new(
        file_content.1,
        &file_content.0,
        generated_solution.len(),
        generated_solution,
    );

    Ok(resolver.multiple_resolve(number_execution))
}

#[pymodule]
fn knapsack(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(resolve_random))?;
    m.add_wrapped(wrap_pyfunction!(resolve_walk))?;
    Ok(())
}
