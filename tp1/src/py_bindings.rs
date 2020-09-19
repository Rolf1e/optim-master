use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::resolve;

#[pyfunction]
fn execute(number_execution: i32) -> PyResult<(f32, Vec<bool>, f32, i32)> {
    Ok(resolve::random_execution("data.txt", number_execution))
}

#[pyfunction]
fn execute_multiple_time(number_execution: i32) -> PyResult<Vec<(f32, Vec<bool>, f32, i32)>> {
    let mut result : Vec<_> = Vec::new();

    for attempt in 0..number_execution {
        result.push(resolve::random_execution("data1000.txt", attempt));
    }

    Ok(result)
}


#[pymodule]
fn knapsack(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(execute))?;
    m.add_wrapped(wrap_pyfunction!(execute_multiple_time))?;
    Ok(())
}


