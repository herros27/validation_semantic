use validation_semantic::models::SupportedModel;

use validation_semantic::core::common_body_generation;
#[test]
fn test_common_body_generation() {
    println!("🧪 Testing common_body_generation function");

    let test_prompt = "Test prompt for validation";
    let model_name = SupportedModel::GeminiFlash.as_str();
    let body = common_body_generation(test_prompt, model_name);

    println!("   Generated body structure:");
    println!(
        "   - contents array length: {}",
        body["contents"].as_array().unwrap().len()
    );
    println!(
        "   - safety settings count: {}",
        body["safetySettings"].as_array().unwrap().len()
    );
    println!(
        "   - response MIME type: {}",
        body["generationConfig"]["responseMimeType"]
    );

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
    assert_eq!(
        body["generationConfig"]["responseMimeType"],
        "application/json"
    );
}