// src/lib.rs

// Deklarasikan modul-modul top-level
pub mod core_logic;
pub mod c_ffi;
pub mod bindings; // Ini akan memuat src/bindings/mod.rs

// Hanya sertakan dan proses binding Python jika fitur diaktifkan
#[cfg(feature = "python_bindings_feature")]
use pyo3::prelude::*;

#[cfg(feature = "python_bindings_feature")]
#[pymodule]
fn validation_semantic(_py: Python, m: &Bound<PyModule>) -> PyResult<()> { // <<<< UBAH NAMA FUNGSI DI SINI
    crate::bindings::python::python_bindings::register_items_for_python_module(_py, m)?;
    Ok(())
}
// Opsional: Jika Anda ingin main.rs bisa memanggil fungsi C FFI
// dengan `validation_semantic_lib::nama_fungsi`, Anda bisa re-export di sini.
// Namun, main.rs biasanya akan link ke rlib dan bisa `use validation_semantic_lib::c_ffi::*`.
// Jadi, re-export eksplisit mungkin tidak selalu diperlukan untuk main.rs jika ia menggunakan
// `use validation_semantic_lib::c_ffi::{validate_text_ffi, free_rust_string};`
// Tapi agar lebih mudah bagi main.rs jika tidak mau `use` path panjang:
pub use c_ffi::{validate_text_ffi, free_rust_string};
// Dan juga re-export tipe yang mungkin dibutuhkan oleh main.rs untuk pengujian
pub use core_logic::{ValidationResponse, SupportedModel};