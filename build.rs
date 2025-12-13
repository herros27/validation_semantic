use std::fs;
use std::path::Path;

/// Fungsi untuk mengubah string ALL_CAPS menjadi CamelCase
fn to_camel_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut c = word.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str().to_lowercase().as_str(),
            }
        })
        .collect()
}

fn main() {
    let out_dir = Path::new("."); // folder package Python
    fs::create_dir_all(out_dir).expect("Gagal membuat folder package");

    let pyi_path = out_dir.join("validation_semantic.pyi");

    let supported_models = vec!["GEMINI_FLASH", "GEMINI_FLASH_LITE", "GEMINI_FLASH_LATEST", "GEMMA"];

    // Generate atribut kelas SupportedModel dengan CamelCase
    let model_attrs = supported_models
        .iter()
        .map(|m| format!("    {}: SupportedModel", to_camel_case(m)))
        .collect::<Vec<_>>()
        .join("\n");

    // Generate konstanta level modul tetap sesuai Rust
    let module_consts = supported_models
        .iter()
        .map(|m| format!("{}: int", m))
        .collect::<Vec<_>>()
        .join("\n");

    let pyi_content = format!(
        r#"# Ditempatkan di: .venv/Lib/site-packages/validation_semantic/__init__.pyi
# Atau di root source paket Anda agar bisa disertakan saat build oleh Maturin.

from typing import Any, Dict

class SupportedModel:
    # Mendefinisikan atribut kelas agar IDE tahu keberadaannya
{model_attrs}

    # Properti dan metode yang diekspos dari Rust
    @property
    def value(self) -> int: ...

    def __int__(self) -> int: ...
    def __repr__(self) -> str: ...
    # def __init__(self, ...) -> None: ... # Jika ada konstruktor Python

# Definisikan signature untuk fungsi Anda
def validate_input_py(text: str, model_selector: SupportedModel, input_type: str) -> Dict[str, Any]: ...

# Definisikan konstanta level modul
{module_consts}

# Jika ada __doc__ atau __all__ yang ingin Anda definisikan secara eksplisit
# __doc__: str | None
# __all__: list[str]
"#,
        model_attrs = model_attrs,
        module_consts = module_consts,
    );

    fs::write(&pyi_path, pyi_content).expect("Gagal menulis file __init__.pyi");

    println!("cargo:rerun-if-changed=build.rs");
    println!("✅ Generated {}", pyi_path.display());


}
