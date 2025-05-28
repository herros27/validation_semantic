// src/bindings/python/python_bindings.rs

#![cfg(feature = "python_bindings_feature")] // Hanya kompilasi jika fitur diaktifkan

use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::core_logic::{
    SupportedModel, APP_CONTEXT, validate_input_with_llm_sync
};

#[pyfunction]
fn validate_text_py(py: Python, text: String, model_selector_int: i32) -> PyResult<PyObject> {
    let context = match &*APP_CONTEXT {
        Ok(ctx) => ctx,
        Err(e) => return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("AppContext init error: {}", e))),
    };

    let model_variant = match SupportedModel::from_int(model_selector_int) {
        Some(m) => m,
        None => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            format!("Invalid model selector: {}. Valid options: [{}]", model_selector_int, SupportedModel::valid_options_desc())
        )),
    };
    let model_name = model_variant.as_str();

    match validate_input_with_llm_sync(&text, model_name, context) {
        Ok(validation_response) => {
            let dict = PyDict::new(py);
            dict.set_item("valid", validation_response.valid)?;
            dict.set_item("message", validation_response.message)?;
            Ok(dict.into())
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyException, _>(format!("Validation error with model '{}': {}", model_name, e))),
    }
}

// Fungsi ini akan dipanggil dari lib.rs untuk mendaftarkan modul Python
pub fn register_items_for_python_module(_py: Python, parent_module: &Bound<PyModule>) -> PyResult<()> {
    // Cara yang lebih umum untuk menambahkan fungsi yang di-wrap:
    parent_module.add_wrapped(wrap_pyfunction!(validate_text_py))?;
    // ATAU, jika Anda ingin menentukan modul secara eksplisit di wrap_pyfunction (meskipun sudah jelas dari parent_module):
    // parent_module.add_function(wrap_pyfunction!(validate_text_py, parent_module)?)?; 
    // Yang di atas (add_wrapped) lebih disarankan jika Anda sudah memiliki referensi ke modul.

    // Tambahkan konstanta model ke modul Python
    parent_module.add("MODEL_GEMINI_2_FLASH", SupportedModel::Gemini2Flash as i32)?;
    parent_module.add("MODEL_GEMINI_2_FLASH_LITE", SupportedModel::Gemini2FlashLite as i32)?;
    parent_module.add("MODEL_GEMINI_1_5_FLASH", SupportedModel::Gemini15Flash as i32)?;
    parent_module.add("MODEL_GEMINI_1_5_PRO", SupportedModel::Gemini15Pro as i32)?;
    Ok(())
}