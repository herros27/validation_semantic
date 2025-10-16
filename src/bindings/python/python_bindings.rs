#![cfg(feature = "python_bindings_feature")]

use crate::core_logic::{
    validate_input_with_llm_sync,
    SupportedModel as RustSupportedModel,
    API_CONFIG,
};
use pyo3::prelude::*;
use pyo3::types::PyDict;

// ----------------------------
// PyClass: SupportedModel
// ----------------------------
#[pyclass(name = "SupportedModel")]
#[derive(Clone, Copy, Debug)]
pub struct PySupportedModel {
    variant: RustSupportedModel,
}

#[allow(non_upper_case_globals)]
#[pymethods]
impl PySupportedModel {
    #[classattr]
    const GeminiFlash: Self = Self { variant: RustSupportedModel::GeminiFlash };
    #[classattr]
    const GeminiFlashLite: Self = Self { variant: RustSupportedModel::GeminiFlashLite };
    #[classattr]
    const GeminiFlashLatest: Self = Self { variant: RustSupportedModel::GeminiFlashLatest };
    #[classattr]
    const Gemma: Self = Self { variant: RustSupportedModel::Gemma };

    fn __repr__(&self) -> String {
        format!("<SupportedModel.{:?}>", self.variant)
    }

    #[getter]
    fn value(&self) -> i32 {
        self.variant as i32
    }
}

// ----------------------------
// Fungsi PyO3
// ----------------------------
#[pyfunction]
fn validate_input_py(
    py: Python,
    text: String,
    model_selector: &PySupportedModel,
    input_type: String,
) -> PyResult<PyObject> {
    let config = match &*API_CONFIG {
        Ok(cfg) => cfg,
        Err(e) => {
            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                format!("ApiConfig init error: {}", e),
            ))
        }
    };

    let model_name = model_selector.variant.as_str();
    match validate_input_with_llm_sync(&text, model_name, &input_type, config) {
        Ok(validation_response) => {
            let dict = PyDict::new(py);
            dict.set_item("valid", validation_response.valid)?;
            dict.set_item("message", validation_response.message)?;
            Ok(dict.into())
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyException, _>(
            format!("Validation error with model '{}': {}", model_name, e),
        )),
    }
}

// ----------------------------
// Registrasi ke modul Python
// ----------------------------
pub fn register_items_for_python_module(
    _py: Python,
    parent_module: &Bound<PyModule>,
) -> PyResult<()> {
    parent_module.add_wrapped(wrap_pyfunction!(validate_input_py))?;
    parent_module.add_class::<PySupportedModel>()?;

    parent_module.add("GEMINI_FLASH", RustSupportedModel::GeminiFlash as i32)?;
    parent_module.add("GEMINI_FLASH_LITE", RustSupportedModel::GeminiFlashLite as i32)?;
    parent_module.add("GEMINI_FLASH_LATEST", RustSupportedModel::GeminiFlashLatest as i32)?;
    parent_module.add("GEMMA", RustSupportedModel::Gemma as i32)?;
    Ok(())
}
