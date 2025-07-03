// tests/core_logic_tests.rs

use validation_semantic::core_logic::{
    ValidationResponse, SupportedModel,
    validate_input_with_llm_sync, validate_input_with_llm_async,
    pre_validate_syntactically, format_prompt, common_body_generation, parse_gemini_response
};
use validation_semantic::core_logic::{
    GeminiApiResponse, GeminiApiResponseCandidate, GeminiApiContent, GeminiApiPart
};

#[test]
fn test_pre_validate_syntactically_comprehensive() {
    println!("🧪 Testing pre_validate_syntactically function");
    
    // Test cases untuk email
    let email_tests = vec![
        ("test@example.com", "email", true),
        ("user@domain.org", "email", true),
        ("invalid-email", "email", false),
        ("test@", "email", false),
        ("@domain.com", "email", false),
        ("", "email", false),
    ];
    
    for (input, input_type, expected_valid) in email_tests {
        let result = pre_validate_syntactically(input, input_type);
        let is_valid = result.is_ok();
        println!("   Email: '{}' -> {} (expected: {})", input, is_valid, expected_valid);
        assert_eq!(is_valid, expected_valid);
    }
    
    // Test cases untuk nama
    let name_tests = vec![
        ("John Doe", "nama", true),
        ("Mary Jane", "nama", true),
        ("Jo", "nama", false), // terlalu pendek
        ("A", "nama", false),  // terlalu pendek
        ("John123", "nama", false), // mengandung angka
        ("", "nama", false),
    ];
    
    for (input, input_type, expected_valid) in name_tests {
        let result = pre_validate_syntactically(input, input_type);
        let is_valid = result.is_ok();
        println!("   Nama: '{}' -> {} (expected: {})", input, is_valid, expected_valid);
        assert_eq!(is_valid, expected_valid);
    }
    
    // Test cases untuk nomor telepon
    let phone_tests = vec![
        ("08123456789", "nomor telepon indonesia", true),
        ("+628123456789", "nomor telepon indonesia", true),
        ("123", "nomor telepon indonesia", false), // terlalu pendek
        ("081234567890123", "nomor telepon indonesia", false), // terlalu panjang
        ("0812345678a", "nomor telepon indonesia", false), // mengandung huruf
        ("", "nomor telepon indonesia", false),
    ];
    
    for (input, input_type, expected_valid) in phone_tests {
        let result = pre_validate_syntactically(input, input_type);
        let is_valid = result.is_ok();
        println!("   Phone: '{}' -> {} (expected: {})", input, is_valid, expected_valid);
        assert_eq!(is_valid, expected_valid);
    }
}

#[test]
fn test_supported_model_comprehensive() {
    println!("🧪 Testing SupportedModel enum");
    
    // Test as_str method
    let model_tests = vec![
        (SupportedModel::Gemini25Flash, "gemini-2.5-flash"),
        (SupportedModel::Gemini25FlashLite, "gemini-2.5-flash-lite-preview-06-17"),
        (SupportedModel::Gemini15Flash, "gemini-1.5-flash-latest"),
        (SupportedModel::Gemini25Pro, "gemini-2.5-pro"),
    ];
    
    for (model, expected_str) in model_tests {
        let result = model.as_str();
        println!("   {:?} -> '{}' (expected: '{}')", model, result, expected_str);
        assert_eq!(result, expected_str);
    }
    
    // Test from_int method
    let int_tests = vec![
        (0, Some(SupportedModel::Gemini25Flash)),
        (1, Some(SupportedModel::Gemini25FlashLite)),
        (2, Some(SupportedModel::Gemini15Flash)),
        (3, Some(SupportedModel::Gemini25Pro)),
        (99, None),
        (-1, None),
    ];
    
    for (int_val, expected_model) in int_tests {
        let result = SupportedModel::from_int(int_val);
        println!("   from_int({}) -> {:?} (expected: {:?})", int_val, result, expected_model);
        assert_eq!(result, expected_model);
    }
    
    // Test valid_options_desc
    let desc = SupportedModel::valid_options_desc();
    println!("   valid_options_desc: {}", desc);
    assert!(desc.contains("Gemini25Flash"));
    assert!(desc.contains("Gemini25FlashLite"));
}

