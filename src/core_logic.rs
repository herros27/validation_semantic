// src/core_logic.rs

use serde::{Deserialize, Serialize};
use std::env;
use dotenv::dotenv;
use once_cell::sync::Lazy;

// Impor kedua jenis klien reqwest
#[cfg(feature = "native_ffi_setup")] // Atau fitur spesifik yang mengaktifkan 'reqwest/blocking'
use reqwest::blocking::Client as BlockingClient;


use reqwest::Client as AsyncClient;

// Struct ValidationResponse dan Enum SupportedModel tetap sama
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ValidationResponse {
    pub valid: bool,
    pub message: String,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SupportedModel {
    Gemini2Flash = 0, Gemini2FlashLite = 1, Gemini15Flash = 2, Gemini15Pro = 3,
}
impl SupportedModel { /* ... implementasi sama ... */
    pub fn as_str(&self) -> &'static str {
        match self {
            SupportedModel::Gemini2Flash => "gemini-2.0-flash",
            SupportedModel::Gemini2FlashLite => "gemini-2.0-flash-lite",
            SupportedModel::Gemini15Flash => "gemini-1.5-flash-latest",
            SupportedModel::Gemini15Pro => "gemini-1.5-pro-latest",
        }
    }
    pub fn from_int(value: i32) -> Option<Self> {
        match value {
            0 => Some(SupportedModel::Gemini2Flash),
            1 => Some(SupportedModel::Gemini2FlashLite),
            2 => Some(SupportedModel::Gemini15Flash),
            3 => Some(SupportedModel::Gemini15Pro),
            _ => None,
        }
    }
    pub fn valid_options_desc() -> String {
        format!("0 (Gemini2Flash), 1 (Gemini2FlashLite), 2 (Gemini15Flash), 3 (Gemini15Pro)")
    }
}


// Struct Gemini API tetap sama
#[derive(Debug, Deserialize)]
struct GeminiApiPart { text: String }
#[derive(Debug, Deserialize)]
struct GeminiApiContent { parts: Vec<GeminiApiPart> }
#[derive(Debug, Deserialize)]
struct GeminiApiResponseCandidate { content: GeminiApiContent }
#[derive(Debug, Deserialize)]
struct GeminiApiResponse { candidates: Vec<GeminiApiResponseCandidate> }

// Konteks Aplikasi sekarang hanya menyimpan API Key
// Klien HTTP akan dibuat di dalam fungsi yang membutuhkannya.
pub struct ApiConfig {
    pub api_key: String,
}

pub static API_CONFIG: Lazy<Result<ApiConfig, String>> = Lazy::new(|| {
    if dotenv().is_err() {
        eprintln!("[WARN] .env file not found or failed to load. GOOGLE_API_KEY must be set in environment.");
    }
    let api_key = env::var("GOOGLE_API_KEY")
        .map_err(|e| format!("[ApiConfig Init Error] GOOGLE_API_KEY not set: {}. Pastikan .env ada atau variabel lingkungan tersetting.", e))?;
    Ok(ApiConfig { api_key })
});

// --- Versi Sinkron ---
// Di dalam fungsi validate_input_with_llm_sync:
#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "native_ffi_setup")] 
pub fn validate_input_with_llm_sync(
    user_input: &str,
    model_name: &str,
    input_type_str: &str,
    config: &ApiConfig, // Menggunakan ApiConfig
) -> Result<ValidationResponse, Box<dyn std::error::Error  + Send + Sync>> {
    let client = BlockingClient::builder() // Buat klien sinkron di sini
        .timeout(std::time::Duration::from_secs(60))
        .build()?;

    let endpoint = format!( /* ... endpoint sama ... */
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model_name,
        config.api_key
    );
    let prompt = format_prompt(user_input, input_type_str); // Gunakan fungsi helper
    let body = común_body_generation(&prompt); // Gunakan fungsi helper

    let gemini_api_response: GeminiApiResponse = client // Gunakan client lokal
        .post(&endpoint)
        .json(&body)
        .send()?
        .error_for_status()?
        .json()?;

    parse_gemini_response(gemini_api_response) // Gunakan fungsi helper
}

// --- Versi Asinkron ---
pub async fn validate_input_with_llm_async(
    user_input: &str,
    model_name: &str,
    input_type_str: &str,
) -> Result<ValidationResponse, Box<dyn std::error::Error + Send + Sync>> {
    let client = {
        let builder = AsyncClient::builder();
    
        #[cfg(not(target_arch = "wasm32"))]
        let builder = builder.timeout(std::time::Duration::from_secs(60));
    
        builder
            .build()
            .map_err(|e| format!("Failed to build HTTP client: {}", e))?
    };
    const API_KEY: &str = "AIzaSyCWnm_TMUb9Zr3HVN_iQOss6zsMwxheoHw";

    let endpoint = format!( /* ... endpoint sama ... */
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model_name,
        API_KEY
    );
    let prompt = format_prompt(user_input, input_type_str); // Gunakan fungsi helper
    let body = común_body_generation(&prompt); // Gunakan fungsi helper

    let gemini_api_response: GeminiApiResponse = client // Gunakan client lokal
        .post(&endpoint)
        .json(&body)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    parse_gemini_response(gemini_api_response) // Gunakan fungsi helper
}

