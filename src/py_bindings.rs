use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::resolve;

#[pyfunction]
fn random_execution(number_execution: i32) -> PyResult<Vec<(String, f32)>> {
    let mut result :Vec<(String, f32)> = Vec::new();
    for attempt in 0..number_execution {
        let random_execution = resolve::execute_random_research(attempt, "data1000.txt");
        result.push((format!("{}", random_execution.2), random_execution.1));
    }
    Ok(result)
}


#[pymodule]
fn knapsack(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(random_execution))?;
    Ok(())
}


