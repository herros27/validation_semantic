// src/bindings/kotlin/uniffi.rs

use crate::core::{
   validate_input_with_llm_sync
};

use crate::config::ApiConfig;

use crate::models::{
    SupportedModel,
};
// 1. Definisikan ulang Enum/Struct agar UniFFI bisa membacanya
// (Kita melakukan wrapping agar tidak mengganggu core_logic yang dipakai WASM)
#[derive(uniffi::Object)] // Menandakan ini adalah Object/Class
pub struct SemanticValidator {
    config: ApiConfig, // Kita simpan config di sini
}
#[uniffi::export]
impl SemanticValidator {
    
    // Constructor (Dipanggil sekali saat inisialisasi)
    #[uniffi::constructor]
    pub fn new(api_key: String) -> Self {
        Self {
            config: ApiConfig { api_key },
        }
    }

    // Fungsi Validasi (TIDAK PERLU parameter api_key lagi!)
    pub fn validate_text(
        &self, // '&self' artinya mengakses data milik object ini
        text: String,
        model: ModelSelector,
        label: String
    ) -> Result<ResponseData, AppError> {
        
        let model_core: SupportedModel = model.into();

        // Kita pakai config yang disimpan di memory (self.config)
        let result = validate_input_with_llm_sync(
            &text, 
            model_core.as_str(), 
            &label, 
            &self.config // <--- Ambil dari memory struct
        ).map_err(|e| AppError::Generic { msg: e.to_string() })?;

        Ok(ResponseData {
            valid: result.valid,
            message: result.message,
        })
    }
}
#[derive(uniffi::Enum)]
pub enum ModelSelector {
    GeminiFlash,
    GeminiFlashLite,
    GeminiFlashLatest,
    Gemma,
}

// Helper untuk konversi dari ModelSelector (UniFFI) ke SupportedModel (Core Logic)
impl From<ModelSelector> for SupportedModel {
    fn from(val: ModelSelector) -> Self {
        match val {
            ModelSelector::GeminiFlash => SupportedModel::GeminiFlash,
            ModelSelector::GeminiFlashLite => SupportedModel::GeminiFlashLite,
            ModelSelector::GeminiFlashLatest => SupportedModel::GeminiFlashLatest,
            ModelSelector::Gemma => SupportedModel::Gemma,
        }
    }
}

#[derive(uniffi::Record)]
pub struct ResponseData {
    pub valid: bool,
    pub message: String,
}

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum AppError {
    #[error("{msg}")] // <--- INI PENTING! Ini format pesan errornya nanti
    Generic { msg: String },
}
// 2. Fungsi Export yang akan dipanggil Kotlin
// #[uniffi::export]
// pub fn validate_text_kotlin(
//     text: String,
//     model: ModelSelector,
//     label: String,
//     api_key: String,
// ) -> Result<ResponseData, AppError> {
    
//     // Kita panggil logika inti di sini (Synchronous wrapper)
//     // Jika core_logic kamu async, kamu butuh runtime (seperti tokio) di sini
//     // atau gunakan uniffi async support (lebih advanced).
    
//     // Contoh asumsi core_logic::validate_input_with_llm_sync ada:
//     let config = core_logic::ApiConfig { api_key: api_key };
//     let model_core: core_logic::SupportedModel = model.into();
    
//     // Panggil fungsi sync dari core_logic (atau block_on jika async)
//     let result = core_logic::validate_input_with_llm_sync(
//         &text, 
//         model_core.as_str(), 
//         &label, 
//         &config
//     ).map_err(|e| AppError::Generic { msg: e.to_string() })?;

//     Ok(ResponseData {
//         valid: result.valid,
//         message: result.message,
//     })
// }