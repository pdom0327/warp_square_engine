use pyo3::prelude::*;
use ::warp_square_engine::get_hello;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn say_hello(nickname: String) -> PyResult<String> {
    Ok(get_hello(nickname))
}

/// A Python module implemented in Rust.
#[pymodule]
fn warp_square_engine(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(say_hello, m)?)?;
    Ok(())
}
