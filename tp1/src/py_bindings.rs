use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::resolve;

#[pyfunction]
fn random_execution(number_execution: i32) -> PyResult<Vec<(i32, f32)>> {
    let mut result :Vec<(i32, f32)> = Vec::new();
    for attempt in 0..number_execution {
        let random_execution = resolve::execute_random_research("data1000.txt");
        println!("{} - Attempt {} is best with {} ", attempt, random_execution.2, random_execution.1);
        result.push((random_execution.2, random_execution.1));
    }

    result.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    Ok(result)
}

#[pyfunction]
fn execute(number_execution: i32) -> PyResult<Vec<(i32, f32)>> {
    let mut result :Vec<(i32, f32)> = Vec::new();
    for attempt in 0 .. number_execution {
        let execution = resolve::execute("data1000.txt");
        result.push((attempt, execution.1));
    }
    result.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    Ok(result)
}


#[pymodule]
fn knapsack(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(random_execution))?;
    m.add_wrapped(wrap_pyfunction!(execute))?;
    Ok(())
}


