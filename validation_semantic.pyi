# Ditempatkan di: .venv/Lib/site-packages/validation_semantic/__init__.pyi
# Atau di root source paket Anda agar bisa disertakan saat build oleh Maturin.

from typing import Any, Dict

class SupportedModel:
    # Mendefinisikan atribut kelas agar IDE tahu keberadaannya
    GeminiFlash: SupportedModel
    GeminiFlashLite: SupportedModel
    GeminiFlashLatest: SupportedModel
    Gemma: SupportedModel

    # Properti dan metode yang diekspos dari Rust
    @property
    def value(self) -> int: ...

    def __int__(self) -> int: ...
    def __repr__(self) -> str: ...
    # def __init__(self, ...) -> None: ... # Jika ada konstruktor Python

# Definisikan signature untuk fungsi Anda
def validate_input_py(text: str, model_selector: SupportedModel, input_type: str) -> Dict[str, Any]: ...

# Definisikan konstanta level modul
GEMINI_FLASH: int
GEMINI_FLASH_LITE: int
GEMINI_FLASH_LATEST: int
GEMMA: int

# Jika ada __doc__ atau __all__ yang ingin Anda definisikan secara eksplisit
# __doc__: str | None
# __all__: list[str]
