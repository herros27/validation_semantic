#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportedModel {
    GeminiFlash = 0,
    GeminiFlashLite = 1,
    GeminiFlashLatest = 2,
    Gemma = 3,
}


impl SupportedModel {
    pub fn as_str(&self) -> &'static str {
        match self {
            SupportedModel::GeminiFlash => "gemini-2.5-flash",
            SupportedModel::GeminiFlashLite => "gemini-flash-lite-latest",
            SupportedModel::GeminiFlashLatest => "gemini-flash-latest",
            SupportedModel::Gemma => "gemma-3-27b-it",
        }
    }
    pub fn from_int(value: i32) -> Option<Self> {
        match value {
            0 => Some(SupportedModel::GeminiFlash),
            1 => Some(SupportedModel::GeminiFlashLite),
            2 => Some(SupportedModel::GeminiFlashLatest),
            3 => Some(SupportedModel::Gemma),
            _ => None,
        }
    }
    pub fn valid_options_desc() -> String {
         format!(
            "0 (GeminiFlash), 1 (GeminiFlashLite), 2 (GeminiFlashLatest), 3 (Gemma)"
        )
    }
}


// RED

// impl SupportedModel {
//     pub fn as_str(&self) -> &'static str {
//         // TAHAP RED: Belum ada mapping string, return string kosong atau panic
//         todo!("Mapping model belum diimplementasikan")
//     }

//     pub fn from_int(_value: i32) -> Option<Self> {
//         // TAHAP RED: Belum ada logika konversi
//         todo!("Konversi int ke model belum diimplementasikan")
//     }

//     pub fn valid_options_desc() -> String {
//         String::from("Options not available")
//     }
// }