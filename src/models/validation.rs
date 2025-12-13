use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ValidationResponse {
    pub valid: bool,
    pub message: String,
}

// Untuk tahap RED, kita bisa menyederhanakan struct ValidationResponse
// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct ValidationResponse {
//     pub valid: bool,

//     #[serde(skip_serializing)]
//     pub message: String,
// }