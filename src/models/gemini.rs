
use serde::{Deserialize};

// Struct untuk parsing respons Gemini API
#[derive(Debug, Deserialize)]
pub struct GeminiApiPart {
    pub text: String,
}
#[derive(Debug, Deserialize)]
pub struct GeminiApiContent {
    pub parts: Vec<GeminiApiPart>,
}
#[derive(Debug, Deserialize)]
pub struct GeminiApiResponseCandidate {
    pub content: GeminiApiContent,
}
#[derive(Debug, Deserialize)]
pub struct GeminiApiResponse {
    pub candidates: Vec<GeminiApiResponseCandidate>,
}

// red
// Struct pendukung API (Hanya definisi, tanpa logic)
// #[derive(Debug, Deserialize)]
// pub struct GeminiApiPart { pub text: String }
// #[derive(Debug, Deserialize)]
// pub struct GeminiApiContent { pub parts: Vec<GeminiApiPart> }
// #[derive(Debug, Deserialize)]
// pub struct GeminiApiResponseCandidate { pub content: GeminiApiContent }
// #[derive(Debug, Deserialize)]
// pub struct GeminiApiResponse { pub candidates: Vec<GeminiApiResponseCandidate> }