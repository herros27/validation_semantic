// src/core_logic.rs

use serde::{Deserialize, Serialize};
use std::env;
use reqwest::blocking::Client;
use dotenv::dotenv;
use once_cell::sync::Lazy;

// Struct untuk Respons Validasi Akhir
#[derive(Debug, Serialize, Deserialize, Clone)] // Tambahkan Clone
pub struct ValidationResponse {
    pub valid: bool,
    pub message: String,
}

// Enum untuk Model yang Didukung
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SupportedModel {
    Gemini2Flash = 0,
    Gemini2FlashLite = 1,
    Gemini15Flash = 2,
    Gemini15Pro = 3,
}

impl SupportedModel {
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

    pub fn valid_options_desc() -> String { // Dibuat pub agar bisa diakses dari c_ffi.rs atau python_bindings.rs
        format!(
            "0 (Gemini2Flash), 1 (Gemini2FlashLite), 2 (Gemini15Flash), 3 (Gemini15Pro)"
        )
    }
}

// Struct untuk deserialisasi respons dari API Gemini
// Ini bisa tetap private untuk modul core_logic jika tidak digunakan di luar
#[derive(Debug, Deserialize)]
struct GeminiApiPart {
    text: String,
}
#[derive(Debug, Deserialize)]
struct GeminiApiContent {
    parts: Vec<GeminiApiPart>,
}
#[derive(Debug, Deserialize)]
struct GeminiApiResponseCandidate {
    content: GeminiApiContent,
}
#[derive(Debug, Deserialize)]
struct GeminiApiResponse {
    candidates: Vec<GeminiApiResponseCandidate>,
}

// Konteks Aplikasi
pub struct AppContext { // pub agar bisa diakses (misalnya oleh fungsi FFI jika diperlukan)
    pub api_key: String, // pub field
    pub client: Client,  // pub field
}

pub static APP_CONTEXT: Lazy<Result<AppContext, String>> = Lazy::new(|| {
    if dotenv().is_err() { // Gunakan dotenv() untuk menghindari ambiguitas
        eprintln!("[WARN] .env file not found or failed to load. GOOGLE_API_KEY must be set in environment.");
    }
    let api_key = env::var("GOOGLE_API_KEY")
        .map_err(|e| format!("[AppContext Init Error] GOOGLE_API_KEY not set: {}. Pastikan .env ada atau variabel lingkungan tersetting.", e))?;
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .map_err(|e| format!("[AppContext Init Error] Failed to create reqwest client: {}", e))?;
    Ok(AppContext { api_key, client })
});

// Logika Inti untuk Validasi dengan LLM
pub fn validate_input_with_llm_sync(
    user_input: &str,
    model_name: &str,
    context: &AppContext,
) -> Result<ValidationResponse, Box<dyn std::error::Error>> {
    let endpoint = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model_name,
        context.api_key
    );

    let prompt = format!(
        "Validasi input berikut dari user: \"{}\".\n\n\
         Berikan penilaian apakah input valid atau tidak. Jika tidak valid, berikan alasan dan saran perbaikan. \
         Jawab dalam format JSON yang ketat seperti ini (tanpa markdown atau teks tambahan di luar JSON): \
         {{ \"valid\": true|false, \"message\": \"penjelasan\" }}",
        user_input.replace("\"", "\\\"")
    );

    let body = serde_json::json!({
        "contents": [ { "parts": [ { "text": prompt } ] } ],
        "safetySettings": [
          { "category": "HARM_CATEGORY_HARASSMENT", "threshold": "BLOCK_NONE" },
          { "category": "HARM_CATEGORY_HATE_SPEECH", "threshold": "BLOCK_NONE" },
          { "category": "HARM_CATEGORY_SEXUALLY_EXPLICIT", "threshold": "BLOCK_NONE" },
          { "category": "HARM_CATEGORY_DANGEROUS_CONTENT", "threshold": "BLOCK_NONE" }
        ],
        "generationConfig": { "responseMimeType": "application/json" }
    });

    let gemini_api_response: GeminiApiResponse = context
        .client
        .post(&endpoint)
        .json(&body)
        .send()?
        .error_for_status()?
        .json()?;

    let model_generated_text = gemini_api_response
        .candidates
        .get(0)
        .and_then(|candidate| candidate.content.parts.get(0))
        .map(|part| part.text.as_str())
        .ok_or_else(|| "Gagal mengekstrak teks dari respons Gemini.".to_string())?;

    let clean_json_str = model_generated_text.trim();

    match serde_json::from_str::<ValidationResponse>(clean_json_str) {
        Ok(parsed) => Ok(parsed),
        Err(e) => Err(format!(
            "Gagal mem-parse JSON dari model. Error: {}. Model output: '{}'",
            e, model_generated_text
        ).into()),
    }
}