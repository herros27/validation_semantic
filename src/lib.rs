#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "python")]
use pyo3::prelude::*;

/// Fungsi utama yang bisa digunakan di semua platform
#[allow(dead_code)]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn add_wasm(a: i32, b: i32) -> i32 {
    add(a, b)
}

#[cfg(feature = "python")]
#[pyfunction]
fn add_py(a: i32, b: i32) -> i32 {
    add(a, b)
}

#[cfg(feature = "python")]
#[pymodule]
fn validation_semantic(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add_py, m)?)?;
    Ok(())
}
