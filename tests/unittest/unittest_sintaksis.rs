
use validation_semantic::core::{
    pre_validate_syntactically,
    
};
use validation_semantic::utils::test_cases::{
    COMMON_EMAIL_TESTS, COMMON_NAME_TESTS, COMMON_PHONE_TESTS
};

#[test]
fn test_pre_validate_syntactically_comprehensive() {
    println!("🧪 Testing pre_validate_syntactically function");

    // Test cases untuk email
    let email_tests = COMMON_EMAIL_TESTS;

    for &(input, input_type, expected_valid) in email_tests {
        let result = pre_validate_syntactically(input, input_type);
        let is_valid = result.is_ok();
        println!(
            "   Email: '{}' -> {} (expected: {})",
            input, is_valid, expected_valid
        );
        assert_eq!(is_valid, expected_valid);
    }

    // Test cases untuk nama
    let name_tests = COMMON_NAME_TESTS;

    for &(input, input_type, expected_valid) in name_tests {
        let result = pre_validate_syntactically(input, input_type);
        let is_valid = result.is_ok();
        println!(
            "   Nama: '{}' -> {} (expected: {})",
            input, is_valid, expected_valid
        );
        assert_eq!(is_valid, expected_valid);
    }

    // Test cases untuk nomor hp
    let phone_tests = COMMON_PHONE_TESTS;

    for &(input, input_type, expected_valid) in phone_tests {
        let result = pre_validate_syntactically(input, input_type);
        let is_valid = result.is_ok();
        println!(
            "   Phone: '{}' -> {} (expected: {})",
            input, is_valid, expected_valid
        );
        assert_eq!(is_valid, expected_valid);
    }
}