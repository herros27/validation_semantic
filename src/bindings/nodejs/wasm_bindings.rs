#![cfg(target_arch = "wasm32")]
use std::cell::RefCell;
use js_sys;
use wasm_bindgen::prelude::*; // Pastikan js_sys diimpor

// Impor tipe dan fungsi async dari core_logic
use crate::core_logic::{
    // ValidationResponse, // Tidak digunakan secara langsung di snippet ini
    SupportedModel,
    // validate_input_with_llm_async, // Tidak digunakan secara langsung di snippet ini
};


thread_local! {
    static CONFIG: RefCell<ModuleConfig> = RefCell::new(ModuleConfig::default());
}

#[derive(Default)]
pub struct ModuleConfig {
    pub api_key: Option<String>,
}

#[wasm_bindgen]
pub fn configure(api_key: String) {
    CONFIG.with(|c| {
        c.borrow_mut().api_key = Some(api_key);
    });
}


// Panic hook opsional
#[wasm_bindgen]
pub fn init_panic_hook() {
    #[cfg(feature = "console_error_panic_hook_feature")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen(js_name = validateInput)]
pub async fn validate_text_js(
    text: String,
    model: i32,
    label: String,
) -> Result<JsValue, JsValue> {
    // Mengembalikan Result<JsValue, JsValue> untuk error handling ke JS
    let api_key = CONFIG.with(|c| c.borrow().api_key.clone())
        .ok_or("API key not configured")?;
    // 1. Mapping angka ke enum model (seperti yang sudah Anda lakukan di React)
    let model_variant = match crate::core_logic::SupportedModel::from_int(model) {
        Some(m) => m,
        None => {
            let error_message = format!(
                "Invalid model selector: {}. Valid options: [{}]",
                model,
                crate::core_logic::SupportedModel::valid_options_desc()
            );
            return Err(JsValue::from_str(&error_message));
        }
    };
    let model_name = model_variant.as_str();

    // 2. Panggil fungsi inti dari core_logic
    // Perhatikan path ke validate_input_with_llm_async mungkin perlu disesuaikan
    // tergantung struktur modul Anda (misalnya, crate::core_logic::...)
    match crate::core_logic::validate_input_with_llm_async(&text, model_name, &label, &api_key).await {//ubah biar parameter nya API_KEY gemini
        Ok(validation_response_rust) => {
            // 3. Serialisasi hasil Rust (ValidationResponse) ke JsValue
            match serde_wasm_bindgen::to_value(&validation_response_rust) {
                Ok(js_val) => Ok(js_val),
                Err(e) => {
                    let error_message = format!("Failed to serialize response to JsValue: {}", e);
                    Err(JsValue::from_str(&error_message))
                }
            }
        }
        Err(err) => {
            // 4. Konversi error dari Rust ke JsValue
            Err(JsValue::from_str(&err.to_string()))
        }
    }
}

#[wasm_bindgen(js_name = getSupportedModelSelectors)]
pub fn get_supported_model_selectors() -> JsValue {
    let models = js_sys::Object::new();

    // Helper closure untuk menangani error set (opsional)
    let handle_set_error = |err: JsValue| {
        web_sys::console::error_2(
            &JsValue::from_str("Failed to set property on JS object:"),
            &err,
        );
        // Anda bisa memilih untuk panic atau tindakan lain di sini jika error ini kritikal
    };

    if let Err(e) = js_sys::Reflect::set(
        &models,
        &JsValue::from_str("GEMINI_FLASH"),
        &JsValue::from_f64(SupportedModel::GeminiFlash as i32 as f64),
    ) {
        handle_set_error(e);
    }

    if let Err(e) = js_sys::Reflect::set(
        &models,
        &JsValue::from_str("GEMINI_FLASH_LITE"),
        &JsValue::from_f64(SupportedModel::GeminiFlashLite as i32 as f64),
    ) {
        handle_set_error(e);
    }

    if let Err(e) = js_sys::Reflect::set(
        &models,
        &JsValue::from_str("GEMINI_FLASH_LATEST"),
        &JsValue::from_f64(SupportedModel::GeminiFlashLatest as i32 as f64),
    ) {
        handle_set_error(e);
    }

    if let Err(e) = js_sys::Reflect::set(
        &models,
        &JsValue::from_str("GEMMA"),
        &JsValue::from_f64(SupportedModel::Gemma as i32 as f64),
    ) {
        handle_set_error(e);
    }

    models.into()
}

#[wasm_bindgen(js_name = getSupportedModels)]
pub fn get_supported_models() -> JsValue {
    let models = js_sys::Object::new();

    let handle_set_error = |err: JsValue| {
        web_sys::console::error_1(&err);
    };

    let set = |name: &str, value: SupportedModel| {
        if let Err(e) = js_sys::Reflect::set(
            &models,
            &JsValue::from_str(name),
            &JsValue::from_f64(value as i32 as f64),
        ) {
            handle_set_error(e);
        }
    };

    set("GeminiFlash", SupportedModel::GeminiFlash);
    set("GeminiFlashLite", SupportedModel::GeminiFlashLite);
    set("GeminiFlashLatest", SupportedModel::GeminiFlashLatest);
    set("Gemma", SupportedModel::Gemma);

    models.into()
}

