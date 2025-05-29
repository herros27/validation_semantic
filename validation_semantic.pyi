# Ditempatkan di: .venv/Lib/site-packages/validation_semantic/__init__.pyi
# Atau di root source paket Anda agar bisa disertakan saat build oleh Maturin.

from typing import Any, Dict # Impor Any dan Dict jika digunakan

class SupportedModel:
    # Mendefinisikan atribut kelas agar IDE tahu keberadaannya
    # Ini adalah instance dari kelas SupportedModel itu sendiri
    GEMINI_2_FLASH: SupportedModel
    GEMINI_2_FLASH_LITE: SupportedModel
    GEMINI_1_5_FLASH: SupportedModel
    GEMINI_1_5_PRO: SupportedModel

    # Properti dan metode yang diekspos dari Rust
    @property
    def value(self) -> int: ... # '...' berarti implementasinya ada di kode native

    def __int__(self) -> int: ...
    def __repr__(self) -> str: ...
    # def __init__(self, ...) -> None: ... # Jika ada konstruktor Python

# Definisikan signature untuk fungsi Anda
# Tipe kembalian adalah dict karena PyDict dikonversi ke dict Python
def validate_text_py(text: str, model_selector: SupportedModel,input_type: str) -> Dict[str, Any]: ...
                                                                      # Menggunakan Dict[str, Any] lebih umum
                                                                      # atau Dict[str, object] juga bisa

# Definisikan konstanta level modul jika Anda juga mengeksposnya dari Rust
# (Ini adalah integer yang Anda tambahkan ke modul di fungsi register_items_for_python_module)
MODEL_GEMINI_2_FLASH: int
MODEL_GEMINI_2_FLASH_LITE: int
MODEL_GEMINI_1_5_FLASH: int
MODEL_GEMINI_1_5_PRO: int

# Jika ada __doc__ atau __all__ yang ingin Anda definisikan secara eksplisit untuk type checker
# __doc__: str | None
# __all__: list[str]