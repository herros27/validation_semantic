use validation_semantic::models::SupportedModel;

#[test]
fn test_supported_model_comprehensive() {
    println!("🧪 Testing SupportedModel enum");

    // Test as_str method
    let model_tests = vec![
        (SupportedModel::GeminiFlash, "gemini-2.5-flash"),
        (
            SupportedModel::GeminiFlashLite,
            "gemini-flash-lite-latest",
        ),
        (SupportedModel::GeminiFlashLatest, "gemini-flash-latest"),
        (SupportedModel::Gemma, "gemma-3-27b-it"),
    ];

    for (model, expected_str) in model_tests {
        let result = model.as_str();
        println!(
            "   {:?} -> '{}' (expected: '{}')",
            model, result, expected_str
        );
        assert_eq!(result, expected_str);
    }

    // Test from_int method
    let int_tests = vec![
        (0, Some(SupportedModel::GeminiFlash)),
        (1, Some(SupportedModel::GeminiFlashLite)),
        (2, Some(SupportedModel::GeminiFlashLatest)),
        (3, Some(SupportedModel::Gemma)),
        (99, None),
        (-1, None),
    ];

    for (int_val, expected_model) in int_tests {
        let result = SupportedModel::from_int(int_val);
        println!(
            "   from_int({}) -> {:?} (expected: {:?})",
            int_val, result, expected_model
        );
        assert_eq!(result, expected_model);
    }

    // Test valid_options_desc
    let desc = SupportedModel::valid_options_desc();
    println!("   valid_options_desc: {}", desc);
    assert!(desc.contains("GeminiFlash"));
    assert!(desc.contains("GeminiFlashLite"));
}
