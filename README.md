# 🤖 `semantic_validation`

![GitHub last commit](https://img.shields.io/github/last-commit/herros27/validation_semantic)
![GitHub stars](https://img.shields.io/github/stars/herros27/validation_semantic?style=social)
![Crates.io](https://img.shields.io/crates/v/semantic_validation)
![Rust Build Status](https://github.com/herros27/validation_semantic/actions/workflows/rust.yml/badge.svg)

`semantic_validation` adalah *library* **validasi semantik** yang tangguh dan berkinerja tinggi, ditulis sepenuhnya dalam **Rust**. Fokus utamanya adalah menyediakan **core logic** yang andal untuk memastikan data Anda tidak hanya valid secara format, tetapi juga **bermakna dan konsisten** sesuai dengan aturan bisnis yang Anda tetapkan.

Kekuatan utama *library* ini terletak pada kemampuannya untuk diintegrasikan ke berbagai *platform* melalui **bindings**. Ini berarti Anda bisa memanfaatkan kecepatan dan keamanan Rust di lingkungan seperti **WebAssembly (WASM)** untuk *browser*, **Python (menggunakan PyO3/Maturin)** untuk *backend* atau *data science*, dan *platform* lainnya di masa mendatang.

---

## 🌟 Fitur Utama

* **Core Logic dalam Rust:** Mesin validasi semantik yang cepat, aman, dan efisien, dibangun di atas fondasi Rust.
* **Validasi Berbasis Aturan:** Terapkan seperangkat aturan yang dapat dikonfigurasi untuk memeriksa integritas dan konsistensi semantik data Anda.
* **Deteksi Anomali:** Mudah mengidentifikasi pola atau nilai data yang tidak sesuai dengan ekspektasi semantik Anda.
* **API yang Fleksibel:** Dirancang untuk mudah diekspos melalui *bindings* ke berbagai bahasa dan lingkungan pemrograman.
* **Laporan Detail:** Dapatkan laporan validasi yang jelas dan informatif, menunjukkan secara spesifik di mana letak masalah semantik dan mengapa.
* **Siap untuk *Cross-Platform*:** Dibuat dengan mempertimbangkan penggunaan di lingkungan *server*, *desktop*, dan bahkan *browser*.

---

## 🚀 Memulai

Ikuti langkah-langkah ini untuk mengkompilasi dan menggunakan *core library* Rust secara lokal.

## ⚙️ Instalasi

Pastikan Anda memiliki Python 3.7 atau versi yang lebih baru.

1.  **Instal library ini** (Asumsi jika sudah di-publish ke PyPI):
    ```bash
    pip install -i https://test.pypi.org/simple/ validation-semantic
    ```
    Jika belum, Anda bisa menginstalnya dari direktori lokal:
    ```bash
    pip install .
    ```

2.  **Instal dependensi** yang dibutuhkan oleh library ini, yaitu Google Generative AI:
    ```bash
    pip install google-generativeai
    ```

---

## 🔑 Konfigurasi

Library ini memerlukan API Key dari Google AI Studio untuk dapat berinteraksi dengan model Gemini.

1.  **Dapatkan API Key Anda**: Kunjungi [Google AI Studio](https://aistudio.google.com/app/apikey) untuk membuat API Key baru.

2.  **Atur Environment Variable**: Cara paling aman untuk menggunakan API Key adalah dengan menyimpannya sebagai *environment variable*.
    
    -   **Untuk Linux/macOS**:
        ```bash
        export GEMINI_API_KEY="API_KEY_ANDA_DISINI"
        ```
    
    -   **Untuk Windows (Command Prompt)**:
        ```bash
        set GEMINI_API_KEY="API_KEY_ANDA_DISINI"
        ```
    
    Library ini akan secara otomatis mendeteksi dan menggunakan *environment variable* dengan nama `GEMINI_API_KEY`.

---

## 🚀 Cara Penggunaan

Penggunaan library ini sangat mudah. Anda hanya perlu mengimpor fungsi `validate_input_py` dan enum `SupportedModel`.

### Contoh Kode Sederhana

Berikut adalah contoh untuk memvalidasi sebuah input yang seharusnya merupakan nama perusahaan.

```python
import json
from validation_semantic import validate_input_py, SupportedModel

# 1. Tentukan input yang ingin divalidasi
text_input = "PT Mencari Cinta Sejati"
input_type = "Nama Perusahaan" # Label ini harus sesuai dengan yang dipahami model

# 2. Pilih model yang akan digunakan
# Pilihan: GeminiFlash, GeminiFlashLite, GeminiFlashLatest, Gemma
model_choice = SupportedModel.GeminiFlash

try:
    # 3. Panggil fungsi validasi
    result = validate_input_py(
        text=text_input,
        model=model_choice,
        label=input_type
    )

    # 4. Tampilkan hasil
    print(f"Input: '{text_input}'")
    print(f"Jenis: '{input_type}'")
    print("-" * 20)
    # Gunakan json.dumps untuk pretty print
    print(json.dumps(result, indent=4, ensure_ascii=False))

except Exception as e:
    print(f"Terjadi kesalahan: {e}")

```

### Memahami Hasil

Fungsi `validate_input_py` akan mengembalikan sebuah dictionary (yang dapat di-serialisasi ke JSON) dengan struktur sebagai berikut:

```json
{
    "valid": true,
    "message": "Input 'PT Mencari Cinta Sejati' adalah nama perusahaan yang valid dan umum di Indonesia.",
    "corrected_input": "PT Mencari Cinta Sejati",
    "reasoning": "Input memiliki format nama perusahaan yang umum di Indonesia dengan prefix 'PT' yang berarti Perseroan Terbatas. Nama tersebut terdengar seperti nama perusahaan sungguhan meskipun bersifat fiktif.",
    "confidence_score": 0.95
}
```
- **`valid`** (`bool`): `true` jika input dianggap valid, `false` jika tidak.
- **`message`** (`str`): Pesan ringkas yang menjelaskan hasil validasi.
- **`corrected_input`** (`str`): Saran perbaikan input jika ada kesalahan kecil.
- **`reasoning`** (`str`): Penjelasan dari model mengapa input tersebut dianggap valid atau tidak.
- **`confidence_score`** (`float`): Skor kepercayaan model terhadap hasil validasinya (antara 0.0 hingga 1.0).

---

## 📚 Referensi API

### Fungsi `validate_input_py`

```python
validate_input_py(text: str, model: SupportedModel, label: str) -> dict
```

**Parameter:**

-   **`text`** (`str`): Teks input yang ingin divalidasi.
-   **`model`** (`SupportedModel`): Enum model yang akan digunakan untuk validasi.
-   **`label`** (`str`): Kategori atau jenis input yang diharapkan (misalnya "Alamat", "Nama Lengkap", "Judul", dll.).

**Return:**

-   Sebuah `dict` yang berisi hasil validasi.

### Enum `SupportedModel`

Enum ini berisi daftar model yang didukung oleh library.

-   `SupportedModel.GeminiFlash`
-   `SupportedModel.GeminiFlashLite`
-   `SupportedModel.GeminiFlashLatest`
-   `SupportedModel.Gemma`

---

## 🖥️ Menjalankan Contoh Aplikasi GUI

Proyek ini juga menyertakan contoh aplikasi antarmuka grafis (GUI) yang dibangun menggunakan PySide6 untuk mempermudah pengujian.

1.  **Instal PySide6**:
    ```bash
    pip install PySide6
    ```

2.  **Pastikan Anda sudah mengatur API Key** seperti yang dijelaskan di [bagian Konfigurasi](#-konfigurasi).

3.  **Jalankan aplikasi**:
    Anggap file utama Anda bernama `main_app.py`. Jalankan perintah berikut dari terminal:
    ```bash
    python main_app.py
    ```

Aplikasi ini menyediakan dua mode:
1.  **Form Tes Tunggal**: Untuk menguji satu input pada satu waktu.
2.  **Form untuk Developer**: Untuk melakukan validasi batch pada semua jenis input sekaligus, sangat berguna untuk pengujian cepat.

## Kontribusi

Kontribusi dalam bentuk apapun sangat kami hargai! Jika Anda menemukan bug atau memiliki ide untuk fitur baru, silakan buka *issue* di repositori GitHub proyek ini.

## Lisensi

Proyek ini dilisensikan di bawah [Lisensi MIT](https://opensource.org/licenses/MIT).
