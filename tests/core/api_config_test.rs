
use validation_semantic::config::API_CONFIG;
#[test]
fn test_api_config_initialization() {
    println!("🧪 Testing API_CONFIG initialization");

    match &*API_CONFIG {
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
