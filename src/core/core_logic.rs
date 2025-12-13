// src/core_logic.rs
// Tahap Refactoring: Memisahkan logika inti ke dalam modul terpisah

use once_cell::sync::Lazy;
use regex::Regex;

// Import reqwest clients untuk sinkron dan asinkron
#[cfg(feature = "native_ffi_setup")]
use reqwest::blocking::Client as BlockingClient;

use reqwest::Client as AsyncClient;
use reqwest::StatusCode;
use crate::models::{
    ValidationResponse,
    GeminiApiResponse
};

use crate::config::*;


// #[derive(Deserialize)]
// pub struct BatchInput {
//     pub field: String,
//     pub value: String,
//     pub input_type: String,
// }




// /*
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
    // Normalisasi input type agar match case-insensitive
    let input_type = input_type_str.trim().to_lowercase();

    // 1. Cek Dasar: Tidak boleh kosong
    if input.is_empty() {
        return Err("Input tidak boleh kosong.".to_string());
    }

    // 2. Batasan Panjang Global (Pencegahan memori overflow/spam ekstrem)
    // Kecuali untuk "text area" atau konten panjang, batasi input wajar (misal 1000 char)
    let is_long_text = matches!(
        input_type.as_str(),
        "text area" | "teks area" | "konten" | "deskripsi" | "blog" | "cerita" | "komentar" | 
        "content" | "description" | "story" | "comment" | "body" | "message" | "post" | "article" | "review" | "summary"
    );

    if !is_long_text && input.len() > 1000 {
        return Err(format!("Input terlalu panjang untuk kategori '{}'.", input_type_str));
    }

    match input_type.as_str() {
        // --- KELOMPOK 1: EMAIL ---
        // Cek: Mengandung @ dan ada titik setelahnya.
        "alamat email" | "email" | "email address" | "mail" => {
            if input.len() > 254 {
                return Err("Email terlalu panjang.".to_string());
            }
            // Regex standar (General): Sesuatu@Sesuatu.Sesuatu
            static EMAIL_REGEX: Lazy<Regex> =
                Lazy::new(|| Regex::new(r"^[^@\s]+@[^@\s]+\.[^@\s]+$").unwrap());
            
            if !EMAIL_REGEX.is_match(input) {
                return Err("Format email tidak valid (contoh: user@domain.com).".to_string());
            }
        }

        // --- KELOMPOK 2: WEBSITE / URL ---
        // Cek: Minimal ada satu titik (.) dan panjang minimal. Tidak wajib http/https.
        "website" | "url" | "link" | "tautan" | "situs" | "domain" | "homepage" | "web" => {
            if !input.contains('.') || input.len() < 4 {
                return Err("Format URL tidak valid (harus mengandung domain, misal: example.com).".to_string());
            }
            if input.contains(' ') {
                return Err("URL tidak boleh mengandung spasi.".to_string());
            }
        }

        "nomor hp indonesia" | "nomor hp" | "phone" | "phone number" | "no hp" | "mobile" | "tel"  => {
            // 1. Normalisasi:
            //    Hapus spasi, strip, kurung.
            //    TAPI: Jangan hapus tanda '+' jika ada di posisi paling depan.
            let input_trimmed = input.trim();
            
            // Cek apakah ada '+' di awal
            // let has_plus = input_trimmed.starts_with('+');

            // Ambil hanya angkanya saja
            let digits_only: String = input_trimmed.chars()
                .filter(|c| c.is_numeric())
                .collect();

            // 2. Cek apakah kosong setelah dibersihkan
            if digits_only.is_empty() {
                return Err("Nomor telepon tidak boleh kosong.".to_string());
            }

            // 3. Validasi Panjang (E.164 Standard)
            //    Maksimal 15 digit. Minimal kita set 7 (untuk jaga-jaga nomor pendek internasional).
            if digits_only.len() < 7 || digits_only.len() > 15 {
                return Err("Panjang nomor telepon tidak valid (Global: 7-15 digit).".to_string());
            }

            // 4. Cek karakter valid
            let is_valid_chars = input_trimmed.chars().all(|c| 
                c.is_numeric() || c == '+' || c == '-' || c == ' ' || c == '(' || c == ')'
            );

            if !is_valid_chars {
                return Err("Nomor telepon mengandung karakter yang tidak valid.".to_string());
            }
        }

        // --- KELOMPOK 3: USERNAME ---
        // Cek: Tidak boleh ada spasi, panjang minimal 3.
        "username" | "nama pengguna" | "handle" | "user id" | "account name" | "id pengguna" | "nama lengkap" | "nama" | "full name" | "name" | "complete name" | "nickname" | "first name" | "last name" => {
            if input.len() < 3 {
                return Err("Username terlalu pendek (minimal 3 karakter).".to_string());
            }
            if input.contains("  ") {
                return Err(format!("'{}' tidak boleh mengandung double spasi.", input_type_str));
            }
            if input.chars().all(char::is_numeric) {
                return Err(format!("'{}' tidak boleh hanya terdiri dari angka.", input_type_str));
            }
            // Opsional: Cek karakter aneh, tapi "general" biarkan alphanumeric + simbol dasar
        }

        // --- KELOMPOK 4: IDENTITAS (NIK, KTP, NPWP, DLL) ---
        // Cek: Harus mengandung angka. Panjang minimal logis (misal 5).
        "nik" | "ktp" | "npwp" | "nomor identitas" | "identity number" | "passport" | "sim" | "id card" | "no ktp" => {
             if input.len() < 5 {
                return Err("Nomor identitas terlalu pendek.".to_string());
            }
            // Harus mengandung setidaknya satu digit angka
            if !input.chars().any(|c| c.is_numeric()) {
                return Err("Nomor identitas harus mengandung angka.".to_string());
            }
        }

        // --- KELOMPOK 5: TANGGAL / WAKTU ---
        // Cek: Harus mengandung angka.
        "tanggal" | "date" | "tanggal lahir" | "dob" | "birth date" | "waktu" | "time" | "tgl" | "tgl lahir" => {
            // General check: minimal ada angka (misal "17 agustus" atau "2023-01-01")
            if !input.chars().any(|c| c.is_numeric()) {
                return Err("Format tanggal/waktu harus mengandung angka.".to_string());
            }
        }

        // --- KELOMPOK 6: NUMERIK (UMUR, HARGA, GAJI) ---
        // Cek: Harus mengandung angka.
        "umur" | "age" | "harga" | "price" | "gaji" | "salary" | "nominal" | "amount" | "jumlah" | "biaya" | "cost" => {
            // Kita izinkan format "Rp 50.000" atau "25 tahun", jadi cukup cek ada angka saja.
            if !input.chars().any(|c| c.is_numeric()) {
                 return Err("Input harus mengandung nilai angka.".to_string());
            }
        }

        // --- KELOMPOK 7: TEKS UMUM (NAMA, ALAMAT, PRODUK, JUDUL, TAG, DLL) ---
        // Cek: Panjang minimal 2 karakter agar tidak cuma 1 huruf/simbol acak.
        "nama institusi" | "nama lembaga" | "institusi" | "lembaga" | 
        "institution name" | "organization name" | "institution" | "organization" | "agency" | "institute" |
        "nama perusahaan" | "perusahaan" | "company name" | "company" | "business name" | "business" | "corporate" |
        "nama produk" | "product name" | "produk" | "product" | 
        "nama barang" | "barang" | "item name" | "item" | 
        "nama item" | "merchandise" | "goods" | "komoditas" | "jenis barang" |
        "nama lokasi" | "lokasi" | "tempat" | "location name" | "location" | "place" | "venue" | "spot" | "area" |
        "judul" | "title" | "subject" | "headline" | "caption" | "topic" |
        "pekerjaan" | "job" | "occupation" | "profesi" | "jabatan" | "role" | "peran" | "posisi" | "karir" | "career" | "job title" |
        "tag" | "kategori" | "category" | "label" | "keyword" | "tags" |
        "alamat" | "address" | "home address" | "street address" | "domicile" => {
            if input.len() < 2 {
                return Err("Input terlalu pendek (minimal 2 karakter).".to_string());
            }
        }

        // --- KELOMPOK 8: KONTEN PANJANG ---
        // Cek: Panjang minimal agak lebih besar (misal 10) agar bukan spam "tes".
        "text area" | "teks area" | "konten" | "deskripsi" | "blog" | "cerita" | "komentar" | 
        "content" | "description" | "story" | "comment" | "body" | "message" | "post" | "article" | "review" | "summary" => {
             if input.len() < 10 {
                return Err("Konten terlalu pendek (minimal 10 karakter).".to_string());
            }
        }

        // Default: Loloskan saja jika tipe tidak dikenal, biarkan LLM yang cek
        _ => {}
    }

    Ok(())
}

