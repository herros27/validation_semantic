// src/main.rs

use validation_semantic::{validate_text_ffi, free_rust_string, ValidationResponse, SupportedModel};
use std::ffi::{CString, CStr};
use std::os::raw::c_char;

fn main() {
    println!("=== Memulai Pengujian FFI dari main.rs dengan Pilihan Model (Enum) ===");

    let test_inputs = [
        // ("Kemas Khairunsyah.", "Nama Lengkap"),
        // ("Jl. Kapas No.9, Semaki, Kec. Umbulharjo, Kota Yogyakarta, DIY 55166", "Alamat Pengiriman"),
        // ("Universitas Ahmad Dahlan", "Nama Institusi"),
        // ("Irun2701", "Nama Pengguna"),
        ("asdsasd sdsadsad", "Nama Lengkap"),
        // ("2200018155@webmail.uad.ac.id", "Nama"),
        // ("Ini input dengan \"tanda kutip\" di dalamnya.", "Teks Umum"),
        // ("", "Input Kosong"), // Jenis input bisa "Input K
    ];

    let models_to_test = [
        SupportedModel::Gemini2Flash,
        // SupportedModel::Gemini2FlashLite,
        // SupportedModel::Gemini15Flash,
        // SupportedModel::Gemini15Pro, //LIMITTTT
        // Kita tidak bisa langsung menambahkan integer '99' di sini karena array ini bertipe SupportedModel.
        // Pengujian untuk integer tidak valid ditangani oleh lib.rs jika dipanggil dari C.
    ];

    // Iterasi untuk setiap varian model (LOOP LUAR)
    for &model_variant in &models_to_test { // Mengganti nama variabel agar lebih jelas
        let model_desc = format!("'{}' (Model variant: {:?})", model_variant.as_str(), model_variant);
        println!("\n\n=== Menguji dengan Model: {} ===", model_desc);

        // Iterasi untuk setiap kasus uji input (LOOP DALAM)
        for (i, (test_input_str, input_type_str)) in test_inputs.iter().enumerate() {
            println!("\n--- Kasus Uji Input {} (Model: {}): ---", i + 1, model_desc);
            println!("Input Teks: \"{}\"", test_input_str);

            let c_input_text = match CString::new(*test_input_str) {
                Ok(cs) => cs,
                Err(e) => {
                    eprintln!("Error membuat CString untuk input teks: '{}': {}", test_input_str, e);
                    continue;
                }
            };

            let c_input_type = match CString::new(*input_type_str) {
                Ok(cs) => cs,
                Err(e) => {
                    eprintln!("Error membuat CString untuk jenis input: '{}': {}", input_type_str, e);
                    continue;
                }
            };

            // Panggil fungsi FFI dengan varian enum SupportedModel secara langsung
            let result_ptr: *mut c_char = validate_text_ffi(c_input_text.as_ptr(), model_variant,c_input_type.as_ptr());

            if result_ptr.is_null() {
                eprintln!("validate_text_ffi mengembalikan pointer null!");
                continue;
            }

            let result_rust_string = unsafe {
                match CStr::from_ptr(result_ptr).to_str() {
                    Ok(s) => s.to_owned(),
                    Err(e) => {
                        eprintln!("Error konversi hasil C string ke Rust string: {}", e);
                        free_rust_string(result_ptr);
                        continue;
                    }
                }
            };

            println!("Hasil JSON dari FFI: {}", result_rust_string);
            match serde_json::from_str::<ValidationResponse>(&result_rust_string) {
                Ok(parsed_response) => {
                    println!("Parsed Response: valid={}, message='{}'", parsed_response.valid, parsed_response.message);
                }
                Err(e) => {
                    eprintln!("Gagal mem-parse JSON respons dari FFI: {}", e);
                }
            }
            free_rust_string(result_ptr);
            println!("Memori untuk hasil kasus uji input {} telah dibebaskan.", i + 1);
        }
        print!("===================================================\n\n");
    }

    // --- Pengujian Kasus Batas dengan Pointer NULL ---
    println!("\n\n=== Menguji Kasus Batas dengan Pointer NULL ===");
    let null_text_ptr: *const c_char = std::ptr::null();
    let valid_model_for_null_test = SupportedModel::Gemini15Flash; // Gunakan varian enum yang valid
    let example_input_type_str = "Contoh Jenis Input";
    let c_example_input_type = CString::new(example_input_type_str).unwrap();

    
    // 1. Teks NULL, Model VALID
    println!("\n--- Kasus: Teks Pointer NULL, Model VALID ({:?}) ---", valid_model_for_null_test);
    let result_ptr_null_text: *mut c_char = validate_text_ffi(null_text_ptr, valid_model_for_null_test, c_example_input_type.as_ptr());
    if !result_ptr_null_text.is_null() {
        let result_str = unsafe { CStr::from_ptr(result_ptr_null_text).to_str().unwrap_or_default() };
        println!("Hasil FFI (Teks NULL): {}", result_str);
        free_rust_string(result_ptr_null_text);
    } else {
        eprintln!("Hasil FFI (Teks NULL) adalah pointer NULL.");
    }

    // Pengujian untuk "Model Tidak Valid" dengan mengirim integer arbitrer seperti 99
    // tidak bisa langsung dilakukan dari main.rs jika FFI mengharapkan tipe SupportedModel.
    // Logika di lib.rs (match SupportedModel::from_int(model_selector as i32))
    // adalah untuk menangani kasus jika pemanggil C mengirim integer yang tidak valid.
    // Untuk menguji cabang error tersebut dari Rust, Anda perlu memanggil
    // `SupportedModel::from_int(99)` secara internal atau jika FFI menerima i32.
    // Jadi, kita tidak bisa secara langsung menyebabkan error "Invalid model selector value received: 99"
    // dari panggilan `validate_text_ffi` di main.rs ini jika parameternya adalah `SupportedModel`.
    // Kita percaya bahwa logika di lib.rs sudah menangani ini untuk pemanggil C.

    // Contoh jika Anda ingin menguji logika from_int secara terpisah (bukan via FFI langsung dengan integer salah):
    println!("\n--- Pengujian Internal Logika `SupportedModel::from_int` ---");
    let invalid_selector_int_test: i32 = 99;
    if let Some(model) = SupportedModel::from_int(invalid_selector_int_test) {
         println!("SupportedModel::from_int({}) -> {:?} -> {}", invalid_selector_int_test, model, model.as_str());
    } else {
         println!("SupportedModel::from_int({}) -> None (sesuai harapan untuk integer tidak valid)", invalid_selector_int_test);
    }


    println!("\n=== Pengujian FFI Selesai ===");
}