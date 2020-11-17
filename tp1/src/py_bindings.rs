use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::executor::*;

const FILE: &str = "data1000.txt";

#[pyfunction]
fn resolve_random_multithread(
    number_execution: i32,
    iterations: i32,
) -> PyResult<(Vec<(f32, f32, i32)>, u128)> {
    Ok(two_read_random(FILE, number_execution, iterations))
}

#[pyfunction]
fn resolve_walk_multithread(
    number_execution: i32,
) -> PyResult<ThreadSolution> {
    Ok(two_read_walk(FILE, number_execution))
}

#[pyfunction]
fn resolve_hill_climber_multithread(
    number_execution: i32,
) -> PyResult<ThreadSolution> {
    Ok(two_read_hill_climber(FILE, number_execution))
}

#[pymodule]
fn knapsack(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(resolve_random_multithread))?;
    m.add_wrapped(wrap_pyfunction!(resolve_walk_multithread))?;
    m.add_wrapped(wrap_pyfunction!(resolve_hill_climber_multithread))?;
    Ok(())
}
