// src/core_logic.rs

use serde::{Deserialize, Serialize};
use std::env;
use dotenv::dotenv;
use once_cell::sync::Lazy;
use regex::Regex;
// Impor kedua jenis klien reqwest
#[cfg(feature = "native_ffi_setup")] // Atau fitur spesifik yang mengaktifkan 'reqwest/blocking'
use reqwest::blocking::Client as BlockingClient;

use reqwest::StatusCode;
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
      // Tahap 1: Validasi Sintaksis Lokal
      if let Err(syntax_error_message) = pre_validate_syntactically(user_input, input_type_str) {
        return Ok(ValidationResponse {
            valid: false,
            message: syntax_error_message,
        });
    }

    // Tahap 2: Validasi Semantik dengan LLM (jika sintaksis OK)
    println!("[DEBUG] Sintaksis OK untuk '{}' ({}), melanjutkan ke validasi LLM.", user_input, input_type_str);
    let client = BlockingClient::builder() // Buat klien sinkron di sini
        .timeout(std::time::Duration::from_secs(60))
        .build()?;

    let endpoint = format!( /* ... endpoint sama ... */
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model_name,
        config.api_key
    );
    let prompt = format_prompt(user_input, input_type_str);
    let body = común_body_generation(&prompt);

    // Kirim permintaan dan dapatkan responsnya
    let response = client.post(&endpoint).json(&body).send()?;
    let status = response.status();

    if status.is_success() {
        // Jika status sukses (2xx), proses seperti biasa
        let gemini_api_response: GeminiApiResponse = response.json()?;
        parse_gemini_response(gemini_api_response)
    } else {
        // Jika status adalah error (4xx atau 5xx)
        // Baca body error sebagai teks untuk informasi tambahan
        // let error_body_text = response.text().unwrap_or_else(|e| {
        //     format!("Tidak dapat membaca body respons error dari server (Status: {}). Error internal saat membaca body: {}", status, e)
        // });

        if status == StatusCode::TOO_MANY_REQUESTS { // Kode status 429
            Err(format!(
                "Model '{}' tidak dapat digunakan saat ini karena telah mencapai batas penggunaan (limit).",
                model_name
            ).into()) // .into() akan mengonversi String menjadi Box<dyn Error...>
        } else {
            // Untuk error HTTP lainnya (4xx selain 429, atau 5xx)
            Err(format!(
                "Gagal menggunakan model '{}'. Server merespons dengan kode: {}.",
                model_name, status
            ).into())
        }
    }
}

// --- Versi Asinkron ---
pub async fn validate_input_with_llm_async(
    user_input: &str,
    model_name: &str,
    input_type_str: &str,
) -> Result<ValidationResponse, Box<dyn std::error::Error + Send + Sync>> {
      // Tahap 1: Validasi Sintaksis Lokal
      if let Err(syntax_error_message) = pre_validate_syntactically(user_input, input_type_str) {
        return Ok(ValidationResponse {
            valid: false,
            message: syntax_error_message,
        });
    }

    // Tahap 2: Validasi Semantik dengan LLM (jika sintaksis OK)
    println!("[DEBUG] Sintaksis OK untuk '{}' ({}), melanjutkan ke validasi LLM.", user_input, input_type_str);
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
    let prompt = format_prompt(user_input, input_type_str);
    let body = común_body_generation(&prompt);

    // Kirim permintaan dan dapatkan responsnya
    let response = client.post(&endpoint).json(&body).send().await?;
    let status = response.status();

    if status.is_success() {
        // Jika status sukses (2xx), proses seperti biasa
        let gemini_api_response: GeminiApiResponse = response.json().await?;
        parse_gemini_response(gemini_api_response)
    } else {
        // Jika status adalah error (4xx atau 5xx)
        // Baca body error sebagai teks untuk informasi tambahan
        // let error_body_text = response.text().await.unwrap_or_else(|e| {
        //     format!("Tidak dapat membaca body respons error dari server (Status: {}). Error internal saat membaca body: {}", status, e)
        // });

        if status == StatusCode::TOO_MANY_REQUESTS { // Kode status 429
            Err(format!(
                "Model '{}' tidak dapat digunakan saat ini karena telah mencapai batas penggunaan (limit).",
                model_name
            ).into()) // .into() akan mengonversi String menjadi Box<dyn Error...>
        } else {
            // Untuk error HTTP lainnya (4xx selain 429, atau 5xx)
            Err(format!(
                "Gagal menggunakan model '{}'. Server merespons dengan kode: {}.",
                model_name, status
            ).into())
        }
    }
}


