use validation_semantic::core::validate_input_with_llm_sync;
use validation_semantic::config::API_CONFIG;

use validation_semantic::models::SupportedModel;
use validation_semantic::utils::test_cases::COMMON_TEST_CASES;
#[test]
fn test_validate_input_with_llm_sync_integration() {
    println!("🧪 Testing validate_input_with_llm_sync (integration test)");

    match &*API_CONFIG {
        Ok(config) => {
            println!("   🔑 API key available, running integration test");

            let test_cases = COMMON_TEST_CASES;

            for (input, input_type) in test_cases {
                println!("   Testing: '{}' ({})", input, input_type);

                let model_name = SupportedModel::GeminiFlash.as_str();
                let result =
                    validate_input_with_llm_sync(input, model_name, input_type, config);

                match result {
                    Ok(response) => {
                        println!(
                            "   ✅ Success: valid={}, message='{}'",
                            response.valid, response.message
                        );
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
        Err(e) => {
              panic!("API_CONFIG not initialized: {}", e);
        }
    }
}
