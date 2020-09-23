use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use crate::parsing;

use crate::resolve;

#[pyfunction]
fn execute(number_execution: i32) -> PyResult<(f32, Vec<bool>, f32, i32)> {
    let file_content = parsing::create_knapsack_from_file("data1000.txt");
    Ok(resolve::random_execution(&file_content.0, number_execution, file_content.1, file_content.2))
}

#[pyfunction]
//execute algo multiple times
fn execute_multiple_time(number_execution: i32) -> PyResult<Vec<(f32, Vec<bool>, f32, i32)>> {
    let file_content = parsing::create_knapsack_from_file("data1000.txt");
    let mut result : Vec<_> = Vec::new();

    //No need to start at 0
    for _ in 1..number_execution {
        result.push(resolve::random_execution(&file_content.0, 100000, file_content.1, file_content.2));
    }

    //TODO improve sorting
    result.sort_by(|a, b| a.3.partial_cmp(&b.3).unwrap());

    Ok(result)
}

#[pyfunction]
//execute algo multiple times with incremented attempts
fn execute_multiple_time_incremented(number_execution: i32) -> PyResult<Vec<(f32, Vec<bool>, f32, i32)>> {
    let file_content = parsing::create_knapsack_from_file("data1000.txt");
    let mut result : Vec<_> = Vec::new();

    //No need to start at 0
    for attempt in 1..number_execution {
        result.push(resolve::random_execution(&file_content.0, attempt, file_content.1, file_content.2));
    }

    //TODO improve sorting
    result.sort_by(|a, b| a.3.partial_cmp(&b.3).unwrap());

    Ok(result)
}


#[pymodule]
fn knapsack(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(execute))?;
    m.add_wrapped(wrap_pyfunction!(execute_multiple_time))?;
    m.add_wrapped(wrap_pyfunction!(execute_multiple_time_incremented))?;
    Ok(())
}


