use colorthief::{get_pallete, Color};
use pyo3::prelude::*;

/// Returns the color pallate of a given image
#[pyfunction]
fn sum_as_string(src: String) -> PyResult<Vec<i32>> {
    
}

/// A Python module implemented in Rust.
#[pymodule]
fn python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