// --- Fungsi Helper untuk Formatting dan Parsing ---
pub fn format_prompt(user_input: &str, input_type_str: &str) -> String {
    let pre_validation_note = "Note: This input has passed basic syntactic validation. \
Focus on semantic validity, reasonableness, and relevant business rules. \
Reject meaningless, dummy, or random input.";

    match input_type_str.to_lowercase().as_str() {
        "alamat email" | "email" | "email address" | "mail" => format!(
        "{pre_validation_note}\nValidate the following email address: \"{input}\".\n\
        - Ensure the format and domain are valid, and NOT from example domains (example.com, example.org, .test, .localhost, .invalid).\n\
        - Reject emails using dummy, disposable, or unprofessional domains.\n\
        - The email must not exceed 254 characters.\n\
        - If the email is invalid, provide a specific reason and a suggestion for correction.\n\
        Respond ONLY in the following JSON format (in Indonesian, without any extra text): \
        {{ \"valid\": true|false, \"message\": \"penjelasan dalam bahasa Indonesia\" }}",
        input = user_input.replace("\"", "\\\"")

        ),
        "nama institusi" | "nama lembaga" | "institusi" | "lembaga" | 
    "institution name" | "organization name" | "institution" | "organization" | "agency" | "institute" => format!(
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
        "nama perusahaan" | "perusahaan" | 
    "company name" | "company" | "business name" | "business" | "corporate" => format!(
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
        "nama produk" | "product name" | "produk" | "product" | 
    "nama barang" | "barang" | "item name" | "item" | 
    "nama item" | "merchandise" | "goods" | "komoditas" | "jenis barang" => format!(
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
       
        "nama lokasi" | "lokasi" | "tempat" | 
    "location name" | "location" | "place" | "venue" | "spot" | "area" => format!(
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
        "nama lengkap" | "nama" | 
    "full name" | "name" | "complete name" | "nickname" | "first name" | "last name" => format!(
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
        "judul" | "title" | "subject" | "headline" | "caption" | "topic" => format!(
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
        "pekerjaan" | "job" | "occupation" | "profesi" | "jabatan" | "role" | "peran" | "posisi" | "karir" | "career" | "job title" => format!(
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
        
        "tag" | "kategori" | "category" | "label" | "keyword" | "tags" => format!(
            "{pre_validation_note}\nValidate the input \"{type_str}\" from the following {type_str}.\n\
            Check whether the input represents a valid tag that could realistically be used for categorization, labeling, or content organization.\n\
            \n\
            Validation rules:\n\
            - The tag must not be random, meaningless, or dummy text (e.g., 'asdf', 'qwerty', 'Lorem ipsum', 'Test Tag').\n\
            - The tag should contain meaningful words that describe, categorize, or relate to content.\n\
            - The tag may include one or multiple words (e.g., 'web development', 'machine learning').\n\
            - Tags with spaces in the middle ARE allowed as long as they form a meaningful phrase.\n\
            - The tag length must be between 2 and 50 characters.\n\
            - The tag cannot be a single overly-generic word (e.g., 'Tag', 'Label', 'Category').\n\
            - The tag must not start or end with a space.\n\
            - Special characters like hyphens (-), underscores (_), periods (.), and numbers are acceptable when used appropriately.\n\
            - Avoid clearly unnatural or random characters (e.g., #, $, %, @) unless they are legitimately part of a tag.\n\
            - The tag should sound like a real category, keyword, or label used for organizing content.\n\
            - Multi-word tags should represent clear concepts, topics, or categories (e.g., 'web development', 'data analysis').\n\
            \n\
            Valid tag examples:\n\
            - \"technology\"\n\
            - \"programming\"\n\
            - \"web-development\"\n\
            - \"machine learning\"\n\
            - \"artificial intelligence\"\n\
            - \"mobile apps\"\n\
            - \"ui design\"\n\
            - \"frontend\"\n\
            - \"backend development\"\n\
            - \"data science\"\n\
            - \"react\"\n\
            - \"python\"\n\
            - \"digital marketing\"\n\
            - \"business strategy\"\n\
            - \"health tips\"\n\
            - \"travel guide\"\n\
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
            - \"@#$%\"\n\
            \n\
            Important note:\n\
            Tags may be single words or multi-word phrases, as long as they represent real categories, concepts, or keywords commonly used for organizing or labeling content.\n\
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

        "alamat" | "address" | "home address" | "street address" | "domicile" => format!(
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
        "text area" | "teks area" | "konten" | "deskripsi" | "blog" | "cerita" | "komentar" | 
    "content" | "description" | "story" | "comment" | "body" | "message" | "post" | "article" | "review" | "summary" => format!(
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
        "username" | "nama pengguna" | "handle" | "user id" | "account name" | "id pengguna"=> format!(
            // Logic username...
            "{pre_validation_note}\nValidate the input \"{type_str}\" from the following {type_str}.\n\
        Check whether the input represents a valid, appropriate, and properly formatted user handle or digital identity.\n\
        \n\
        Validation rules:\n\
        - The username must not be random, meaningless, or dummy text (e.g., 'asdf', 'qwerty', 'user1', 'test').\n\
        - The username MUST NOT contain spaces. It should be a single continuous string.\n\
        - The username should typically contain alphanumeric characters (a-z, 0-9), underscores (_), or periods (.).\n\
        - The length must be between 3 and 30 characters.\n\
        - The username cannot be a reserved system word (e.g., 'admin', 'root', 'support', 'system', 'moderator').\n\
        - The username must not look like a full email address (unless specifically requested) or a URL.\n\
        - Avoid clearly unnatural or random character sequences (e.g., '!!!!', '$$$') unless part of a stylized handle.\n\
        - The username should sound like a legitimate digital identity chosen by a human.\n\
        \n\
        STRICT REJECTION RULES - REJECT usernames that are:\n\
        - CONTAINING SPACES (e.g., 'John Doe' is a Name, NOT a Username. 'john_doe' is a Username).\n\
        - Profane, offensive, or vulgar in any language (especially Indonesian and English).\n\
        - Hate speech, racial slurs, or sexually explicit terms.\n\
        - Impersonating verified entities or official positions (e.g., 'Official_Admin', 'Staff_Support').\n\
        - Generic placeholders that look like bot generation (e.g., 'user_12345', 'guest999').\n\
        \n\
        Valid username examples:\n\
        - \"herros27\"\n\
        - \"kemas_khairunsyah\"\n\
        - \"siti.aminah99\"\n\
        - \"gamer_pro_id\"\n\
        - \"teknologi.masa.kini\"\n\
        - \"john_doe\"\n\
        - \"budi.santoso\"\n\
        - \"coding_master\"\n\
        - \"design_guru_2024\"\n\
        - \"alpha.wolf\"\n\
        \n\
        Invalid username examples (MUST REJECT):\n\
        - \"John Doe\" (Contains space - invalid for username)\n\
        - \"kemas khairunsyah\" (Contains space)\n\
        - \"admin\" (Reserved word)\n\
        - \"root\" (Reserved word)\n\
        - \"asdfghjkl\"\n\
        - \"user1234\" (Too generic)\n\
        - \"test_user\"\n\
        - \"anjing_gila\" (Profanity/Inappropriate in Indonesian)\n\
        - \"f*ck_you\" (Profanity)\n\
        - \"$$$$$\"\n\
        - \"http://website.com\"\n\
        - \"budi@gmail.com\" (Looks like email, not a handle)\n\
        \n\
        Important note:\n\
        Distinguish clearly between a 'Name' (Nama Lengkap) and a 'Username'. A Name can have spaces and special characters (e.g., 'Dr. Budi'). \
        A Username is a digital identifier/handle used for login or profile URL and typically CANNOT have spaces (e.g., 'dr_budi'). \
        Prioritize checking for spaces and inappropriate content.\n\
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

        "website" | "url" | "link" | "tautan" | "situs" | "domain" | "homepage" | "web" => format!(
            "{pre_validation_note}\nValidate the input \"{type_str}\" from the following {type_str}.\n\
            Check whether the input represents a valid, properly formatted, and realistic website URL or domain name.\n\
            \n\
            Validation rules:\n\
            - The URL must not be random, meaningless, or dummy text (e.g., 'asdf', 'link', 'test.com', 'example.com').\n\
            - The input should look like a standard URL format (e.g., starting with 'http://', 'https://', 'www.', or ending with a valid TLD like '.com', '.id', '.org').\n\
            - Valid TLDs (Top Level Domains) are required (e.g., .com, .net, .id, .co.id, .io, .dev, etc.).\n\
            - Reject incomplete domains (e.g., just 'google' without '.com').\n\
            - Reject local or private network addresses unless explicitly allowed (e.g., 'localhost', '127.0.0.1', '192.168.x.x').\n\
            - The URL must not contain spaces (URLs cannot have spaces).\n\
            - The URL should appear to be a publicly accessible site.\n\
            \n\
            STRICT REJECTION RULES - REJECT inputs that are:\n\
            - MISSING TLD (e.g., 'facebook', 'youtube' -> Invalid. Must be 'facebook.com').\n\
            - LOCALHOST/PRIVATE IP (e.g., 'localhost:3000', '192.168.1.1').\n\
            - DUMMY DOMAINS (e.g., 'example.com', 'mysite.com', 'test.com').\n\
            - GENERIC TEXT (e.g., 'link', 'website saya', 'klik disini').\n\
            - MALICIOUS/PHISHING PATTERNS (e.g., suspicious long subdomains or misleading typos of famous brands if obvious).\n\
            \n\
            Valid URL examples:\n\
            - \"https://www.google.com\"\n\
            - \"www.tokopedia.com\"\n\
            - \"facebook.com\"\n\
            - \"https://github.com/herros27\"\n\
            - \"kemas.dev\"\n\
            - \"detik.com\"\n\
            - \"https://subdomain.example-real-business.co.id/page\"\n\
            \n\
            Invalid URL examples (MUST REJECT):\n\
            - \"google\" (Missing .com)\n\
            - \"www.google\" (Missing TLD)\n\
            - \"http://\" (Incomplete)\n\
            - \"localhost\"\n\
            - \"127.0.0.1\"\n\
            - \"example.com\"\n\
            - \"test.com\"\n\
            - \"website saya\" (Contains spaces)\n\
            - \"asdf\"\n\
            \n\
            Important note:\n\
            Be lenient on the protocol (http/https). If the user types 'google.com', accept it even without 'https://'. \
            However, be STRICT on the structure (must have a domain name and a TLD). Reject single words that look like search queries.\n\
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

        "nik" | "ktp" | "npwp" | "nomor identitas" | "identity number" | "passport" | "sim" | "id card" | "no ktp" => format!(
            "{pre_validation_note}\nValidate the input \"{type_str}\" from the following {type_str}.\n\
            Check whether the input represents a valid, properly formatted, and realistic identity number.\n\
            \n\
            Validation rules:\n\
            - The input must not be random, meaningless, or dummy text (e.g., '123456', '000000', '111111', 'ktp saya').\n\
            - **NIK / KTP (Indonesian ID)**: MUST be exactly 16 digits long. It must contain ONLY numbers. It should not look like a sequential number (1234567890123456).\n\
            - **NPWP (Tax ID)**: MUST be 15 or 16 digits long. It typically contains numbers and formatting characters (dots/hyphens).\n\
            - **Passport**: MUST be alphanumeric (contain both letters and numbers, usually starting with a letter). Length is typically 7-9 characters.\n\
            - **SIM (Driving License)**: MUST be numeric, typically 12-14 digits.\n\
            - General ID: Must look like a formal government-issued ID, not a simple integer like '1' or '100'.\n\
            \n\
            STRICT REJECTION RULES - REJECT inputs that are:\n\
            - WRONG LENGTH (e.g., NIK with 10 digits or 20 digits).\n\
            - SEQUENTIAL/REPEATING (e.g., '123456789...', '99999999...').\n\
            - ALPHABETIC IN NUMERIC FIELDS (e.g., 'NIK123' -> Invalid, NIK must be purely numeric).\n\
            - DUMMY DATA (e.g., '12345', 'test', 'id_card', '0000000000000000').\n\
            \n\
            Valid examples:\n\
            - \"3273120101900001\" (NIK - 16 digits, realistic format)\n\
            - \"3171234567890001\" (NIK)\n\
            - \"09.254.294.3-407.000\" (NPWP - formatted)\n\
            - \"092542943407000\" (NPWP - unformatted)\n\
            - \"B1234567\" (Passport - Alphanumeric)\n\
            - \"X9876543\" (Passport)\n\
            \n\
            Invalid examples (MUST REJECT):\n\
            - \"123456\" (Too short)\n\
            - \"1234567890123456\" (Sequential dummy)\n\
            - \"1111111111111111\" (Repeating dummy)\n\
            - \"327312010190000\" (15 digits - NIK must be 16)\n\
            - \"A123456789012345\" (NIK cannot contain letters)\n\
            - \"0000000000000000\"\n\
            - \"kartu tanda penduduk\"\n\
            \n\
            Important note:\n\
            For 'NIK' or 'KTP', be very strict about the **16-digit length** requirement. If the user provides spaces or dashes (e.g. '3273-1201...'), consider it valid if the total digit count is 16.\n\
            \n\
            The output must be in the following JSON format (without any additional text):\n\
            {{\n\
                \"valid\": true or false,\n\
                \"message\": \"a short explanation of why it is valid or invalid (mention specifically about length or format if wrong) and use Bahasa Indonesia for this message response\"\n\
            }}\n\
            \n\
            Input: \"{input}\"",
            type_str = input_type_str,
            input = user_input.replace("\"", "\\\"")
        ),

        "tanggal" | "date" | "tanggal lahir" | "dob" | "birth date" | "waktu" | "time" | "tgl" | "tgl lahir" => format!(
            "{pre_validation_note}\nValidate the input \"{type_str}\" from the following {type_str}.\n\
            Check whether the input represents a valid, logically correct, and realistic date or time.\n\
            \n\
            Validation rules:\n\
            - The input must not be random text, dummy numbers, or impossible dates (e.g., 'asdf', '00-00-0000', '32-01-2023').\n\
            - **Format**: Accept common formats like DD-MM-YYYY, YYYY-MM-DD, DD/MM/YYYY, or natural language (e.g., '17 Agustus 1945').\n\
            - **Calendar Logic**: Verify that the day exists in that month (e.g., Reject '30 February', '31 April').\n\
            - **Context - Birth Date (DOB)**: \n\
                - The date MUST be in the past.\n\
                - The year must be realistic for a living person (e.g., between 1900 and Current Year). Reject years like 1800 or 2050.\n\
            - **Context - Future/General Date**: If the label is just 'Date' (not DOB), future dates are allowed unless specified otherwise.\n\
            - **Time**: If input is time, valid formats are HH:MM (24h or 12h with AM/PM). Hours must be 0-23, Minutes 0-59.\n\
            \n\
            STRICT REJECTION RULES - REJECT inputs that are:\n\
            - IMPOSSIBLE DATES (e.g., '32/01/2022', '30/02/2023').\n\
            - UNREALISTIC DOB (e.g., '01/01/2050' -> User hasn't been born yet).\n\
            - DUMMY PATTERNS (e.g., '00/00/0000', '11/11/1111', '1234').\n\
            - INCOMPLETE (e.g., just '2023' or just 'Januari').\n\
            \n\
            Valid examples:\n\
            - \"17-08-1945\"\n\
            - \"17 Agustus 1945\"\n\
            - \"1990-12-31\"\n\
            - \"01/01/2000\"\n\
            - \"14:30\" (Time)\n\
            - \"10 Desember 2024\"\n\
            \n\
            Invalid examples (MUST REJECT):\n\
            - \"30 Februari 2023\" (Date doesn't exist)\n\
            - \"00-00-0000\"\n\
            - \"2050-01-01\" (Invalid if context is Date of Birth)\n\
            - \"123456\"\n\
            - \"tanggal\"\n\
            - \"asdf\"\n\
            - \"32/12/2022\"\n\
            \n\
            The output must be in the following JSON format (without any additional text):\n\
            {{\n\
                \"valid\": true or false,\n\
                \"message\": \"a short explanation of why it is valid or invalid (if invalid, mention specifically if the date doesn't exist or is unrealistic) and use Bahasa Indonesia for this message response\"\n\
            }}\n\
            \n\
            Input: \"{input}\"",
            type_str = input_type_str,
            input = user_input.replace("\"", "\\\"")
        ),

        "umur" | "age" | "harga" | "price" | "gaji" | "salary" | "nominal" | "amount" | "jumlah" | "biaya" | "cost" => format!(
            "{pre_validation_note}\nValidate the input \"{type_str}\" from the following {type_str}.\n\
            Check whether the input represents a valid, logically correct, and realistic numeric value based on its context.\n\
            \n\
            Validation rules:\n\
            - The input must be a numeric value. It can contain digits, currency symbols (Rp, $), or separators (.,).\n\
            - REJECT text that is not a number (e.g., 'mahal', 'murah', 'banyak', 'asdf').\n\
            - **Context - Age (Umur)**: \n\
                - Must be an integer between 0 and 120.\n\
                - Reject negative numbers.\n\
                - Reject unrealistic ages (e.g., 200, 1000).\n\
            - **Context - Price/Salary/Amount (Harga/Gaji)**: \n\
                - Must be a non-negative number (>= 0).\n\
                - Allow currency formatting (e.g., 'Rp 10.000', '100.000', '$50').\n\
                - Reject unrealistic values for the context (e.g., Salary of '10 rupiah', Price of '-5000').\n\
            \n\
            STRICT REJECTION RULES - REJECT inputs that are:\n\
            - NEGATIVE VALUES (unless context explicitly allows debt/loss, generally Age/Price are positive).\n\
            - UNREALISTIC NUMBERS (e.g., Age: '1000', Salary: '1').\n\
            - RANDOM TEXT (e.g., 'seribu', 'sejuta' -> Reject if you require digit format. Accept only if digits are present).\n\
            - MIXED GIBBERISH (e.g., '123asdf').\n\
            \n\
            Valid examples:\n\
            - \"25\" (Age)\n\
            - \"17 tahun\" (Age with unit - Acceptable)\n\
            - \"5.000.000\" (Price/Salary)\n\
            - \"Rp 150.000\"\n\
            - \"$10.50\"\n\
            - \"0\" (Free/Zero amount)\n\
            \n\
            Invalid examples (MUST REJECT):\n\
            - \"-5\" (Negative age/price)\n\
            - \"200\" (Invalid if context is Age)\n\
            - \"abc\"\n\
            - \"mahal banget\"\n\
            - \"10.0.0.1\" (IP Address, not a number)\n\
            \n\
            Important note:\n\
            Handle Indonesian number formatting where '.' is often used as a thousand separator (e.g., 1.000 = one thousand). \
            If the input contains units (e.g., '25 years old', 'Rp 1000'), extract the numeric part to validate realism.\n\
            \n\
            The output must be in the following JSON format (without any additional text):\n\
            {{\n\
                \"valid\": true or false,\n\
                \"message\": \"a short explanation of why it is valid or invalid (mention if the number is unrealistic or negative) and use Bahasa Indonesia for this message response\"\n\
            }}\n\
            \n\
            Input: \"{input}\"",
            type_str = input_type_str,
            input = user_input.replace("\"", "\\\"")
        ),
            _ => format!(
            r#"
            Role: Strict Data Semantic Validator.
            Task: Analyze the following user input which is claimed to be of type "{type_str}".

            Input to Validate: "{input}"
            Context Note: {note}

            Validation Rules:
            1.  **Meaning & Realism**: Reject input that appears to be gibberish (e.g., "asdf", "qwerty"), placeholders (e.g., "Lorem Ipsum", "Test Data", "String"), or generic dummy data.
            2.  **Type Consistency**: The input must semantically match the intended type "{type_str}".
            3.  **Specific Checks**:
                - **Email**: Must use a valid, non-dummy domain (reject example.com, test.com).
                - **Name**: Must look like a legitimate human name, institution, or real entity.
                - **Phone**: If it looks like a phone number, ensure it makes sense (prefer Indonesian format 08xx if ambiguous).
                - **Address**: Must contain realistic geographic elements (street, city, etc.).
                - **Text/Content**: Must have coherent sentence structure and meaning.

            Output Requirements:
            - Respond ONLY with a raw JSON object. Do not include Markdown formatting (like ```json).
            - The "message" field MUST be in **INDONESIAN** (Bahasa Indonesia).
            - If invalid, the "message" should explain why it is rejected and suggest a correction.

            JSON Schema:
            {{
                "valid": true/false,
                "message": "Reason for validity or invalidity in Indonesian"
            }}
            "#,
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

pub fn extract_text_from_gemini(
    gemini_api_response: GeminiApiResponse,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {

    // Ambil teks dari candidate pertama → part pertama
    let model_generated_text_str: String = gemini_api_response
        .candidates
        .get(0)
        .and_then(|candidate| candidate.content.parts.get(0))
        .map(|part| part.text.clone())
        .ok_or_else(|| "Gagal mengekstrak teks dari respons LLM.".to_string())?;

    // Hapus block markdown seperti ```json ... ```
    let cleaned = clean_json_markdown(&model_generated_text_str);

    Ok(cleaned.to_string())
}
// */

// src/core_logic.rs
// tahap RED: implementasi minimal agar test bisa compile dan integrasi bisa jalan

// --- DEFINISI STRUKTUR DATA (Harus ada agar test bisa compile) ---

/*
// --- FUNGSI UTAMA (STUB / KOSONG) ---

// 1. Validasi Sintaksis (Lokal)
pub fn pre_validate_syntactically(_user_input: &str, _input_type_str: &str) -> Result<(), String> {
    // TAHAP RED:
    // Kita belum menulis Regex apapun.
    // Menggunakan todo!() akan membuat test crash -> RED.
    todo!("Validasi sintaksis belum dibuat")
}

// 2. Format Prompt
pub fn format_prompt(_user_input: &str, _input_type_str: &str) -> String {
    // TAHAP RED:
    // Return string kosong. Test yang mengecek apakah prompt mengandung kata "JSON" akan gagal.
    String::new()
}

// 3. Generate Body JSON
pub fn common_body_generation(_prompt: &str, _model_name: &str) -> serde_json::Value {
    // TAHAP RED:
    serde_json::json!({})
}

// 4. Parse Response
pub fn parse_gemini_response(
    _gemini_api_response: GeminiApiResponse,
) -> Result<ValidationResponse, Box<dyn std::error::Error + Send + Sync>> {
    // TAHAP RED:
    // Belum ada logika cleaning markdown atau parsing JSON.
    todo!("Logika parsing response belum diimplementasikan")
}

// --- FUNGSI INTEGRASI (ASYNC/SYNC) ---

pub async fn validate_input_with_llm_async(
    _user_input: &str,
    _model_name: &str,
    _input_type_str: &str,
    _gemini_api_key: &str,
) -> Result<ValidationResponse, Box<dyn std::error::Error + Send + Sync>> {
    // TAHAP RED:
    // Fungsi dipanggil test, tapi langsung panic.
    todo!("Fungsi validasi async belum diimplementasikan")
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "native_ffi_setup")]
pub fn validate_input_with_llm_sync(
    _user_input: &str,
    _model_name: &str,
    _input_type_str: &str,
    _config: &ApiConfig,
) -> Result<ValidationResponse, Box<dyn std::error::Error + Send + Sync>> {
    // TAHAP RED:
    todo!("Fungsi validasi sync belum diimplementasikan")
}
 */
//tahap GREEN: implementasi lengkap semua fungsi di atas agar test dan integrasi bisa jalan sukses
// src/core_logic.rs
/* 


// --- STRUKTUR DATA UTAMA ---



// --- IMPLEMENTASI LOGIKA UTAMA (MENGGANTIKAN TODO!) ---

// 1. Validasi Sintaksis (Regex)
pub fn pre_validate_syntactically(user_input: &str, input_type_str: &str) -> Result<(), String> {
    let input = user_input.trim();
    let input_type = input_type_str.trim().to_lowercase();

    if input.is_empty() {
        return Err("Input tidak boleh kosong.".to_string());
    }

    match input_type.as_str() {
        "alamat email" | "email" => {
            if input.len() > 254 {
                return Err("Alamat email terlalu panjang.".to_string());
            }
            // Regex sederhana untuk email
            static EMAIL_REGEX: Lazy<Regex> =
                Lazy::new(|| Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap());
            if !EMAIL_REGEX.is_match(input) {
                return Err("Format alamat email tidak valid.".to_string());
            }
        }
        "nama lengkap" | "nama" => {
             // Logic: Minimal ada 1 huruf alfabet
            if !input.chars().any(|c| c.is_alphabetic()) {
                return Err("Nama harus mengandung setidaknya satu huruf.".to_string());
            }
            if input.len() < 3 {
                 return Err("Nama terlalu pendek.".to_string());
            }
            if input.contains("  ") {
                return Err("Nama tidak boleh mengandung dua spasi berurutan.".to_string());
            }
        }
        "nomor hp indonesia" | "nomor hp" | "phone" | "no hp" | "mobile" => {
            // Regex: +628... atau 08... 
            // Diubah {7,12} menjadi {7,14} untuk mengakomodasi input 15 digit (08 + 13 angka)
            static PHONE_ID_REGEX: Lazy<Regex> =
                Lazy::new(|| Regex::new(r"^(\+62|0)8[0-9]{7,14}$").unwrap());
                
            if !PHONE_ID_REGEX.is_match(input) {
                return Err("Format nomor hp tidak valid (harus 08... atau +628... dan angka).".to_string());
            }
        }
        _ => {
            // Tipe lain lolos pre-validasi, lanjut ke LLM
        }
    }
    Ok(())
}

// 2. Format Prompt
pub fn format_prompt(user_input: &str, input_type_str: &str) -> String {
    let pre_note = "Validasi input berikut dalam format JSON (valid: boolean, message: string bahasa indonesia).";
    
    // Sederhanakan untuk contoh Green, gunakan format yang Anda miliki sebelumnya untuk logic lengkapnya
    // Di sini saya pastikan mengandung kata kunci yang dicek oleh Test
    format!(
        "{}\nInput: \"{}\"\nTipe: \"{}\"\nPastikan output JSON valid.",
        pre_note,
        user_input.replace("\"", "\\\""),
        input_type_str
    )
}

// 3. Generate Body JSON
pub fn common_body_generation(prompt: &str, model_name: &str) -> serde_json::Value {
    let mut body = serde_json::json!({
        "contents": [{ "parts": [{ "text": prompt }] }],
        "safetySettings": [
            { "category": "HARM_CATEGORY_HARASSMENT", "threshold": "BLOCK_NONE" },
            { "category": "HARM_CATEGORY_HATE_SPEECH", "threshold": "BLOCK_NONE" },
            { "category": "HARM_CATEGORY_SEXUALLY_EXPLICIT", "threshold": "BLOCK_NONE" },
            { "category": "HARM_CATEGORY_DANGEROUS_CONTENT", "threshold": "BLOCK_NONE" }
        ]
    });

    if model_name.starts_with("gemini") {
        if let Some(map) = body.as_object_mut() {
            map.insert(
                "generationConfig".to_string(),
                serde_json::json!({
                    "responseMimeType": "application/json"
                }),
            );
        }
    }
    body
}

// 4. Parse Response & Helper
fn clean_json_markdown(raw: &str) -> &str {
    raw.trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim()
}

pub fn parse_gemini_response(gemini_api_response: GeminiApiResponse) -> Result<ValidationResponse, Box<dyn std::error::Error + Send + Sync>> {
    let text = gemini_api_response.candidates.first()
        .and_then(|c| c.content.parts.first())
        .map(|p| &p.text)
        .ok_or("Empty response from LLM")?;

    let clean_text = clean_json_markdown(text);
    
    // Handle array response logic (seperti di test case)
    let json_val: serde_json::Value = serde_json::from_str(clean_text)?;
    
    let target_obj = if let Some(arr) = json_val.as_array() {
        arr.first().ok_or("Empty JSON array")?
    } else {
        &json_val
    };

    let resp: ValidationResponse = serde_json::from_value(target_obj.clone())?;
    Ok(resp)
}

// --- FUNGSI INTEGRASI ASYNC ---
pub async fn validate_input_with_llm_async(
    user_input: &str,
    model_name: &str,
    input_type_str: &str,
    gemini_api_key: &str,
) -> Result<ValidationResponse, Box<dyn std::error::Error + Send + Sync>> {
    // 1. Pre-validation
    if let Err(e) = pre_validate_syntactically(user_input, input_type_str) {
        return Ok(ValidationResponse { valid: false, message: e });
    }

    // 2. HTTP Request
    let client = AsyncClient::new();
    let url = format!(
        "[https://generativelanguage.googleapis.com/v1beta/models/](https://generativelanguage.googleapis.com/v1beta/models/){}:generateContent?key={}",
        model_name, gemini_api_key
    );
    
    let prompt = format_prompt(user_input, input_type_str);
    let body = common_body_generation(&prompt, model_name);

    let resp = client.post(&url).json(&body).send().await?;
    
    if !resp.status().is_success() {
        return Err(format!("API Error: {}", resp.status()).into());
    }

    let gemini_resp: GeminiApiResponse = resp.json().await?;
    parse_gemini_response(gemini_resp)
}

// --- FUNGSI INTEGRASI SYNC ---
#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "native_ffi_setup")]
pub fn validate_input_with_llm_sync(
    user_input: &str,
    model_name: &str,
    input_type_str: &str,
    config: &ApiConfig,
) -> Result<ValidationResponse, Box<dyn std::error::Error + Send + Sync>> {
     // 1. Pre-validation
    if let Err(e) = pre_validate_syntactically(user_input, input_type_str) {
        return Ok(ValidationResponse { valid: false, message: e });
    }

    let client = BlockingClient::new();
    let url = format!(
        "[https://generativelanguage.googleapis.com/v1beta/models/](https://generativelanguage.googleapis.com/v1beta/models/){}:generateContent?key={}",
        model_name, config.api_key
    );

    let prompt = format_prompt(user_input, input_type_str);
    let body = common_body_generation(&prompt, model_name);

    let resp = client.post(&url).json(&body).send()?;

    if !resp.status().is_success() {
        return Err(format!("API Error: {}", resp.status()).into());
    }

    // Perbaikan: gunakan text() lalu parse, karena struct GeminiApiResponse butuh struktur tepat
    let text_resp = resp.text()?;
    let gemini_resp: GeminiApiResponse = serde_json::from_str(&text_resp)?;
    
    parse_gemini_response(gemini_resp)
}

*/