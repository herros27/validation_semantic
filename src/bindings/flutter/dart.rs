// src/bindings/flutter/api.rs

use crate::core::{
    validate_input_with_llm_sync,
};
use crate::config::ApiConfig;

use crate::models::{
    SupportedModel,
};
// ---------------------------------------------------------
// 1. OBJECT / CLASS (Stateful)
// ---------------------------------------------------------

// Di FRB, cukup definisikan struct public biasa.
// FRB otomatis mendeteksi ini sebagai Class di Dart.
pub struct SemanticValidatorFrb {
    config: ApiConfig,
}

impl SemanticValidatorFrb {
    
    // Constructor
    // FRB otomatis mengenali method bernama 'new' sebagai constructor di Dart.
    pub fn new(api_key: String) -> Self {
        Self {
            config: ApiConfig { api_key },
        }
    }

    // Method Validasi
    // CATATAN PENTING: 
    // FRB secara default akan menjalankan fungsi ini di Thread Pool (Worker).
    // Jadi di sisi Dart nanti hasilnya adalah 'Future', UI tidak akan macet.
    pub fn validate_text(
        &self, 
        text: String, 
        model: ModelSelectorFrb, 
        label: String
    ) -> anyhow::Result<ResponseDataFrb> {
        // Kita konversi enum FRB ke enum Core Logic
        let model_core: SupportedModel = model.into();

        // Panggil core logic (Sync)
        // Meskipun ini blocking di Rust, FRB membuatnya Async di Dart. Aman!
        let result = validate_input_with_llm_sync(
            &text, 
            model_core.as_str(), 
            &label, 
            &self.config
        ).map_err(|e| anyhow::anyhow!(e.to_string()))?; // Simplifikasi Error dengan Anyhow

        Ok(ResponseDataFrb {
            valid: result.valid,
            message: result.message,
        })
    }
}

// ---------------------------------------------------------
// 2. ENUM
// ---------------------------------------------------------

// Cukup public enum biasa
pub enum ModelSelectorFrb {
    GeminiFlash,
    GeminiFlashLite,
    GeminiFlashLatest,
    Gemma,
}

// Helper konversi ke Core Logic (sama seperti UniFFI)
impl From<ModelSelectorFrb> for SupportedModel {
    fn from(val: ModelSelectorFrb) -> Self {
        match val {
            ModelSelectorFrb::GeminiFlash => SupportedModel::GeminiFlash,
            ModelSelectorFrb::GeminiFlashLite => SupportedModel::GeminiFlashLite,
            ModelSelectorFrb::GeminiFlashLatest => SupportedModel::GeminiFlashLatest,
            ModelSelectorFrb::Gemma => SupportedModel::Gemma,
        }
    }
}

// ---------------------------------------------------------
// 3. STRUCT DATA (Return Type)
// ---------------------------------------------------------

// Cukup public struct biasa
pub struct ResponseDataFrb {
    pub valid: bool,
    pub message: String,
}