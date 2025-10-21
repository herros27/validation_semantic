---

# 🤖 Library `validation_semantic`

![GitHub last commit](https://img.shields.io/github/last-commit/herros27/validation_semantic)
![GitHub stars](https://img.shields.io/github/stars/herros27/validation_semantic?style=social)
![TestPyPI](https://img.shields.io/badge/TestPyPI-validation--semantic-blue?logo=pypi)
![Rust Build Status](https://github.com/herros27/validation_semantic/actions/workflows/release.yml/badge.svg)

`semantic_validation` adalah *library* **validasi semantik** yang tangguh dan berkinerja tinggi, ditulis sepenuhnya dalam **Rust**. Fokus utamanya adalah menyediakan **core logic** yang andal untuk memastikan data Anda tidak hanya valid secara format, tetapi juga **bermakna dan konsisten** sesuai dengan aturan bisnis yang Anda tetapkan.

Kekuatan utama *library* ini terletak pada kemampuannya untuk diintegrasikan ke berbagai *platform* melalui **bindings**. Ini berarti Anda bisa memanfaatkan kecepatan dan keamanan Rust di lingkungan seperti **WebAssembly (WASM)** untuk *browser*, **Python (menggunakan PyO3/Maturin)** untuk *backend* atau *data science*, dan *platform* lainnya di masa mendatang.

---

## 📑 Daftar Isi

1. [🌟 Fitur Utama](#-fitur-utama)
2. [🚀 Memulai](#-memulai)
3. [⚛️ Menggunakan Library di React (Vite)](#️-menggunakan-library-di-react-vite)

   * [🧩 Instalasi Library dan Plugin Pendukung](#-1️⃣-instalasi-library-dan-plugin-pendukung)
   * [⚙️ Konfigurasi Vite](#️-2️⃣-konfigurasi-vite)
   * [🚀 Gunakan Modul WASM di React](#-3️⃣-gunakan-modul-wasm-di-react)
   * [🧾 Contoh Output](#-4️⃣-contoh-output)
   * [📘 Ringkasan Fungsi Utama](#-5️⃣-ringkasan-fungsi-utama)
4. [🐍 Instalasi untuk Python](#-python)

   * [🔑 Konfigurasi API Key](#-konfigurasi)
   * [🚀 Cara Penggunaan untuk Python](#-cara-penggunaan-untuk-python)
5. [📦 Validasi Banyak Input Sekaligus (Batch Validation)](#-validasi-banyak-input-sekaligus-batch-validation)
6. [🖥️ Menjalankan Contoh Aplikasi GUI (Desktop dengan Python)](#️-menjalankan-contoh-aplikasi-gui-dekstop-dengan-python-)
7. [🤝 Kontribusi](#-kontribusi)
8. [📄 Lisensi](#-lisensi)

---

## 🌟 Fitur Utama

* **Core Logic dalam Rust:** Mesin validasi semantik yang cepat, aman, dan efisien, dibangun di atas fondasi Rust.
* **Validasi Berbasis Aturan:** Terapkan seperangkat aturan yang dapat dikonfigurasi untuk memeriksa integritas dan konsistensi semantik data Anda.
* **Deteksi Anomali:** Mudah mengidentifikasi pola atau nilai data yang tidak sesuai dengan ekspektasi semantik Anda.
* **API yang Fleksibel:** Dirancang untuk mudah diekspos melalui *bindings* ke berbagai bahasa dan lingkungan pemrograman.
* **Laporan Detail:** Dapatkan laporan validasi yang jelas dan informatif.
* **Siap untuk *Cross-Platform*:** Digunakan di server, desktop, maupun browser.

---

## 🚀 Memulai

`semantic_validation` dirancang agar dapat digunakan lintas platform — Anda dapat memanfaatkan *core logic*-nya yang ditulis dalam **Rust** melalui *binding* ke berbagai bahasa dan lingkungan pemrograman.

Saat ini, library ini dapat digunakan di dua platform utama:

* **Frontend:** React (Vite) menggunakan WebAssembly (WASM)
* **Backend / Desktop:** Python (via PyO3 / Maturin)

---

## ⚛️ Menggunakan Library di React (Vite)

Library ini dapat digunakan di **React (Vite)** dengan memanfaatkan **WebAssembly (WASM)** yang dibangun menggunakan Rust.
Langkah-langkah berikut menjelaskan cara instalasi dan penggunaannya.

---

### 🧩 1️⃣ Instalasi Library dan Plugin Pendukung

```bash
# Instal library utama
npm install validation_semantic

# Instal plugin WASM untuk Vite
npm install vite-plugin-wasm vite-plugin-top-level-await
```

> Plugin ini diperlukan agar Vite bisa memuat file `.wasm` dengan benar dan mendukung penggunaan `await` di level atas module.

---

### ⚙️ 2️⃣ Konfigurasi Vite

```ts
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'
import wasm from "vite-plugin-wasm"
import topLevelAwait from "vite-plugin-top-level-await"

export default defineConfig({
  plugins: [
    react(),
    wasm(),                 // Aktifkan dukungan untuk WebAssembly
    topLevelAwait(),        // Izinkan penggunaan "await" di top-level
    tailwindcss(),
  ],
})
```

---

### 🚀 3️⃣ Gunakan Modul WASM di React

```tsx
import { useWasm } from "validation_semantic";

export default function Example() {
  const { wasmReady, wasmModule, error } = useWasm();

  async function runValidation() {
    if (!wasmReady || !wasmModule) {
      console.warn("WASM belum siap");
      return;
    }

    const models = wasmModule.getSupportedModelSelectors();
    const model = models["GEMINI_2_5_FLASH"];

    const result = await wasmModule.validateTextJs(
      "PT Sinar Mentari",
      model,
      "Nama Perusahaan"
    );

    console.log(result);
  }

  if (error) console.error(error);
  else runValidation();
}
```

---

### 🧾 4️⃣ Contoh Output

```json
{
  "valid": true,
  "message": "Input 'PT Sinar Mentari' adalah nama perusahaan yang valid dan umum di Indonesia."
}
```

---

### 📘 5️⃣ Ringkasan Fungsi Utama

| Fungsi                                    | Deskripsi                                            |
| ----------------------------------------- | ---------------------------------------------------- |
| `useWasm()`                               | *Hook* untuk memuat dan menginisialisasi modul WASM. |
| `wasmModule.getSupportedModelSelectors()` | Mengambil daftar model yang tersedia.                |
| `validateTextJs(text, model, type)`       | Melakukan validasi semantik teks.                    |

---

## 🐍 **Python**

Untuk Python, Anda dapat menginstal library ini langsung dari **TestPyPI** menggunakan `pip`.

```bash
pip install -i https://test.pypi.org/simple/ validation-semantic
```

Setelah terinstal, Anda bisa langsung mengimpor dan menggunakan fungsi `validate_input_py` di kode Python Anda:

```python
from validation_semantic import validate_input_py, SupportedModel
```

---

## 🔑 Konfigurasi

Library ini memerlukan API Key dari Google AI Studio.

```bash
# Linux/macOS
export GEMINI_API_KEY="API_KEY_ANDA"

# Windows (Command Prompt)
set GEMINI_API_KEY="API_KEY_ANDA"
```

---

## 🚀 Cara Penggunaan Untuk Python

```python
import json
from validation_semantic import validate_input_py, SupportedModel

text_input = "PT Mencari Cinta Sejati"
input_type = "Nama Perusahaan"

model_choice = SupportedModel.GeminiFlash

result = validate_input_py(
    text=text_input,
    model=model_choice,
    label=input_type
)

print(json.dumps(result, indent=4, ensure_ascii=False))
```

**Output:**

```json
{
    "valid": false,
    "message": "Input 'PT Mencari Cinta Sejati' adalah nama perusahaan yang tidak valid dan umum di Indonesia."
}
```

---

## 📦 Validasi Banyak Input Sekaligus (Batch Validation)

Contoh penggunaan batch validation melalui GUI berbasis **PySide6**.

```python
self.worker = BatchValidationWorker(user_inputs, model)
self.thread.started.connect(self.worker.run)
```

Semua hasil dikirim kembali ke GUI melalui sinyal `finished`.

---

## 🖥️ Menjalankan Contoh Aplikasi GUI (Dekstop dengan Python)

```bash
pip install PySide6
python main_app.py
```

---

## 🤝 Kontribusi

Kontribusi sangat diterima!
Silakan buat *issue* atau *pull request* di [GitHub Repository](https://github.com/herros27/validation_semantic).

---

## 📄 Lisensi

Proyek ini dilisensikan di bawah [MIT License](https://opensource.org/licenses/MIT).

---
