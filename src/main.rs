// src/main.rs

use validation_semantic::{validate_text_ffi, free_rust_string, ValidationResponse, SupportedModel};
use std::ffi::{CString, CStr};
use std::os::raw::c_char;

fn main() {
    println!("🚀 Memulai Pengujian FFI Validation Semantic");
    println!("============================================================");

    let test_inputs = [
        ("Kemas Khairunsyah", "Nama Lengkap"),
        // ("Kemas Khairunsyah.", "Nama Lengkap"),
        // ("Jl. Kapas No.9, Semaki, Kec. Umbulharjo, Kota Yogyakarta, DIY 55166", "Alamat Pengiriman"),
        // ("Universitas Ahmad Dahlan", "Nama Institusi"),
        // ("Irun2701", "Nama Pengguna"),
        // ("2200018155@webmail.uad.ac.id", "Email"),
        ("Loren ipsum dolor sit amet", "Teks Umum"),
        // ("", "Input Kosong"),
    ];

    let models_to_test = [
        // SupportedModel::Gemini25Flash,
        SupportedModel::Gemini25FlashLite,
        // SupportedModel::Gemini15Flash,
        // SupportedModel::Gemini25Pro,
    ];

    // Iterasi untuk setiap model
    for &model_variant in &models_to_test {
        let model_name = model_variant.as_str();
        println!("\n📋 Menguji Model: {}", model_name);
        println!("----------------------------------------");

        // Iterasi untuk setiap kasus uji input
        for (i, (test_input_str, input_type_str)) in test_inputs.iter().enumerate() {
            println!("\n🔍 Test Case {}: {}", i + 1, input_type_str);
            println!("   Input: \"{}\"", test_input_str);

            let c_input_text = match CString::new(*test_input_str) {
                Ok(cs) => cs,
                Err(e) => {
                    eprintln!("❌ Error membuat CString untuk input: {}", e);
                    continue;
                }
            };

            let c_input_type = match CString::new(*input_type_str) {
                Ok(cs) => cs,
                Err(e) => {
                    eprintln!("❌ Error membuat CString untuk tipe input: {}", e);
                    continue;
                }
            };

            // Panggil fungsi FFI
            let result_ptr: *mut c_char = validate_text_ffi(
                c_input_text.as_ptr(), 
                model_variant,
                c_input_type.as_ptr()
            );

            if result_ptr.is_null() {
                eprintln!("❌ FFI mengembalikan pointer null!");
                continue;
            }

            let result_rust_string = unsafe {
                match CStr::from_ptr(result_ptr).to_str() {
                    Ok(s) => s.to_owned(),
                    Err(e) => {
                        eprintln!("❌ Error konversi hasil ke string: {}", e);
                        free_rust_string(result_ptr);
                        continue;
                    }
                }
            };

            // Parse dan tampilkan hasil
            match serde_json::from_str::<ValidationResponse>(&result_rust_string) {
                Ok(parsed_response) => {
                    let status_icon = if parsed_response.valid { "✅" } else { "❌" };
                    println!("   {} Valid: {}", status_icon, parsed_response.valid);
                    println!("   📝 Message: {}", parsed_response.message);
                }
                Err(e) => {
                    eprintln!("❌ Gagal parse JSON: {}", e);
                    println!("   Raw response: {}", result_rust_string);
                }
            }
            
            free_rust_string(result_ptr);
        }
    }

    // Pengujian kasus batas
    println!("\n🧪 Pengujian Kasus Batas");
    println!("============================================");
    
    let null_text_ptr: *const c_char = std::ptr::null();
    let valid_model_for_null_test = SupportedModel::Gemini15Flash;
    let example_input_type_str = "Contoh Jenis Input";
    let c_example_input_type = CString::new(example_input_type_str).unwrap();

    // Test dengan teks NULL
    println!("\n🔍 Test Case: Teks NULL");
    let result_ptr_null_text: *mut c_char = validate_text_ffi(
        null_text_ptr, 
        valid_model_for_null_test, 
        c_example_input_type.as_ptr()
    );
    
    if !result_ptr_null_text.is_null() {
        let result_str = unsafe { 
            CStr::from_ptr(result_ptr_null_text).to_str().unwrap_or_default() 
        };
        println!("   📝 Hasil: {}", result_str);
        free_rust_string(result_ptr_null_text);
    } else {
        println!("   ❌ Hasil: pointer NULL");
    }

    // Test internal logic
    println!("\n🔍 Test Internal Logic");
    let invalid_selector_int_test: i32 = 99;
    match SupportedModel::from_int(invalid_selector_int_test) {
        Some(model) => {
            println!("   ⚠️  from_int({}) -> {:?} (tidak diharapkan)", 
                invalid_selector_int_test, model);
        }
        None => {
            println!("   ✅ from_int({}) -> None (sesuai harapan)", 
                invalid_selector_int_test);
        }
    }

    println!("\n🎉 Pengujian FFI Selesai");
    println!("============================================================");
}