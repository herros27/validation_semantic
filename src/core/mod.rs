pub mod core_logic;

pub use core_logic::{
    validate_input_with_llm_async, 

    common_body_generation, 
    format_prompt, 
    parse_gemini_response, 
    pre_validate_syntactically
};


#[cfg(all(not(target_arch = "wasm32"), feature = "native_ffi_setup"))]
pub use core_logic::validate_input_with_llm_sync;