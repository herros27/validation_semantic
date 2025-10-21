// src/core_logic.rs

use dotenv::dotenv;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::env;

// Import reqwest clients untuk sinkron dan asinkron
#[cfg(feature = "native_ffi_setup")]
use reqwest::blocking::Client as BlockingClient;

use reqwest::Client as AsyncClient;
use reqwest::StatusCode;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ValidationResponse {
    pub valid: bool,
    pub message: String,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportedModel {
    GeminiFlash = 0,
    GeminiFlashLite = 1,
    GeminiFlashLatest = 2,
    Gemma = 3,
}
impl SupportedModel {
    pub fn as_str(&self) -> &'static str {
        match self {
            SupportedModel::GeminiFlash => "gemini-2.5-flash",
            SupportedModel::GeminiFlashLite => "gemini-2.5-flash-lite-preview-06-17",
            SupportedModel::GeminiFlashLatest => "gemini-2.5-flash-preview-09-2025",
            SupportedModel::Gemma => "gemma-3n-e2b-it",
        }
    }
    pub fn from_int(value: i32) -> Option<Self> {
        match value {
            0 => Some(SupportedModel::GeminiFlash),
            1 => Some(SupportedModel::GeminiFlashLite),
            2 => Some(SupportedModel::GeminiFlashLatest),
            3 => Some(SupportedModel::Gemma),
            _ => None,
        }
    }
    pub fn valid_options_desc() -> String {
         format!(
            "0 (GeminiFlash), 1 (GeminiFlashLite), 2 (GeminiFlashLatest), 3 (Gemma)"
        )
    }
}

// Struct untuk parsing respons Gemini API
#[derive(Debug, Deserialize)]
pub struct GeminiApiPart {
    pub text: String,
}
#[derive(Debug, Deserialize)]
pub struct GeminiApiContent {
    pub parts: Vec<GeminiApiPart>,
}
#[derive(Debug, Deserialize)]
pub struct GeminiApiResponseCandidate {
    pub content: GeminiApiContent,
}
#[derive(Debug, Deserialize)]
pub struct GeminiApiResponse {
    pub candidates: Vec<GeminiApiResponseCandidate>,
}

// Konfigurasi API untuk menyimpan API key
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

// --- Fungsi Validasi dengan LLM (Sinkron) ---
#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "native_ffi_setup")]
pub fn validate_input_with_llm_sync(
    user_input: &str,
    model_name: &str,
    input_type_str: &str,
    config: &ApiConfig,
) -> Result<ValidationResponse, Box<dyn std::error::Error + Send + Sync>> {
    // Tahap 1: Validasi Sintaksis Lokal
    if let Err(syntax_error_message) = pre_validate_syntactically(user_input, input_type_str) {
        return Ok(ValidationResponse {
            valid: false,
            message: syntax_error_message,
        });
    }

    // Tahap 2: Validasi Semantik dengan LLM
    println!(
        "      ✅ Validasi Sintaksis OK untuk '{}' ({}), melanjutkan ke validasi LLM.",
        user_input, input_type_str
    );

    let client = BlockingClient::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()?;

    let endpoint = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model_name, config.api_key
    );

    let prompt = format_prompt(user_input, input_type_str);
    let body = common_body_generation(&prompt, model_name);

    let response = client.post(&endpoint).json(&body).send()?;
    let status = response.status();
    let text_body = response.text()?; // simpan hasil text dulu

    if status.is_success() {
        // Parse JSON dari teks (karena response sudah diambil)
        let gemini_api_response: GeminiApiResponse = serde_json::from_str(&text_body)?;
        parse_gemini_response(gemini_api_response)
    } else {
        // Kalau error (status bukan 2xx)
        let mut error_message = format!(
            "Gagal menggunakan model '{}'. Server merespons dengan kode: {}.",
            model_name, status
        );

        // Coba ambil pesan error dari body JSON (kalau ada)
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text_body) {
            if let Some(msg) = json["error"]["message"].as_str() {
                error_message.push_str(&format!(" Pesan: {}", msg));
            }
        } else if !text_body.is_empty() {
            error_message.push_str(&format!(" Detail: {}", text_body));
        }

        // Tangani kasus limit API
        if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            error_message = format!(
                "Model '{}' tidak dapat digunakan saat ini karena telah mencapai batas penggunaan (limit).",
                model_name
            );
        }

        Err(error_message.into())
    }
}

// --- Fungsi Validasi dengan LLM (Asinkron) ---
pub async fn validate_input_with_llm_async( //ubah biar parameter nya API_KEY gemini
    user_input: &str,
    model_name: &str,
    input_type_str: &str,
    gemini_api_key: &str,
) -> Result<ValidationResponse, Box<dyn std::error::Error + Send + Sync>> {
    // Tahap 1: Validasi Sintaksis Lokal
    if let Err(syntax_error_message) = pre_validate_syntactically(user_input, input_type_str) {
        return Ok(ValidationResponse {
            valid: false,
            message: syntax_error_message,
        });
    }

    // Tahap 2: Validasi Semantik dengan LLM
    println!(
        "[DEBUG] Sintaksis OK untuk '{}' ({}), melanjutkan ke validasi LLM.",
        user_input, input_type_str
    );
    let client = {
        let builder = AsyncClient::builder();

        #[cfg(not(target_arch = "wasm32"))]
        let builder = builder.timeout(std::time::Duration::from_secs(60));

        builder
            .build()
            .map_err(|e| format!("Failed to build HTTP client: {}", e))?
    };
    // const API_KEY: &str = "AIzaSyAv_Kb1i1VWg0fbscDGLQwJPYJEmsxLOYA";
    let endpoint = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model_name, gemini_api_key
    );
    let prompt = format_prompt(user_input, input_type_str);
    let body = common_body_generation(&prompt, model_name);

    // Kirim permintaan dan dapatkan responsnya
    let response = client.post(&endpoint).json(&body).send().await?;
    let status = response.status();
    let text_body = response.text().await?; // Ambil body sebagai string

    if status.is_success() {
        // Proses respons sukses
        let gemini_api_response: GeminiApiResponse = serde_json::from_str(&text_body)?;
        parse_gemini_response(gemini_api_response)
    } else {
        // Penanganan error HTTP
        let mut error_message = format!(
            "Gagal menggunakan model '{}'. Server merespons dengan kode: {}.",
            model_name, status
        );

        // Coba ambil pesan error dari body JSON (kalau ada)
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text_body) {
            if let Some(msg) = json["error"]["message"].as_str() {
                error_message.push_str(&format!(" Pesan: {}", msg));
            }
        } else if !text_body.is_empty() {
            error_message.push_str(&format!(" Detail: {}", text_body));
        }

        if status == StatusCode::TOO_MANY_REQUESTS {
            error_message = format!(
                "Model '{}' tidak dapat digunakan saat ini karena telah mencapai batas penggunaan (limit).",
                model_name
            );
        }

        Err(error_message.into())
    }
}

