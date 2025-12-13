use validation_semantic::models::ValidationResponse;

#[test]
fn test_validation_response_serialization() {
    let cases = vec![
        ValidationResponse { valid: true, message: "OK".into() },
        ValidationResponse { valid: false, message: "Error".into() },
    ];

    for response in cases {
        let json = serde_json::to_string(&response).unwrap();
        let parsed: ValidationResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response.valid, parsed.valid);
        assert_eq!(response.message, parsed.message);
    }
}
