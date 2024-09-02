use std::fs::File;

use pyo3::exceptions::PyFileNotFoundError;
use pyo3::prelude::*;

/// Prints a message.
#[pyfunction]
fn hello() -> PyResult<String> {
    Ok("Hello from rust-python-test!".into())
}

#[pyfunction]
fn lst_to_crd_rs(fname: &str) -> PyResult<()> {
    println!("Trying to open file {}", fname);

    let _f = File::open(fname)?;

    println!("File {} opened successfully", fname);
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn _lowlevel(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    m.add_function(wrap_pyfunction!(lst_to_crd_rs, m)?)?;
    Ok(())
}