fn clean_json_markdown(raw: &str) -> &str {
    raw.trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim()
}

// --- Fungsi Validasi Sintaksis Lokal ---
pub fn pre_validate_syntactically(user_input: &str, input_type_str: &str) -> Result<(), String> {
    let input = user_input.trim();
    let input_type = input_type_str.trim().to_lowercase();

    if input.is_empty() {
        return Err("Input tidak boleh kosong.".to_string());
    }

    // if !matches!(input_type, "deskripsi" | "text area" | "konten" | "blog") && input.len() > 512 {
    //     return Err("Input terlalu panjang (maksimal 512 karakter).".to_string());
    // }

    if !["text area" , "teks area" , "konten" , "deskripsi" , "blog"].contains(&input_type.as_str())
        && input.len() > 512
    {
        return Err("Input terlalu panjang (maksimal 512 karakter).".into());
    }

    // if !matches!(input_type.as_str(), "deskripsi" | "text area" | "konten" | "blog") && input.len() > 512 {
    //     return Err("Input terlalu panjang (maksimal 512 karakter).".to_string());
    // }

    match input_type.as_str() {
        "alamat email" | "email" => {
            if input.len() > 254 {
                return Err("Alamat email terlalu panjang (maksimal 254 karakter).".to_string());
            }
            static EMAIL_REGEX: Lazy<Regex> =
                Lazy::new(|| Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap());
            if !EMAIL_REGEX.is_match(input) {
                return Err("Format alamat email tidak valid.".to_string());
            }

            // CATATAN: Pengecekan berikut sengaja menolak domain untuk testing (contoh: 'test@example.com').
            // Hal ini akan menyebabkan tes yang menggunakan domain tersebut gagal,
            // yang mana merupakan perilaku yang diharapkan untuk lingkungan produksi.
            // let domain = input.split('@').nth(1).unwrap_or("");
            // let forbidden = ["example.com", "example.org", "example.net", "test", "localhost", "invalid"];
            // if forbidden.iter().any(|d| domain.ends_with(d)) {
            //     return Err("Domain email tidak boleh menggunakan domain contoh/test/localhost/invalid.".to_string());
            // }
        }
        "nama lengkap" | "nama" => {
            // static NAME_REGEX: Lazy<Regex> =
            //     Lazy::new(|| Regex::new(r"^[a-zA-Z\s'-.]{3,80}$").unwrap());
            // if !NAME_REGEX.is_match(input) {
            //     return Err("Nama hanya boleh berisi huruf, spasi, tanda hubung (-), atau apostrof ('). Panjang 3-80 karakter.".to_string());
            // }
            if !input.chars().any(|c| c.is_alphabetic()) {
                return Err("Nama harus mengandung setidaknya satu huruf.".to_string());
            }
            if input.contains("  ") {
                return Err("Nama tidak boleh mengandung dua spasi berurutan.".to_string());
            }
            // Pengecekan spasi di awal/akhir dihapus karena sudah ditangani oleh `.trim()` di awal fungsi.
        }
        "nomor hp indonesia" => {
            // Pengecekan panjang dan spasi manual dihapus karena sudah tercakup oleh validasi Regex di bawah.
            // Regex adalah satu-satunya sumber kebenaran untuk format.
            static PHONE_ID_REGEX: Lazy<Regex> =
                Lazy::new(|| Regex::new(r"^(\+62|0)8[0-9]{7,12}$").unwrap());
            if !PHONE_ID_REGEX.is_match(input) {
                return Err("Format nomor hp Indonesia tidak valid. Harus diawali +628 atau 08 dan diikuti 7-12 digit angka.".to_string());
            }
        }
        "agama" => {
            // Pengecekan panjang dan spasi manual dihapus karena sudah tercakup oleh validasi Regex di bawah.
            // Regex adalah satu-satunya sumber kebenaran untuk format.
            // static PHONE_ID_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\+62|0)8[0-9]{7,12}$").unwrap());
            // if !PHONE_ID_REGEX.is_match(input) {
            //     return Err("Format nomor hp Indonesia tidak valid. Harus diawali +628 atau 08 dan diikuti 7-12 digit angka.".to_string());
            // }
        }
        "" => {
            // Pengecekan panjang dan spasi manual dihapus karena sudah tercakup oleh validasi Regex di bawah.
            // Regex adalah satu-satunya sumber kebenaran untuk format.
            static PHONE_ID_REGEX: Lazy<Regex> =
                Lazy::new(|| Regex::new(r"^(\+62|0)8[0-9]{7,12}$").unwrap());
            if !PHONE_ID_REGEX.is_match(input) {
                return Err("Format nomor hp Indonesia tidak valid. Harus diawali +628 atau 08 dan diikuti 7-12 digit angka.".to_string());
            }
        }
        _ => {
            // Validasi umum sudah dilakukan di atas
        }
    }
    Ok(())
}