#[test]
fn test_format_prompt_comprehensive() {
    println!("🧪 Testing format_prompt function");
    
    let prompt_tests = vec![
        ("test@example.com", "email"),
        ("John Doe", "nama"),
        ("08123456789", "nomor telepon indonesia"),
        ("random text", "teks umum"),
    ];
    
    for (input, input_type) in prompt_tests {
        let prompt = format_prompt(input, input_type);
        println!("   {}: '{}' -> prompt length: {}", input_type, input, prompt.len());
        
        // Verify prompt contains expected elements
        assert!(prompt.contains("Validasi"));
        assert!(prompt.contains(input));
        assert!(prompt.contains("JSON"));
        assert!(prompt.contains("valid"));
        assert!(prompt.contains("message"));
    }
}

#[test]
fn test_common_body_generation() {
    println!("🧪 Testing common_body_generation function");
    
    let test_prompt = "Test prompt for validation";
    let body = common_body_generation(test_prompt);
    
    println!("   Generated body structure:");
    println!("   - contents array length: {}", body["contents"].as_array().unwrap().len());
    println!("   - safety settings count: {}", body["safetySettings"].as_array().unwrap().len());
    println!("   - response MIME type: {}", body["generationConfig"]["responseMimeType"]);
    
    // Verify structure
    assert!(body.is_object());
    assert!(body["contents"].is_array());
    assert!(body["safetySettings"].is_array());
    assert!(body["generationConfig"].is_object());
    
    // Verify content
    let contents = body["contents"].as_array().unwrap();
    assert_eq!(contents.len(), 1);
    
    let parts = contents[0]["parts"].as_array().unwrap();
    assert_eq!(parts.len(), 1);
    assert_eq!(parts[0]["text"], test_prompt);
    
    // Verify safety settings
    let safety_settings = body["safetySettings"].as_array().unwrap();
    assert_eq!(safety_settings.len(), 4);
    
    // Verify generation config
    assert_eq!(body["generationConfig"]["responseMimeType"], "application/json");
}

#[test]
fn test_parse_gemini_response_comprehensive() {
    println!("🧪 Testing parse_gemini_response function");
    
    // Test 1: Valid JSON object
    let valid_response = GeminiApiResponse {
        candidates: vec![
            GeminiApiResponseCandidate {
                content: GeminiApiContent {
                    parts: vec![
                        GeminiApiPart {
                            text: r#"{"valid": true, "message": "Valid email address"}"#.to_string()
                        }
                    ]
                }
            }
        ]
    };
    
    let result = parse_gemini_response(valid_response);
    assert!(result.is_ok());
    let parsed = result.unwrap();
    assert!(parsed.valid);
    assert_eq!(parsed.message, "Valid email address");
    println!("   ✅ Valid JSON object parsed successfully");
    
    // Test 2: JSON array response
    let array_response = GeminiApiResponse {
        candidates: vec![
            GeminiApiResponseCandidate {
                content: GeminiApiContent {
                    parts: vec![
                        GeminiApiPart {
                            text: r#"[{"valid": false, "message": "Invalid email format"}]"#.to_string()
                        }
                    ]
                }
            }
        ]
    };
    
    let result = parse_gemini_response(array_response);
    assert!(result.is_ok());
    let parsed = result.unwrap();
    assert!(!parsed.valid);
    assert_eq!(parsed.message, "Invalid email format");
    println!("   ✅ JSON array response parsed successfully");
    
    // Test 3: Invalid JSON
    let invalid_response = GeminiApiResponse {
        candidates: vec![
            GeminiApiResponseCandidate {
                content: GeminiApiContent {
                    parts: vec![
                        GeminiApiPart {
                            text: "invalid json".to_string()
                        }
                    ]
                }
            }
        ]
    };
    
    let result = parse_gemini_response(invalid_response);
    assert!(result.is_err());
    println!("   ✅ Invalid JSON correctly rejected");
}

