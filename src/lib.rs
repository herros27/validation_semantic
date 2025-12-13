// src/lib.rs
#[cfg(not(target_arch = "wasm32"))]
uniffi::setup_scaffolding!("validation_semantic");

// Deklarasikan modul-modul top-level
pub mod core;
pub mod models;

pub mod bindings; // Ini akan memuat src/bindings/mod.rs
pub mod config;

pub mod utils;

#[cfg(feature = "python_bindings_feature")]
pub mod python_entry;

#[cfg(feature = "native_ffi_setup")]
pub use crate::bindings::c_ffi::{free_rust_string, validate_text_ffi};
// Dan juga re-export tipe yang mungkin dibutuhkan oleh main.rs untuk pengujian
// #[cfg(feature = "native_ffi_setup")]
// pub use crate::core_logic::{SupportedModel, ValidationResponse};