// --- Fungsi Helper untuk Formatting dan Parsing ---
pub fn format_prompt(user_input: &str, input_type_str: &str) -> String {
    let pre_validation_note = "Note: This input has passed basic syntactic validation. \
Focus on semantic validity, reasonableness, and relevant business rules. \
Reject meaningless, dummy, or random input.";

    match input_type_str.to_lowercase().as_str() {
        "alamat email" | "email" => format!(
        "{pre_validation_note}\nValidate the following email address: \"{input}\".\n\
        - Ensure the format and domain are valid, and NOT from example domains (example.com, example.org, .test, .localhost, .invalid).\n\
        - Reject emails using dummy, disposable, or unprofessional domains.\n\
        - The email must not exceed 254 characters.\n\
        - If the email is invalid, provide a specific reason and a suggestion for correction.\n\
        Respond ONLY in the following JSON format (in Indonesian, without any extra text): \
        {{ \"valid\": true|false, \"message\": \"penjelasan dalam bahasa Indonesia\" }}",
        input = user_input.replace("\"", "\\\"")

        ),
        "nama institusi" | "nama lembaga" | "institusi" | "lembaga" => format!(
            "{pre_validation_note}\nValidate the input \"{type_str}\" from the following {type_str}.\n\
            Check whether the input represents a valid institution, organization, or agency name that could realistically exist.\n\
            \n\
            Validation rules:\n\
            - The institution name must not be random, meaningless, or dummy text (e.g., 'asdf', 'qwerty', 'Lorem ipsum', 'Test Institution').\n\
            - The name should contain recognizable institutional elements (university, school, company, foundation, agency, etc.).\n\
            - The name may include common institutional terms like 'Universitas', 'Institut', 'Sekolah', 'PT', 'CV', 'Yayasan', 'Lembaga', etc.\n\
            - The name length must be between 5 and 150 characters.\n\
            - The name cannot be a single generic word without institutional context (e.g., 'Institution', 'Organization', 'Company').\n\
            - The name must not start or end with a space.\n\
            - Special characters like hyphens (-), periods (.), parentheses (), and ampersands (&) are acceptable if used appropriately.\n\
            - Avoid clearly unnatural or random characters (e.g., #, $, %, @) unless they are part of a legitimate institution name.\n\
            - The name should sound professional and legitimate, not like placeholder text.\n\
            \n\
            Valid institution name examples:\n\
            - \"Universitas Indonesia\"\n\
            - \"Institut Teknologi Bandung\"\n\
            - \"SMA Negeri 1 Jakarta\"\n\
            - \"PT Telkom Indonesia Tbk\"\n\
            - \"Yayasan Pendidikan Astra Honda Motor\"\n\
            - \"Bank Central Asia\"\n\
            - \"Lembaga Ilmu Pengetahuan Indonesia\"\n\
            - \"Harvard University\"\n\
            - \"Google LLC\"\n\
            - \"Microsoft Corporation\"\n\
            - \"Kementerian Pendidikan dan Kebudayaan\"\n\
            \n\
            Invalid institution name examples:\n\
            - \"asdf institution\"\n\
            - \"qwerty123\"\n\
            - \"Lorem Ipsum University\"\n\
            - \"Test Company\"\n\
            - \"Random School\"\n\
            - \"###\"\n\
            - \"My Institution\"\n\
            \n\
            Important note:\n\
            Institution names vary widely across different sectors and countries. Accept both formal names \
            (like 'Universitas Gadjah Mada') and corporate names (like 'PT Bank Mandiri Tbk'). \
            The key is that the name should appear to be a legitimate, established organization rather than placeholder text.\n\
            \n\
            The output must be in the following JSON format (without any additional text):\n\
            {{\n\
                \"valid\": true or false,\n\
                \"message\": \"a short explanation of why it is valid or invalid and use Bahasa Indonesia for this message response\"\n\
            }}\n\
            \n\
            Input: \"{input}\"",
            type_str = input_type_str,
            input = user_input.replace("\"", "\\\"")
        ),
        "nama perusahaan" => format!(
            "{pre_validation_note}\nValidate the input \"{type_str}\" from the following {type_str}.\n\
            Check whether the input represents a valid company or business name that could realistically exist.\n\
            \n\
            Validation rules:\n\
            - The company name must not be random, meaningless, or dummy text (e.g., 'asdf', 'qwerty', 'Lorem ipsum', 'Test Company').\n\
            - The name should contain recognizable business or corporate elements that suggest it's a real company.\n\
            - The name may include common business suffixes like 'PT', 'CV', 'LLC', 'Inc.', 'Ltd.', 'Corp.', 'Co.', etc.\n\
            - The name length must be between 3 and 120 characters.\n\
            - The name cannot be a single generic word without business context (e.g., 'Company', 'Business', 'Corporation').\n\
            - The name must not start or end with a space.\n\
            - Special characters like hyphens (-), periods (.), ampersands (&), and numbers are acceptable if used appropriately.\n\
            - Avoid clearly unnatural or random characters (e.g., #, $, %, @) unless they are part of a legitimate company name.\n\
            - The name should sound professional and legitimate, not like placeholder text.\n\
            - Company names may include industry-specific terms, founder names, or descriptive business elements.\n\
            \n\
            Valid company name examples:\n\
            - \"PT Telkom Indonesia Tbk\"\n\
            - \"Bank Central Asia\"\n\
            - \"Google LLC\"\n\
            - \"Microsoft Corporation\"\n\
            - \"Apple Inc.\"\n\
            - \"Amazon.com Inc.\"\n\
            - \"PT Bank Mandiri Tbk\"\n\
            - \"CV Sumber Makmur\"\n\
            - \"Unilever Indonesia\"\n\
            - \"McDonald's Corporation\"\n\
            - \"Johnson & Johnson\"\n\
            - \"Berkshire Hathaway Inc.\"\n\
            - \"Procter & Gamble Co.\"\n\
            - \"Coca-Cola Company\"\n\
            - \"Nike Inc.\"\n\
            \n\
            Invalid company name examples:\n\
            - \"asdf company\"\n\
            - \"qwerty123\"\n\
            - \"Lorem Ipsum Corp\"\n\
            - \"Test Company\"\n\
            - \"Random Business\"\n\
            - \"###\"\n\
            - \"My Company\"\n\
            - \"Generic Corp\"\n\
            \n\
            Important note:\n\
            Company names can vary significantly across different jurisdictions and business structures. Accept both formal corporate names \
            (like 'PT Bank Mandiri Tbk') and informal business names (like 'CV Sumber Makmur'). Company names may include founder names, \
            industry terms, geographical references, or descriptive elements that help identify the business.\n\
            The key is that the name should appear to be a legitimate business entity that could realistically exist.\n\
            \n\
            The output must be in the following JSON format (without any additional text):\n\
            {{\n\
                \"valid\": true or false,\n\
                \"message\": \"a short explanation of why it is valid or invalid and use Bahasa Indonesia for this message response\"\n\
            }}\n\
            \n\
            Input: \"{input}\"",
            type_str = input_type_str,
            input = user_input.replace("\"", "\\\"")
        ),
        "nama produk" => format!(
            "{pre_validation_note}\nValidate the input \"{type_str}\" from the following {type_str}.\n\
            Check whether the input represents a valid, legal, and appropriate product name that could realistically exist in the market.\n\
            \n\
            Validation rules:\n\
            - The product name must not be random, meaningless, or dummy text (e.g., 'asdf', 'qwerty', 'Lorem ipsum', 'Test Product').\n\
            - The name should contain recognizable product elements or characteristics that suggest it's a real product.\n\
            - The name may include brand names, product categories, model numbers, or descriptive terms.\n\
            - The name length must be between 3 and 100 characters.\n\
            - The name cannot be a single generic word without product context (e.g., 'Product', 'Item', 'Goods').\n\
            - The name must not start or end with a space.\n\
            - Special characters like hyphens (-), periods (.), numbers, and ampersands (&) are acceptable if used appropriately.\n\
            - Avoid clearly unnatural or random characters (e.g., #, $, %, @) unless they are part of a legitimate product name.\n\
            - The name should sound like a real product that could be sold or marketed legally.\n\
            - Product names may include version numbers, model codes, or series identifiers.\n\
            \n\
            STRICT REJECTION RULES - REJECT products that are:\n\
            - Illegal or prohibited substances (drugs, unregulated medicines, counterfeit products)\n\
            - Sexual enhancement products, aphrodisiacs, or adult content products\n\
            - Products that violate health regulations or BPOM guidelines\n\
            - Products with inappropriate, offensive, or sexually explicit content\n\
            - Products that promote illegal activities or substances\n\
            - Unregulated pharmaceutical or medical products sold without proper authorization\n\
            \n\
            Valid product name examples:\n\
            - \"iPhone 15 Pro\"\n\
            - \"Samsung Galaxy S24\"\n\
            - \"MacBook Air M2\"\n\
            - \"Nike Air Max 270\"\n\
            - \"Coca-Cola Classic\"\n\
            - \"Toyota Camry Hybrid\"\n\
            - \"PlayStation 5\"\n\
            - \"Microsoft Office 365\"\n\
            - \"Adidas Ultraboost 22\"\n\
            - \"Tesla Model 3\"\n\
            - \"Indomie Goreng\"\n\
            - \"Aqua 600ml\"\n\
            - \"Sari Roti Tawar\"\n\
            - \"Beras Premium Pandanwangi\"\n\
            - \"Paracetamol 500mg\" (legitimate medicine)\n\
            - \"Vitamin C 1000mg\" (legitimate supplement)\n\
            \n\
            Invalid product name examples (MUST REJECT):\n\
            - \"Obat Kuat Sex\" (illegal sexual enhancement product)\n\
            - \"Viagra Generic\" (unregulated medicine)\n\
            - \"asdf product\"\n\
            - \"qwerty123\"\n\
            - \"Lorem Ipsum Phone\"\n\
            - \"Test Product\"\n\
            - \"Random Item\"\n\
            - \"###\"\n\
            - \"My Product\"\n\
            - \"Generic Goods\"\n\
            - Any product with sexual, adult, or inappropriate content\n\
            - Any unregulated pharmaceutical or medical product\n\
            \n\
            Important note:\n\
            Product names can vary widely across different industries and markets. Accept both international brand names \
            (like 'iPhone 15 Pro') and local product names (like 'Indomie Goreng'). However, STRICTLY REJECT any product \
            that appears to be illegal, inappropriate, or violates health/medical regulations.\n\
            The key is that the name should appear to be a legitimate, legal product that could realistically exist in the market.\n\
            \n\
            The output must be in the following JSON format (without any additional text):\n\
            {{\n\
                \"valid\": true or false,\n\
                \"message\": \"a short explanation of why it is valid or invalid and use Bahasa Indonesia for this message response\"\n\
            }}\n\
            \n\
            Input: \"{input}\"",
            type_str = input_type_str,
            input = user_input.replace("\"", "\\\"")
        ),
       
        "nama lokasi" | "lokasi" | "tempat" => format!(
            "{pre_validation_note}\nValidate the input \"{type_str}\" from the following {type_str}.\n\
            Check whether the input represents a valid location name that could realistically exist.\n\
            \n\
            Validation rules:\n\
            - The location name must not be random, meaningless, or dummy text (e.g., 'asdf', 'qwerty', 'Lorem ipsum', 'Test Location').\n\
            - The name should contain recognizable geographical elements that suggest it's a real place.\n\
            - The name may include cities, districts, provinces, countries, landmarks, buildings, or geographical features.\n\
            - The name length must be between 3 and 100 characters.\n\
            - The name cannot be a single generic word without geographical context (e.g., 'Location', 'Place', 'Area').\n\
            - The name must not start or end with a space.\n\
            - Special characters like hyphens (-), periods (.), apostrophes ('), and numbers are acceptable if used appropriately.\n\
            - Avoid clearly unnatural or random characters (e.g., #, $, %, @) unless they are part of a legitimate location name.\n\
            - The name should sound like a real geographical location that could exist on a map.\n\
            - Location names may include administrative divisions, directional indicators, or descriptive geographical terms.\n\
            \n\
            Valid location name examples:\n\
            - \"Jakarta\"\n\
            - \"Surabaya\"\n\
            - \"Bandung\"\n\
            - \"Medan\"\n\
            - \"New York City\"\n\
            - \"Los Angeles\"\n\
            - \"London\"\n\
            - \"Tokyo\"\n\
            - \"Paris\"\n\
            - \"Monas (Monumen Nasional)\"\n\
            - \"Borobudur Temple\"\n\
            - \"Bali\"\n\
            - \"Yogyakarta\"\n\
            - \"Semarang\"\n\
            - \"Makassar\"\n\
            - \"Palembang\"\n\
            - \"Banjarmasin\"\n\
            - \"Pontianak\"\n\
            - \"Manado\"\n\
            - \"Ambon\"\n\
            - \"Kota Tua Jakarta\"\n\
            - \"Pasar Tanah Abang\"\n\
            \n\
            Invalid location name examples:\n\
            - \"asdf city\"\n\
            - \"qwerty123\"\n\
            - \"Lorem Ipsum Town\"\n\
            - \"Test Location\"\n\
            - \"Random Place\"\n\
            - \"###\"\n\
            - \"My Location\"\n\
            - \"Generic Area\"\n\
            \n\
            Important note:\n\
            Location names can vary significantly across different countries, languages, and administrative systems. Accept both formal place names \
            (like 'Jakarta') and informal location references (like 'Kota Tua Jakarta' or 'Pasar Tanah Abang'). Location names may include \
            administrative divisions, landmarks, historical references, or descriptive geographical terms that help identify the specific place.\n\
            The key is that the name should appear to be a legitimate geographical location that could realistically exist.\n\
            \n\
            The output must be in the following JSON format (without any additional text):\n\
            {{\n\
                \"valid\": true or false,\n\
                \"message\": \"a short explanation of why it is valid or invalid and use Bahasa Indonesia for this message response\"\n\
            }}\n\
            \n\
            Input: \"{input}\"",
            type_str = input_type_str,
            input = user_input.replace("\"", "\\\"")
        ),
        "nama lengkap" | "nama" => format!(
            "{pre_validation_note}\nValidate the input \"{type_str}\" from the following {type_str}.\n\
            Check whether the input represents a valid human full name in general, \
            while remaining tolerant toward unique, international, or non-conventional names.\n\
            \n\
            Validation rules:\n\
            - The name must not be random, meaningless, or dummy text (e.g., 'asdf', 'qwerty', 'Lorem ipsum', 'My Name').\n\
            - The name may contain certain special characters such as hyphens (-), apostrophes ('), \
            or non-Latin letters (æ, é, ñ, ü, ø, etc.) if they are reasonably used in real human names.\n\
            - Avoid clearly unnatural or random characters (e.g., #, $, %, @, or digits in the middle of the name without clear meaning).\n\
            - The name length must be between 3 and 80 characters.\n\
            - The name cannot be a single generic word without human context (e.g., 'Human', 'Person', 'Name').\n\
            - The name must not start or end with a space.\n\
            \n\
            Valid name examples:\n\
            - \"Budi Santoso\"\n\
            - \"Siti Aisyah\"\n\
            - \"O'Connor\"\n\
            - \"Anne-Marie\"\n\
            - \"X Æ A-Xii Musk\"\n\
            - \"Zoë Kravitz\"\n\
            - \"Renée O'Hara\"\n\
            \n\
            Invalid name examples:\n\
            - \"asdf\"\n\
            - \"abcd efghijk\"\n\
            - \"qwerty123\"\n\
            - \"Lorem Ipsum\"\n\
            - \"My Name\"\n\
            - \"###\"\n\
            \n\
            Important note:\n\
            Some real human names may include non-standard characters or unique spellings, \
            such as those used by public figures like \"X Æ A-Xii Musk\", \"Søren Kierkegaard\", \
            or \"Zoë Kravitz\". Such names should still be considered valid as long as they are genuine human names.\n\
            \n\
            The output must be in the following JSON format (without any additional text):\n\
            {{\n\
                \"valid\": true or false,\n\
                \"message\": \"a short explanation of why it is valid or invalid and use Bahasa Indonesia for this message response\"\n\
            }}\n\
            \n\
            Input: \"{input}\"",
            type_str = input_type_str,
            input = user_input.replace("\"", "\\\"")
        ),
        "judul" => format!(
            "{pre_validation_note}\nValidate the input \"{type_str}\" from the following {type_str}.\n\
            Check whether the input represents a valid title that could realistically be used for articles, documents, books, or other content.\n\
            \n\
            Validation rules:\n\
            - The title must not be random, meaningless, or dummy text (e.g., 'asdf', 'qwerty', 'Lorem ipsum', 'Test Title').\n\
            - The title should contain meaningful words that suggest it describes actual content or purpose.\n\
            - The title may include descriptive words, subject matter, or thematic elements that indicate its content.\n\
            - The title length must be between 5 and 200 characters.\n\
            - The title cannot be a single generic word without descriptive context (e.g., 'Title', 'Article', 'Document').\n\
            - The title must not start or end with a space.\n\
            - Special characters like hyphens (-), colons (:), question marks (?), exclamation marks (!), and parentheses () are acceptable if used appropriately.\n\
            - Avoid clearly unnatural or random characters (e.g., #, $, %, @) unless they are part of a legitimate title format.\n\
            - The title should sound like a real title that could be used for actual content.\n\
            - Titles may include subtitles, explanatory phrases, or descriptive elements separated by punctuation.\n\
            - The title should be grammatically coherent and meaningful.\n\
            \n\
            Valid title examples:\n\
            - \"Cara Membuat Website dengan HTML dan CSS\"\n\
            - \"Panduan Lengkap Belajar Pemrograman Python\"\n\
            - \"Sejarah Perkembangan Teknologi di Indonesia\"\n\
            - \"Tips dan Trik Memasak Nasi Goreng yang Lezat\"\n\
            - \"Analisis Pasar Saham: Prediksi 2024\"\n\
            - \"How to Build a Successful Startup\"\n\
            - \"The Future of Artificial Intelligence\"\n\
            - \"Understanding Machine Learning Algorithms\"\n\
            - \"Panduan Wisata ke Bali: Tempat-Tempat Terbaik\"\n\
            - \"Resep Masakan Tradisional Indonesia\"\n\
            - \"Cara Mengatasi Stres di Tempat Kerja\"\n\
            - \"Review: iPhone 15 vs Samsung Galaxy S24\"\n\
            - \"Belajar Bahasa Inggris: Grammar Dasar\"\n\
            - \"Investasi Saham untuk Pemula\"\n\
            - \"Teknologi Blockchain: Penjelasan Lengkap\"\n\
            \n\
            Invalid title examples:\n\
            - \"asdf title\"\n\
            - \"qwerty123\"\n\
            - \"Lorem Ipsum Article\"\n\
            - \"Test Title\"\n\
            - \"Random Document\"\n\
            - \"###\"\n\
            - \"My Title\"\n\
            - \"Generic Article\"\n\
            - \"asdf qwerty zxcv\"\n\
            - \"123456789\"\n\
            \n\
            Important note:\n\
            Titles can vary widely across different types of content, languages, and purposes. Accept both formal titles \
            (like 'Analisis Pasar Saham: Prediksi 2024') and informal titles (like 'Tips dan Trik Memasak Nasi Goreng yang Lezat'). \
            Titles may include subtitles, explanatory phrases, or descriptive elements that help identify the content's purpose or subject matter.\n\
            The key is that the title should appear to be a legitimate title that could realistically be used for actual content.\n\
            \n\
            The output must be in the following JSON format (without any additional text):\n\
            {{\n\
                \"valid\": true or false,\n\
                \"message\": \"a short explanation of why it is valid or invalid and use Bahasa Indonesia for this message response\"\n\
            }}\n\
            \n\
            Input: \"{input}\"",
            type_str = input_type_str,
            input = user_input.replace("\"", "\\\"")
        ),
        "pekerjaan" => format!(
            "{pre_validation_note}\nValidate the input \"{type_str}\" from the following {type_str}.\n\
            Check whether the input represents a valid job title, occupation, or profession that could realistically exist.\n\
            \n\
            Validation rules:\n\
            - The job title must not be random, meaningless, or dummy text (e.g., 'asdf', 'qwerty', 'Lorem ipsum', 'Test Job').\n\
            - The name should contain recognizable occupational or professional elements that suggest it's a real job.\n\
            - The name may include industry-specific terms, skill levels, departments, or professional designations.\n\
            - The name length must be between 3 and 80 characters.\n\
            - The name cannot be a single generic word without professional context (e.g., 'Job', 'Work', 'Profession').\n\
            - The name must not start or end with a space.\n\
            - Special characters like hyphens (-), periods (.), parentheses (), and forward slashes (/) are acceptable if used appropriately.\n\
            - Avoid clearly unnatural or random characters (e.g., #, $, %, @) unless they are part of a legitimate job title.\n\
            - The name should sound like a real profession or job that could exist in the workplace.\n\
            - Job titles may include seniority levels, specializations, or department affiliations.\n\
            - The title should be grammatically coherent and professionally appropriate.\n\
            \n\
            Valid job title examples:\n\
            - \"Software Developer\"\n\
            - \"Marketing Manager\"\n\
            - \"Data Analyst\"\n\
            - \"Graphic Designer\"\n\
            - \"Project Manager\"\n\
            - \"Sales Representative\"\n\
            - \"HR Specialist\"\n\
            - \"Financial Analyst\"\n\
            - \"Customer Service Representative\"\n\
            - \"Web Developer\"\n\
            - \"Content Writer\"\n\
            - \"Account Manager\"\n\
            - \"UX/UI Designer\"\n\
            - \"DevOps Engineer\"\n\
            - \"Product Manager\"\n\
            - \"Digital Marketing Specialist\"\n\
            - \"Business Analyst\"\n\
            - \"Quality Assurance Engineer\"\n\
            - \"Social Media Manager\"\n\
            - \"Technical Writer\"\n\
            - \"Senior Software Engineer\"\n\
            - \"Junior Accountant\"\n\
            - \"Lead Designer\"\n\
            - \"Operations Manager\"\n\
            \n\
            Invalid job title examples:\n\
            - \"asdf worker\"\n\
            - \"qwerty123\"\n\
            - \"Lorem Ipsum Job\"\n\
            - \"Test Position\"\n\
            - \"Random Work\"\n\
            - \"###\"\n\
            - \"My Job\"\n\
            - \"Generic Position\"\n\
            - \"asdf qwerty zxcv\"\n\
            - \"123456789\"\n\
            \n\
            Important note:\n\
            Job titles can vary significantly across different industries, companies, and regions. Accept both formal job titles \
            (like 'Senior Software Engineer') and informal job descriptions (like 'Web Developer'). Job titles may include \
            seniority levels, specializations, department affiliations, or skill-specific designations that help identify \
            the professional role and responsibilities.\n\
            The key is that the title should appear to be a legitimate profession or job that could realistically exist in the workplace.\n\
            \n\
            The output must be in the following JSON format (without any additional text):\n\
            {{\n\
                \"valid\": true or false,\n\
                \"message\": \"a short explanation of why it is valid or invalid and use Bahasa Indonesia for this message response\"\n\
            }}\n\
            \n\
            Input: \"{input}\"",
            type_str = input_type_str,
            input = user_input.replace("\"", "\\\"")
        ),
        
        "tag" => format!(
            "{pre_validation_note}\nValidate the input \"{type_str}\" from the following {type_str}.\n\
            Check whether the input represents a valid tag that could realistically be used for categorization, labeling, or content organization.\n\
            \n\
            Validation rules:\n\
            - The tag must not be random, meaningless, or dummy text (e.g., 'asdf', 'qwerty', 'Lorem ipsum', 'Test Tag').\n\
            - The tag should contain meaningful words that suggest it describes or categorizes content.\n\
            - The tag may include descriptive terms, categories, topics, or keywords that help organize content.\n\
            - The tag length must be between 2 and 50 characters (tags are typically shorter than other content).\n\
            - The tag cannot be a single generic word without descriptive context (e.g., 'Tag', 'Label', 'Category').\n\
            - The tag must not start or end with a space.\n\
            - Special characters like hyphens (-), underscores (_), periods (.), and numbers are acceptable if used appropriately.\n\
            - Avoid clearly unnatural or random characters (e.g., #, $, %, @, spaces) unless they are part of a legitimate tag format.\n\
            - The tag should sound like a real category or keyword that could be used for content organization.\n\
            - Tags are typically lowercase or camelCase, and may be single words or short phrases.\n\
            - The tag should be concise and descriptive of the content it represents.\n\
            \n\
            Valid tag examples:\n\
            - \"technology\"\n\
            - \"programming\"\n\
            - \"web-development\"\n\
            - \"machine-learning\"\n\
            - \"artificial-intelligence\"\n\
            - \"data-science\"\n\
            - \"mobile-apps\"\n\
            - \"ui-design\"\n\
            - \"frontend\"\n\
            - \"backend\"\n\
            - \"javascript\"\n\
            - \"python\"\n\
            - \"react\"\n\
            - \"nodejs\"\n\
            - \"tutorial\"\n\
            - \"beginner\"\n\
            - \"advanced\"\n\
            - \"tips\"\n\
            - \"review\"\n\
            - \"news\"\n\
            - \"business\"\n\
            - \"marketing\"\n\
            - \"startup\"\n\
            - \"productivity\"\n\
            - \"design\"\n\
            - \"cooking\"\n\
            - \"travel\"\n\
            - \"fitness\"\n\
            - \"health\"\n\
            - \"education\"\n\
            \n\
            Invalid tag examples:\n\
            - \"asdf tag\"\n\
            - \"qwerty123\"\n\
            - \"Lorem Ipsum\"\n\
            - \"Test Tag\"\n\
            - \"Random Label\"\n\
            - \"###\"\n\
            - \"My Tag\"\n\
            - \"Generic Category\"\n\
            - \"asdf qwerty\"\n\
            - \"123456789\"\n\
            - \"tag with spaces\"\n\
            - \"@#$%\"\n\
            \n\
            Important note:\n\
            Tags can vary widely across different platforms, content types, and organizational systems. Accept both single-word tags \
            (like 'technology') and compound tags (like 'web-development' or 'machine-learning'). Tags are typically used for \
            content categorization, search optimization, and organization purposes.\n\
            The key is that the tag should appear to be a legitimate category or keyword that could realistically be used for content organization.\n\
            \n\
            The output must be in the following JSON format (without any additional text):\n\
            {{\n\
                \"valid\": true or false,\n\
                \"message\": \"a short explanation of why it is valid or invalid and use Bahasa Indonesia for this message response\"\n\
            }}\n\
            \n\
            Input: \"{input}\"",
            type_str = input_type_str,
            input = user_input.replace("\"", "\\\"")
        ),
        "alamat" => format!(
            "{pre_validation_note}\nValidate the input \"{type_str}\" from the following {type_str}.\n\
            Check whether the input represents a valid physical address that could realistically exist.\n\
            \n\
            Validation rules:\n\
            - The address must contain at least a street name or building name and a city/town name.\n\
            - The address should follow a reasonable format structure (street number, street name, city, postal code, etc.).\n\
            - The address must not be random, meaningless, or dummy text (e.g., 'asdf', 'qwerty', 'Lorem ipsum', 'Test Address').\n\
            - The address length must be between 10 and 200 characters.\n\
            - The address should contain recognizable geographical elements (street, avenue, road, city names, etc.).\n\
            - Special characters like numbers, hyphens (-), commas (,), and periods (.) are acceptable if used appropriately.\n\
            - The address cannot be a single generic word without location context (e.g., 'Address', 'Location', 'Place').\n\
            - The address must not start or end with a space.\n\
            - Avoid clearly unnatural or random characters (e.g., #, $, %, @) unless they are part of a legitimate address format.\n\
            \n\
            Valid address examples:\n\
            - \"Jl. Sudirman No. 123, Jakarta Pusat 10270\"\n\
            - \"123 Main Street, New York, NY 10001\"\n\
            - \"Apt 4B, 456 Oak Avenue, Bandung 40111\"\n\
            - \"Villa Permata Blok A-15, Tangerang Selatan\"\n\
            - \"1234 Elm St., Suite 200, Chicago, IL 60601\"\n\
            - \"Kompleks Perumahan Mawar, Jl. Kenanga No. 45, Surabaya\"\n\
            \n\
            Invalid address examples:\n\
            - \"asdf qwerty\"\n\
            - \"Test Address\"\n\
            - \"Lorem Ipsum Street\"\n\
            - \"123###\"\n\
            - \"My Home Address\"\n\
            - \"Random Location\"\n\
            \n\
            Important note:\n\
            Address formats vary significantly between countries and regions. Accept both international formats \
            (like US addresses with state codes) and local formats (like Indonesian addresses with 'Jl.' for 'Jalan'). \
            The key is that the address should appear to be a real, deliverable location rather than placeholder text.\n\
            \n\
            The output must be in the following JSON format (without any additional text):\n\
            {{\n\
                \"valid\": true or false,\n\
                \"message\": \"a short explanation of why it is valid or invalid and use Bahasa Indonesia for this message response\"\n\
            }}\n\
            \n\
            Input: \"{input}\"",
            type_str = input_type_str,
            input = user_input.replace("\"", "\\\"")
        ),
        "text area" | "teks area" | "konten" | "deskripsi" | "blog" | "cerita" | "komentar" => format!(
        "{note}\nValidate the input \"{type_str}\" from the following text.\n\
        Check whether the entered text is truly meaningful content (for example: article, blog, story, comment, note, or description) and not just dummy/placeholder or random text.\n\
        \n\
        Validation rules:\n\
        - Do not limit character length as long as it is reasonable (e.g., 30–5000 characters).\n\
        - The content must consist of sentences/paragraphs with clear meaning and understandable context.\n\
        - STRICTLY REJECT any text containing these words: 'test', 'dummy', 'sample', 'example', 'placeholder', 'lorem ipsum', 'asdf', 'qwerty', 'not real', 'fake', 'mock', 'demo', 'trial'.\n\
        - It must NOT be dummy/placeholder text such as 'Lorem ipsum', 'asdf', 'qwerty', or random meaningless words.\n\
        - It must NOT be repetitive copy-paste paragraphs without meaning.\n\
        - It must NOT be too short (minimum 30 characters).\n\
        - It is recommended to have proper sentence structure, not just keywords or random lists.\n\
        - Can consist of multiple paragraphs.\n\
        - The content must provide useful information, not just empty or generic statements.\n\
        - For stories: must have a coherent narrative or flow, even if simple.\n\
        - For comments: must contain opinions, reactions, or responses to a specific topic.\n\
        - For general content: must provide useful information, explanation, or description.\n\
        \n\
        Examples of valid content:\n\
        - \"Today I learned about the difference between HTTP and HTTPS. HTTPS is more secure because it uses encryption, keeping user data protected.\"\n\
        - \"It is important for young people to understand the impact of social media. When used wisely, social media can help with learning and networking.\"\n\
        - \"Yesterday I went to a traditional market and met a friendly vegetable seller. He shared about the difficulties of selling crops during the pandemic.\"\n\
        - \"In my opinion, this article is very informative and helped me understand the concept of machine learning better. The explanation is beginner-friendly.\"\n\
        - \"I agree with your view on the importance of character education. In today’s digital era, moral values should be emphasized early on.\"\n\
        \n\
        Examples of INVALID content (must be rejected):\n\
        - \"Lorem ipsum dolor sit amet, consectetur adipiscing elit...\"\n\
        - \"asdf qwerty zxcv 12345\"\n\
        - \"hello hello hello hello hello\" (repeated meaningless words)\n\
        - \"my blog\" (too short and meaningless)\n\
        - \"good\" (too short without context)\n\
        - \"my story\" (no narrative or storyline)\n\
        - \"Test paragraph not real\" (contains 'test' and 'not real')\n\
        - \"This is a sample text\" (contains 'sample')\n\
        - \"Dummy content for example\" (contains 'dummy' and 'example')\n\
        - \"Placeholder text here\" (contains 'placeholder')\n\
        - \"Mock data for demo\" (contains 'mock' and 'demo')\n\
        \n\
        WARNING: If the input contains words indicating dummy, test, or placeholder text, it MUST be rejected even if the length is sufficient.\n\
        \n\
        Respond ONLY in the following JSON format (in Indonesian, without any extra text):\n\
        {{\n\
            \"valid\": true or false,\n\
            \"message\": \"penjelasan singkat dalam bahasa Indonesia mengapa valid/tidak valid\"\n\
        }}\n\
        \n\
        Input: \"{input}\"",
        note = pre_validation_note,
        type_str = input_type_str,
        input = user_input.replace("\"", "\\\"")

        ),
        _ => format!(
            "{note}\nValidasi input berikut dari user, bertipe \"{type_str}\": \"{input}\"\n\
            Periksa apakah input yang diberikan merupakan data yang valid, bermakna, dan realistis sesuai dengan jenis input yang diminta.\n\
            \n\
            Aturan validasi umum:\n\
            - Tolak input yang tidak bermakna, dummy, placeholder, atau asal-asalan (misal: 'Lorem ipsum', 'asdf', 'qwerty', 'Test Data', 'Random Input', dsb).\n\
            - Input harus memiliki konteks yang jelas dan dapat dipahami sesuai jenis datanya.\n\
            - Input tidak boleh berupa teks acak atau rangkaian karakter tanpa makna.\n\
            - Input harus terdengar seperti data yang legitimate dan realistis.\n\
            \n\
            Aturan spesifik berdasarkan jenis input:\n\
            - Jika input adalah email: domain tidak boleh domain contoh/dummy (seperti example.com, test.com).\n\
            - Jika input adalah nama: harus terlihat seperti nama manusia, institusi, produk, atau entitas asli.\n\
            - Jika input adalah nomor HP: harus masuk akal dan sesuai format Indonesia (08xx-xxxx-xxxx).\n\
            - Jika input adalah alamat: harus mengandung elemen geografis yang realistis.\n\
            - Jika input adalah pekerjaan: harus terdengar seperti job title yang legitimate.\n\
            - Jika input adalah judul: harus memiliki struktur yang wajar untuk judul konten.\n\
            - Jika input adalah tag: harus berupa kategori atau kata kunci yang bermakna.\n\
            - Jika input adalah konten teks: harus memiliki makna dan struktur kalimat yang wajar.\n\
            \n\
            Contoh input valid:\n\
            - Email: \"john.doe@company.com\", \"siti.aisyah@gmail.com\"\n\
            - Nama: \"Budi Santoso\", \"Universitas Indonesia\", \"iPhone 15\"\n\
            - Alamat: \"Jl. Sudirman No. 123, Jakarta Pusat\"\n\
            - Pekerjaan: \"Software Developer\", \"Marketing Manager\"\n\
            - Judul: \"Cara Belajar Programming untuk Pemula\"\n\
            - Tag: \"technology\", \"web-development\"\n\
            \n\
            Contoh input tidak valid:\n\
            - \"asdf\", \"qwerty123\", \"Lorem ipsum\", \"Test Data\", \"Random Input\"\n\
            - Email: \"test@example.com\", \"user@dummy.org\"\n\
            - Nama: \"My Name\", \"Test Company\", \"Generic Product\"\n\
            - Nomor HP: \"1234567890\" (tidak sesuai format Indonesia)\n\
            \n\
            Jika tidak valid, berikan alasan dan saran perbaikan.\n\
            Jawab HANYA dalam format JSON berikut (tanpa teks tambahan):\n\
            {{ \"valid\": true|false, \"message\": \"penjelasan singkat mengapa valid/tidak valid dalam Bahasa Indonesia\" }}",
            note = pre_validation_note,
            type_str = input_type_str,
            input = user_input.replace("\"", "\\\"")
        ),
    }
}

