use std::env;
use once_cell::sync::Lazy;
use dotenv::dotenv;

// Refactored API Config Module
// Konfigurasi API untuk menyimpan API key
// pub struct ApiConfig {
//     pub api_key: String,
// }

// pub static API_CONFIG: Lazy<Result<ApiConfig, String>> = Lazy::new(|| {
//     if dotenv().is_err() {
//         eprintln!("[WARN] .env file not found or failed to load. GOOGLE_API_KEY must be set in environment.");
//     }
//     let api_key = env::var("GOOGLE_API_KEY")
//         .map_err(|e| format!("[ApiConfig Init Error] GOOGLE_API_KEY not set: {}. Pastikan .env ada atau variabel lingkungan tersetting.", e))?;
//     Ok(ApiConfig { api_key })
// });

// Green
// Konfigurasi API
pub struct ApiConfig {
    pub api_key: String,
}

pub static API_CONFIG: Lazy<Result<ApiConfig, String>> = Lazy::new(|| {
    if dotenv().is_err() {
        // Warning jika .env tidak ketemu (wajar di CI/CD atau production env variabel)
    }
    let api_key = env::var("GOOGLE_API_KEY")
        .map_err(|e| format!("GOOGLE_API_KEY not set: {}", e))?;
    Ok(ApiConfig { api_key })
});


// Red
// pub struct ApiConfig { pub api_key: String }
// pub static API_CONFIG: Lazy<Result<ApiConfig, String>> = Lazy::new(|| {
//     Ok(ApiConfig { api_key: "".to_string() })
// });
