// src/python_entry.rs
use pyo3::prelude::*;

// Kita akses modul bindings lewat crate root
use crate::bindings;

#[pymodule]
fn validation_semantic(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    // Panggil fungsi register yang ada di bindings
    bindings::python::python_bindings::register_items_for_python_module(_py, m)?;
    Ok(())
}

