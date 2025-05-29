// src/bindings/python/python_bindings.rs
#![cfg(feature = "python_bindings_feature")]

use pyo3::prelude::*;
use pyo3::types::PyDict;
use crate::core_logic::{
 SupportedModel as RustSupportedModel, // Ganti nama impor agar tidak bentrok
    APP_CONTEXT, validate_input_with_llm_sync
};

// Buat wrapper class Python untuk enum Rust
#[pyclass(name = "SupportedModel")] // Nama kelas di Python
#[derive(Clone, Copy, Debug)]
struct PySupportedModel {
    variant: RustSupportedModel, // Simpan varian Rust enum di dalamnya
}

#[pymethods]
impl PySupportedModel {
    // Ekspos setiap varian enum sebagai static property dari kelas Python
    #[classattr]
    #[pyo3(name = "GEMINI_2_FLASH")] // Nama atribut di Python
    const GEMINI_2_FLASH: Self = Self { variant: RustSupportedModel::Gemini2Flash };

    #[classattr]
    #[pyo3(name = "GEMINI_2_FLASH_LITE")]
    const GEMINI_2_FLASH_LITE: Self = Self { variant: RustSupportedModel::Gemini2FlashLite };

    #[classattr]
    #[pyo3(name = "GEMINI_1_5_FLASH")]
    const GEMINI_1_5_FLASH: Self = Self { variant: RustSupportedModel::Gemini15Flash };

    #[classattr]
    #[pyo3(name = "GEMINI_1_5_PRO")]
    const GEMINI_1_5_PRO: Self = Self { variant: RustSupportedModel::Gemini15Pro };

    // Anda mungkin ingin menambahkan metode __int__ atau properti value
    // agar mudah mendapatkan nilai integernya jika diperlukan
    fn __int__(&self) -> i32 {
        self.variant as i32
    }

    fn __repr__(&self) -> String {
        format!("<SupportedModel.{:?}>", self.variant)
    }

    #[getter]
    fn value(&self) -> i32 {
        self.variant as i32
    }
}

#[pyfunction]
fn validate_text_py(py: Python, text: String, model_selector: &PySupportedModel, input_type: String) -> PyResult<PyObject> { // Terima &PySupportedModel
    let context = match &*APP_CONTEXT {
        Ok(ctx) => ctx,
        Err(e) => return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("AppContext init error: {}", e))),
    };

    // Tidak perlu lagi from_int jika kita percaya tipenya sudah benar dari Python
    // atau kita bisa tetap validasi berdasarkan nilai integernya.
    // Langsung gunakan varian Rust dari PySupportedModel
    let model_name = model_selector.variant.as_str();
    let model_selector_int = model_selector.variant as i32; // Jika masih perlu integer untuk logging atau error

    match validate_input_with_llm_sync(&text, model_name, &input_type, context) {
        Ok(validation_response) => {
            let dict = PyDict::new(py);
            dict.set_item("valid", validation_response.valid)?;
            dict.set_item("message", validation_response.message)?;
            Ok(dict.into())
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyException, _>(
            format!("Validation error with model '{}' (selector: {}): {}", model_name, model_selector_int, e)
        )),
    }
}

pub fn register_items_for_python_module(_py: Python, parent_module: &Bound<PyModule>) -> PyResult<()> {
    parent_module.add_wrapped(wrap_pyfunction!(validate_text_py))?;
    parent_module.add_class::<PySupportedModel>()?; // Daftarkan kelas PySupportedModel
    Ok(())
}