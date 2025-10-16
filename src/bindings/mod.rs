// src/bindings/mod.rs

// Hanya sertakan modul python jika fitur python_bindings_feature aktif
#[cfg(feature = "python_bindings_feature")]
pub mod python;

// Hanya sertakan modul c_ffi jika fitur native_ffi_setup aktif
// Ini PENTING karena main.rs Anda mengandalkan fitur ini (via default)
#[cfg(feature = "native_ffi_setup")]
pub mod c_ffi;

// Hanya sertakan modul node (untuk Wasm) jika fitur wasm_bindings_setup aktif
// ATAU jika target arsitekturnya adalah wasm32
#[cfg(any(feature = "wasm_bindings_setup", target_arch = "wasm32"))]
pub mod nodejs;