// --- Fungsi Helper untuk menghindari duplikasi kode ---
fn format_prompt(user_input: &str, input_type_str: &str) -> String {
    match input_type_str.to_lowercase().as_str() {
        "alamat email" | "email" => format!( /* ... prompt email ... */
            "Validasi alamat email berikut: \"{}\". \
             Pastikan formatnya benar, domainnya terlihat valid dan bukan domain contoh (seperti example.com). \
             Alamat email juga tidak boleh lebih dari 254 karakter. \
             Jawab dalam format JSON: {{ \"valid\": true|false, \"message\": \"penjelasan\" }}",
            user_input.replace("\"", "\\\"")
        ),
        "nama lengkap" | "nama" => format!( /* ... prompt nama ... */
            "Validasi nama lengkap berikut: \"{}\". \
             Nama lengkap seharusnya hanya mengandung huruf, spasi, dan mungkin tanda hubung atau apostrof tunggal. \
             Tidak boleh mengandung angka atau simbol aneh. Panjangnya wajar. \
             Jawab dalam format JSON: {{ \"valid\": true|false, \"message\": \"penjelasan\" }}",
            user_input.replace("\"", "\\\"")
        ),
        "nomor telepon indonesia" => format!( /* ... prompt nomor telepon ... */
             "Validasi nomor telepon berikut: \"{}\". \
              nomor telepon indonesia seharusnya hanya mengandung angka, dan di awali dengan +62 atau 08.\
              Tidak boleh mengandung simbol aneh atau huruf/ Panjangnya wajar.\
              Jawab dalam format JSON: {{ \"valid\": true|false, \"message\": \"penjelasan\" }}",
            user_input.replace("\"", "\\\"")
        ),
        _ => format!( /* ... prompt default ... */
            "Validasi input berikut dari user, yang merupakan sebuah **{}**: \"{}\".\n\n\
             Berikan penilaian apakah input tersebut valid atau tidak sebagai **{}** untuk penggunaan praktis. \
             Jika ini adalah alamat email, TIDAK BOLEH menggunakan domain yang dicadangkan untuk contoh atau dokumentasi (seperti example.com, example.net, example.org, example.edu, atau domain .test, .localhost, .invalid). \
             Jika tidak valid karena alasan ini atau alasan lain, berikan alasan dan saran perbaikan. \
             Jawab dalam format JSON yang ketat seperti ini (tanpa markdown atau teks tambahan di luar JSON): \
             {{ \"valid\": true|false, \"message\": \"penjelasan\" }}",
            input_type_str,
            user_input.replace("\"", "\\\""),
            input_type_str
        ),
    }
}

fn común_body_generation(prompt: &str) -> serde_json::Value {
    serde_json::json!({
        "contents": [ { "parts": [ { "text": prompt } ] } ],
        "safetySettings": [
          { "category": "HARM_CATEGORY_HARASSMENT", "threshold": "BLOCK_NONE" },
          { "category": "HARM_CATEGORY_HATE_SPEECH", "threshold": "BLOCK_NONE" },
          { "category": "HARM_CATEGORY_SEXUALLY_EXPLICIT", "threshold": "BLOCK_NONE" },
          { "category": "HARM_CATEGORY_DANGEROUS_CONTENT", "threshold": "BLOCK_NONE" }
        ],
        "generationConfig": { "responseMimeType": "application/json" }
    })
}

fn parse_gemini_response(
    gemini_api_response: GeminiApiResponse,
) -> Result<ValidationResponse, Box<dyn std::error::Error + Send + Sync>> {
    let model_generated_text_str: String = gemini_api_response
        .candidates
        .get(0)
        .and_then(|candidate| candidate.content.parts.get(0))
        .map(|part| part.text.clone())
        .ok_or_else(|| "Gagal mengekstrak teks dari respons Gemini.".to_string())?;

    let clean_json_str = model_generated_text_str.trim();

    // Pertama, parse sebagai Value untuk mendeteksi apakah array atau object
    let json_val: serde_json::Value = serde_json::from_str(clean_json_str).map_err(|e| {
        format!(
            "Gagal parse string ke JSON Value. Error: {}. Model output: '{}'",
            e, clean_json_str
        )
    })?;

    // Cek apakah ini array, jika iya ambil elemen pertama
    let json_obj = if let Some(array) = json_val.as_array() {
        array.get(0).cloned().ok_or("Model output berupa array kosong")?
    } else {
        json_val
    };

    // Lalu parse menjadi struct ValidationResponse
    let parsed: ValidationResponse = serde_json::from_value(json_obj).map_err(|e| {
        format!(
            "Gagal mem-parse JSON menjadi ValidationResponse. Error: {}. Model output: '{}'",
            e, clean_json_str
        )
    })?;

    Ok(parsed)
}
