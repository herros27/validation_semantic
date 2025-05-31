#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

// Impor tipe dan fungsi async dari core_logic
use crate::core_logic::{
    ValidationResponse,
    SupportedModel,
    validate_input_with_llm_async, // Sudah tidak perlu ApiConfig
};

// Panic hook opsional
#[wasm_bindgen]
pub fn init_panic_hook() {
    #[cfg(feature = "console_error_panic_hook_feature")] 
    console_error_panic_hook::set_once();
}

// Fungsi utama untuk validasi teks, dipanggil dari JavaScript
#[wasm_bindgen(js_name = validateTextJs)]
pub async fn validate_text_js(
    text: String,
    model_selector_int: i32,
    input_type: String,
) -> Result<JsValue, JsValue> {

    // Mapping angka ke enum model
    let model_variant = match SupportedModel::from_int(model_selector_int) {
        Some(m) => m,
        None => return Err(JsValue::from_str(&format!(
            "Invalid model selector: {}. Valid options: [{}]",
            model_selector_int,
            SupportedModel::valid_options_desc()
        ))),
    };
    let model_name = model_variant.as_str();

    // Panggil langsung fungsi core async tanpa config
    match validate_input_with_llm_async(&text, model_name, &input_type).await {
        Ok(validation_response_rust) => {
            match serde_wasm_bindgen::to_value(&validation_response_rust) {
                Ok(js_val) => Ok(js_val),
                Err(e) => Err(JsValue::from_str(&format!(
                    "Failed to serialize response to JsValue: {}", e
                ))),
            }
        }
        Err(err) => Err(JsValue::from_str(&err.to_string())),
    }
}

// Getter konstanta model
#[wasm_bindgen]
pub fn get_model_gemini_2_flash_lite_selector() -> i32 {
    SupportedModel::Gemini2FlashLite as i32
}

#[wasm_bindgen]
pub fn get_model_gemini_2_flash_selector() -> i32 {
    SupportedModel::Gemini2Flash as i32
}

#[wasm_bindgen]
pub fn get_model_gemini_1_5_flash_selector() -> i32 {
    SupportedModel::Gemini15Flash as i32
}

#[wasm_bindgen]
pub fn get_model_gemini_1_5_pro_selector() -> i32 {
    SupportedModel::Gemini15Pro as i32
}
