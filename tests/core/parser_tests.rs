use validation_semantic::core::{
    parse_gemini_response,
   
};

use validation_semantic::models::{
    GeminiApiContent, GeminiApiPart,
    GeminiApiResponse, 
    GeminiApiResponseCandidate,
};

#[test]
fn test_parse_gemini_response_object() {
    let response = GeminiApiResponse {
        candidates: vec![GeminiApiResponseCandidate {
            content: GeminiApiContent {
                parts: vec![GeminiApiPart {
                    text: r#"{"valid": true, "message": "OK"}"#.to_string(),
                }],
            },
        }],
    };

    let parsed = parse_gemini_response(response).unwrap();
    assert!(parsed.valid);
    assert_eq!(parsed.message, "OK");
}

#[test]
fn test_parse_gemini_response_array() {
    let response = GeminiApiResponse {
        candidates: vec![GeminiApiResponseCandidate {
            content: GeminiApiContent {
                parts: vec![GeminiApiPart {
                    text: r#"[{"valid": false, "message": "Invalid"}]"#.to_string(),
                }],
            },
        }],
    };

    let parsed = parse_gemini_response(response).unwrap();
    assert!(!parsed.valid);
}

#[test]
fn test_parse_gemini_response_invalid_json() {
    let response = GeminiApiResponse {
        candidates: vec![GeminiApiResponseCandidate {
            content: GeminiApiContent {
                parts: vec![GeminiApiPart {
                    text: "invalid json".to_string(),
                }],
            },
        }],
    };

    assert!(parse_gemini_response(response).is_err());
}