// Fungsi untuk membuat body request ke Gemini API
pub fn common_body_generation(prompt: &str, model_name: &str) -> serde_json::Value {
    // Konfigurasi dasar
    let mut body = serde_json::json!({
        "contents": [
            {
                "parts": [
                    { "text": prompt }
                ]
            }
        ],
        "safetySettings": [
            { "category": "HARM_CATEGORY_HARASSMENT", "threshold": "BLOCK_NONE" },
            { "category": "HARM_CATEGORY_HATE_SPEECH", "threshold": "BLOCK_NONE" },
            { "category": "HARM_CATEGORY_SEXUALLY_EXPLICIT", "threshold": "BLOCK_NONE" },
            { "category": "HARM_CATEGORY_DANGEROUS_CONTENT", "threshold": "BLOCK_NONE" }
        ]
    });

    // Jika model Gemini, tambahkan konfigurasi khusus
    if model_name.starts_with("gemini") {
        // Aman: pakai as_object_mut() untuk memasukkan field ke root level
        if let Some(map) = body.as_object_mut() {
            map.insert(
                "generationConfig".to_string(),
                serde_json::json!({
                    "temperature": 0.1,
                    "topK": 40,
                    "topP": 0.8,
                    // "maxOutputTokens": 512,
                    "stopSequences": ["\n\n", "Input:", "Example:", "Note:"],
                    "responseMimeType": "application/json"
                }),
            );
        }
    }

    body
}