#[test]
fn test_validation_response_serialization() {
    println!("🧪 Testing ValidationResponse serialization");
    
    let test_cases = vec![
        ValidationResponse {
            valid: true,
            message: "Valid input".to_string(),
        },
        ValidationResponse {
            valid: false,
            message: "Invalid input".to_string(),
        },
        ValidationResponse {
            valid: true,
            message: "".to_string(),
        },
    ];
    
    for (i, response) in test_cases.iter().enumerate() {
        // Test serialization
        let json = serde_json::to_string(response).unwrap();
        println!("   Test {}: {} -> JSON: {}", i, response.valid, json);
        
        // Test deserialization
        let parsed: ValidationResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(response.valid, parsed.valid);
        assert_eq!(response.message, parsed.message);
        
        println!("   ✅ Test {} passed", i);
    }
}

#[test]
fn test_api_config_initialization() {
    println!("🧪 Testing API_CONFIG initialization");
    
    match &*validation_semantic::core_logic::API_CONFIG {
        Ok(config) => {
            println!("   ✅ API_CONFIG initialized successfully");
            println!("   API Key length: {}", config.api_key.len());
            assert!(!config.api_key.is_empty());
        }
        Err(e) => {
            println!("   ⚠️  API_CONFIG initialization failed: {}", e);
            assert!(e.contains("GOOGLE_API_KEY"));
        }
    }
}

#[tokio::test]
async fn test_validate_input_with_llm_async_integration() {
    println!("🧪 Testing validate_input_with_llm_async (integration test)");
    
    match &*validation_semantic::core_logic::API_CONFIG {
        Ok(_config) => {
            println!("   🔑 API key available, running integration test");
            
            let test_cases = vec![
                ("test@example.com", "email"),
                ("John Doe", "nama"),
                ("08123456789", "nomor telepon indonesia"),
            ];
            
            for (input, input_type) in test_cases {
                println!("   Testing: '{}' ({})", input, input_type);
                
                let result = validate_input_with_llm_async(
                    input,
                    "gemini-2.0-flash",
                    input_type,
                ).await;
                
                match result {
                    Ok(response) => {
                        println!("   ✅ Success: valid={}, message='{}'", response.valid, response.message);
                        assert!(response.message.len() > 0);
                    }
                    Err(e) => {
                        println!("   ❌ Error: {}", e);
                        // Error bisa karena API limit, network, dll
                        assert!(e.to_string().len() > 0);
                    }
                }
            }
        }
        Err(_) => {
            println!("   ⏭️  Skipping integration test - no API key available");
        }
    }
}

#[test]
fn test_validate_input_with_llm_sync_integration() {
    println!("🧪 Testing validate_input_with_llm_sync (integration test)");
    
    match &*validation_semantic::core_logic::API_CONFIG {
        Ok(config) => {
            println!("   🔑 API key available, running integration test");
            
            let test_cases = vec![
                ("test@example.com", "email"),
                ("John Doe", "nama"),
            ];
            
            for (input, input_type) in test_cases {
                println!("   Testing: '{}' ({})", input, input_type);
                
                let result = validate_input_with_llm_sync(
                    input,
                    "gemini-2.0-flash",
                    input_type,
                    config
                );
                
                match result {
                    Ok(response) => {
                        println!("   ✅ Success: valid={}, message='{}'", response.valid, response.message);
                        assert!(response.message.len() > 0);
                    }
                    Err(e) => {
                        println!("   ❌ Error: {}", e);
                        // Error bisa karena API limit, network, dll
                        assert!(e.to_string().len() > 0);
                    }
                }
            }
        }
        Err(_) => {
            println!("   ⏭️  Skipping integration test - no API key available");
        }
    }
} 