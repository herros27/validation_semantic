use validation_semantic::core::format_prompt;

#[test]
fn test_format_prompt_comprehensive() {
    println!("🧪 Testing format_prompt function");

    let prompt_tests = vec![
        ("test@example.com", "email"),
        ("John Doe", "nama"),
        ("08123456789", "nomor hp indonesia"),
        ("random text", "teks umum"),
    ];

    for (input, input_type) in prompt_tests {
        let prompt = format_prompt(input, input_type);
        println!(
            "   {}: '{}' -> prompt length: {}",
            input_type,
            input,
            prompt.len()
        );

        // Verify prompt contains expected elements
        assert!(prompt.to_lowercase().contains("validate") || prompt.to_lowercase().contains("validasi"));
        assert!(prompt.contains(input));
        assert!(prompt.to_lowercase().contains("json"));
        assert!(prompt.to_lowercase().contains("valid"));
        assert!(prompt.to_lowercase().contains("message"));
    }
}