pub fn common_body_generation_gemma(prompt: &str) -> serde_json::Value {
    serde_json::json!({
        "contents": [
            {
                "role": "user",
                "parts": [
                    { "text": prompt }
                ]
            }
        ],
        "safetySettings": [
            { "category": "HARM_CATEGORY_HARASSMENT", "threshold": "BLOCK_NONE" },
            { "category": "HARM_CATEGORY_HATE_SPEECH", "threshold": "BLOCK_NONE" },
            { "category": "HARM_CATEGORY_SEXUALLY_EXPLICIT", "threshold": "BLOCK_NONE" },
            { "category": "HARM_CATEGORY_DANGEROUS_CONTENT", "threshold": "BLOCK_NONE" }
        ],
        "generationConfig": {
            "responseMimeType": "application/json"
        }
    })
}


pub fn parse_gemini_response(gemini_api_response: GeminiApiResponse,
) -> Result<ValidationResponse, Box<dyn std::error::Error + Send + Sync>> {
    // Ekstrak teks hasil dari model
    let model_generated_text_str: String = gemini_api_response
        .candidates
        .get(0)
        .and_then(|candidate| candidate.content.parts.get(0))
        .map(|part| part.text.clone())
        .ok_or_else(|| "Gagal mengekstrak teks dari respons LLM.".to_string())?;

    // 🧹 Bersihkan format markdown dari model (```json ... ```)
    let clean_json_str = clean_json_markdown(&model_generated_text_str);

    // 🔍 Coba parse hasilnya jadi JSON Value
    let json_val: serde_json::Value = serde_json::from_str(clean_json_str).map_err(|e| {
        format!(
            "Gagal parse string ke JSON Value. Error: {}. Model output: '{}'",
            e, model_generated_text_str
        )
    })?;

    // Jika output berupa array, ambil elemen pertama
    let json_obj = if let Some(array) = json_val.as_array() {
        array
            .get(0)
            .cloned()
            .ok_or("Model output berupa array kosong")?
    } else {
        json_val
    };

    // Parse menjadi struct ValidationResponse
    let parsed: ValidationResponse = serde_json::from_value(json_obj).map_err(|e| {
        format!(
            "Gagal mem-parse JSON menjadi ValidationResponse. Error: {}. Model output: '{}'",
            e, model_generated_text_str
        )
    })?;

    Ok(parsed)
}