// --- Fungsi Validasi Sintaksis Lokal ---
fn pre_validate_syntactically(user_input: &str, input_type_str: &str) -> Result<(), String> {
    // Batas panjang umum untuk mencegah input yang sangat besar
    let lower_input_type = input_type_str.to_lowercase();
    if lower_input_type != "deskripsi" && user_input.len() > 512 {
        return Err("Input terlalu panjang (melebihi 512 karakter).".to_string());
    }
  
    if user_input.trim().is_empty() {
        return Err("Input tidak boleh kosong.".to_string());
    }

    match input_type_str.to_lowercase().as_str() {
        "alamat email" | "email" => {
            if user_input.len() > 254 {
                return Err("Error sintaks: Alamat email terlalu panjang (maks 254 karakter).".to_string());
            }
            // Regex sederhana untuk format email.
            static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap());
            if !EMAIL_REGEX.is_match(user_input) {
                return Err("Error sintaks: Format alamat email tidak valid.".to_string());
            }
        }
        "nama lengkap" | "nama" => {
            // Regex untuk karakter yang diizinkan dan panjang total (2-100 karakter).
            // Tidak lagi menggunakan lookahead.
            static NAME_CHARS_LENGTH_REGEX: Lazy<Regex> = Lazy::new(|| {
                Regex::new(r"^[a-zA-Z\s'-]{2,100}$").unwrap()
            });

            if !NAME_CHARS_LENGTH_REGEX.is_match(user_input) {
                return Err(
                    "Error sintaks: Nama lengkap harus terdiri dari 2 hingga 100 karakter \
                    dan hanya boleh berisi huruf (a-z, A-Z), spasi, tanda hubung (-), atau apostrof (')."
                    .to_string()
                );
            }

            // Pemeriksaan tambahan: pastikan ada setidaknya satu huruf.
            if !user_input.chars().any(|c| c.is_alphabetic()) {
                return Err(
                    "Error sintaks: Nama lengkap harus mengandung setidaknya satu huruf."
                    .to_string()
                );
            }
        }
        "nomor telepon indonesia" => {
            if user_input.len() < 9 || user_input.len() > 15 {
                 return Err("Error sintaks: Panjang nomor telepon Indonesia tidak valid (harus antara 9-15 digit).".to_string());
            }
            static PHONE_ID_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\+62|0)8[0-9]{7,12}$").unwrap());
            if !PHONE_ID_REGEX.is_match(user_input) {
                return Err("Error sintaks: Format nomor telepon Indonesia tidak valid. Harus diawali +628 atau 08 dan diikuti 7-12 digit angka.".to_string());
            }
        }
        _ => {
            // Untuk tipe input lain, validasi sintaksis umum (panjang, tidak kosong) sudah dilakukan di atas.
        }
    }
    Ok(())
}



// --- Fungsi Helper untuk menghindari duplikasi kode ---
fn format_prompt(user_input: &str, input_type_str: &str) -> String {
    // Beritahu LLM bahwa validasi sintaksis dasar mungkin sudah dilakukan
    // dan fokus pada aspek semantik/makna/aturan bisnis yang lebih kompleks.
    let pre_validation_note = "Catatan: Input ini mungkin telah melewati pemeriksaan sintaksis dasar. \
                              Fokus pada validitas semantik, kewajaran, dan aturan spesifik (misalnya, domain contoh tidak diizinkan untuk email).";

    match input_type_str.to_lowercase().as_str() {
        "alamat email" | "email" => format!(
            "{} Validasi alamat email berikut: \"{}\". \
             Pastikan formatnya benar, domainnya terlihat valid dan BUKAN domain contoh (seperti example.com, example.org, .test, .localhost, .invalid). \
             Alamat email juga tidak boleh lebih dari 254 karakter. \
             Jawab dalam format JSON: {{ \"valid\": true|false, \"message\": \"penjelasan\" }}",
            pre_validation_note, user_input.replace("\"", "\\\"")
        ),
        "nama lengkap" | "nama" => format!(
            "{} Validasi nama lengkap berikut: \"{}\". \
             Nama lengkap seharusnya hanya mengandung huruf, spasi, dan mungkin tanda hubung atau apostrof tunggal. \
             Tidak boleh mengandung angka atau simbol aneh. Panjangnya wajar. \
             Periksa juga apakah nama ini terlihat seperti nama manusia yang sesungguhnya (bukan sekumpulan karakter acak). \
             Jawab dalam format JSON: {{ \"valid\": true|false, \"message\": \"penjelasan\" }}",
            pre_validation_note, user_input.replace("\"", "\\\"")
        ),
        "nomor telepon indonesia" => format!(
            "{} Validasi nomor telepon Indonesia berikut: \"{}\". \
             Nomor telepon Indonesia seharusnya hanya mengandung angka, diawali dengan +62 atau 08. \
             Tidak boleh mengandung simbol aneh atau huruf. Panjangnya wajar untuk nomor Indonesia. \
             Periksa juga apakah nomor ini masuk akal (misalnya, bukan 080000000). \
             Jawab dalam format JSON: {{ \"valid\": true|false, \"message\": \"penjelasan\" }}",
            pre_validation_note, user_input.replace("\"", "\\\"")
        ),
        _ => format!(
            "{} Validasi input berikut dari user, yang merupakan sebuah **{}**: \"{}\".\n\n\
             Berikan penilaian apakah input tersebut valid dan bermakna sebagai **{}** untuk penggunaan praktis. \
             Jika ini adalah alamat email, TIDAK BOLEH menggunakan domain yang dicadangkan untuk contoh atau dokumentasi (seperti example.com, example.net, example.org, example.edu, atau domain .test, .localhost, .invalid). \
             Jika tidak valid karena alasan ini atau alasan lain, berikan alasan dan saran perbaikan. \
             Jawab dalam format JSON yang ketat seperti ini (tanpa markdown atau teks tambahan di luar JSON): \
             {{ \"valid\": true|false, \"message\": \"penjelasan\" }}",
            pre_validation_note,
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
