// src/bindings/c_ffi.rs
#![cfg(feature = "native_ffi_setup")] // Atau fitur yang lebih spesifik jika perlu
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use crate::core_logic::{
    ValidationResponse, SupportedModel, API_CONFIG, validate_input_with_llm_sync
};

/// Fungsi error handler internal untuk FFI C
fn handle_c_ffi_error(message: String) -> *mut c_char {
    let error_response = ValidationResponse {
        valid: false,
        message,
    };
    let json_error = serde_json::to_string(&error_response)
        .unwrap_or_else(|_| "{\"valid\":false,\"message\":\"Failed to serialize FFI error response\"}".to_string());
    CString::new(json_error).map_or_else(
        |_| CString::new("{\"valid\":false,\"message\":\"Critical FFI CString creation error\"}").unwrap().into_raw(),
        |cs| cs.into_raw()
    )
}

#[no_mangle]
pub extern "C" fn validate_text_ffi(
    text_ptr: *const c_char,
    model_selector: SupportedModel,
    input_type_ptr: *const c_char,
) -> *mut c_char {
    if text_ptr.is_null() {
        return handle_c_ffi_error("Input text pointer is null.".to_string());
    }

    let c_str_text = unsafe { CStr::from_ptr(text_ptr) };
    let text_input = match c_str_text.to_str() {
        Ok(s) => s,
        Err(_) => return handle_c_ffi_error("Invalid UTF-8 input string for text.".to_string()),
    };

    let model_name_to_use: &'static str;
    match SupportedModel::from_int(model_selector as i32) {
        Some(valid_model_variant) => {
            model_name_to_use = valid_model_variant.as_str();
        }
        None => {
            return handle_c_ffi_error(format!(
                "Invalid model selector value received: {}. Valid options are: [{}].",
                model_selector as i32,
                SupportedModel::valid_options_desc()
            ));
        }
    }

      // Validasi dan konversi input_type_ptr
    if input_type_ptr.is_null() {
        return handle_c_ffi_error("Input type pointer is null.".to_string());
    }
    let c_str_input_type = unsafe { CStr::from_ptr(input_type_ptr) };
    let input_type_str = match c_str_input_type.to_str() {
        Ok(s) => s,
        Err(_) => return handle_c_ffi_error("Invalid UTF-8 input string for input type.".to_string()),
    };

    match &*API_CONFIG {
        Ok(context_ref) => {
            match validate_input_with_llm_sync(text_input, model_name_to_use, input_type_str,context_ref) {
                Ok(res) => {
                    let json_res = serde_json::to_string(&res).unwrap_or_else(|_| "{\"valid\":false,\"message\":\"Failed to serialize successful validation response\"}".to_string());
                    CString::new(json_res).map_or_else(
                        |e| handle_c_ffi_error(format!("Failed to create CString from JSON result: {}", e)),
                        |cs| cs.into_raw(),
                    )
                }
                Err(e) => handle_c_ffi_error(format!("Error during validation with model '{}': {}", model_name_to_use, e)),
            }
        }
        Err(init_err_msg) => handle_c_ffi_error(format!("AppContext initialization failed: {}", init_err_msg)),
    }
}

#[no_mangle]
pub extern "C" fn free_rust_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}