---

# 🤖 Library `validation_semantic` Dengan Gemini API

![GitHub last commit](https://img.shields.io/github/last-commit/herros27/validation_semantic)
![GitHub stars](https://img.shields.io/github/stars/herros27/validation_semantic?style=social)
![TestPyPI](https://img.shields.io/badge/TestPyPI-validation--semantic-blue?logo=pypi)
![Rust Build Status](https://github.com/herros27/validation_semantic/actions/workflows/release.yml/badge.svg)

`validation_semantic` adalah *library* **validasi semantik** yang cepat, aman, dan cerdas — dibangun sepenuhnya dengan **Rust** dan didukung oleh **model Gemini dari Google AI Studio**.

Library ini tidak hanya memeriksa validitas data secara **sintaksis** (misalnya format email atau nomor telepon), tetapi juga melakukan **analisis semantik** untuk memahami *makna dan konteks* dari input pengguna. Dengan integrasi **Gemini API**, proses validasi menjadi lebih kontekstual dan adaptif terhadap berbagai jenis data maupun bahasa.

Berbeda dari validator konvensional, `validation_semantic` berfokus pada **pemahaman arti dan tujuan data**, bukan sekadar pola teks.
Sebagai contoh, library ini dapat membedakan apakah sebuah input lebih sesuai dikategorikan sebagai nama institusi, alamat email, deskripsi, atau teks naratif — menghasilkan validasi yang jauh lebih presisi dan bermakna.

### 🌍 Dukungan Multiplatform

Kelebihan utama `validation_semantic` terletak pada desain modular dan interoperabilitas lintas platform melalui **bindings**:

* 🧩 **WebAssembly (WASM)** — memungkinkan integrasi di *frontend* seperti React atau Vue dengan performa tinggi langsung di browser.
* 🐍 **Python (via PyO3 / Maturin)** — ideal untuk *backend services*, *data validation pipelines*, atau *machine learning preprocessing*.
* 🔧 Dukungan untuk *binding* lain seperti Kotlin sedang dalam tahap pengembangan.

Dengan kombinasi **kecepatan Rust** dan **kecerdasan Gemini**, `validation_semantic` menghadirkan sistem validasi modern yang **kontekstual, efisien, dan mudah diintegrasikan** ke berbagai proyek.

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
   * [📦 Validasi Banyak Input Sekaligus (Batch Validation)](#-validasi-banyak-input-sekaligus-batch-validation-dengan-python)
5. [🧩 Jenis Input yang Dapat Divalidasi](#️-jenis-input-yang-dapat-divalidasi)
6. [🤝 Kontribusi](#-kontribusi)
7. [📄 Lisensi](#-lisensi)

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


## 📦 Validasi Banyak Input Sekaligus (Batch Validation) dengan python


### Kode Lengkap:

```python
class BatchValidationWorker:
    def __init__(self, inputs, model):
        self.inputs = inputs
        self.model = model

    def run(self):
        results = {}
        for label, text in self.inputs.items():
            if not text.strip():
                continue  # lewati input kosong

            try:
                
                result = validate_input_py(text.strip(), self.model, label) #validate_input_py merupakan fungsi dari library
                results[label] = {
                    "input": text.strip(),
                    "result": result,
                    "error": None
                }
            except Exception as e:
                results[label] = {
                    "input": text.strip(),
                    "result": None,
                    "error": str(e)
                }
        return results
```

### Contoh Penggunaan:

```python

if __name__ == "__main__":
    user_inputs = {
        "nama": "John Doe",
        "email": "john@example.com",
        "alamat": "error di sini"
    }
    model = SupportedModel.GeminiFlashLite
    worker = BatchValidationWorker(user_inputs, model)
    results = worker.run()

    print(results)
    json_output = json.dumps(results, indent=4, ensure_ascii=False)
    print("\n=== Hasil Validasi Batch ===")
    print(json_output)
    for label, info in results.items():
        print(f"[{label}]")
        print(" Input:", info["input"])
        if info["error"]:
            print(" ❌ Error:", info["error"])
        else:
            if info["result"]["valid"] == True:
                print(" ✅ Valid:", info["result"]["message"])
            else:
                print(" ⚠️  Invalid:", info["result"]["message"])
          
        print()
```

### Hasil Output:

```
[nama]
 Input: John Doe
 ✅ Valid: Nama valid.

[email]
 Input: john@example.com
 ⚠️  Invalid: Domain 'example.com' adalah domain contoh dan tidak valid untuk penggunaan nyata. Silakan gunakan domain yang  valid dan profesional.

[alamat]
 Input: error di sini
 ⚠️  Invalid: Alamat tidak valid karena mengandung kata-kata yang tidak membentuk struktur alamat fisik yang realistis.     

```
### JSON Output:

```json
{
    "nama": {
        "input": "John Doe",
        "result": {
            "valid": true,
            "message": "Nama valid."
        },
        "error": null
    },
    "email": {
        "input": "john@example.com",
        "result": {
            "valid": false,
            "message": "Alamat email menggunakan domain 'example.com' yang merupakan domain contoh dan tidak valid untuk penggunaan nyata. Silakan gunakan domain yang valid dan profesional."
        },
        "error": null
    },
    "alamat": {
        "input": "error di sini",
        "result": {
            "valid": false,
            "message": "Alamat tidak valid karena mengandung kata-kata yang tidak membentuk struktur alamat fisik yang realistis."
        },
        "error": null
    }
}

```

---

Contoh penggunaan batch validation melalui GUI berbasis **PySide6**.

```python
self.worker = BatchValidationWorker(user_inputs, model)
self.thread.started.connect(self.worker.run)
```

Semua hasil dikirim kembali ke GUI melalui sinyal `finished`.

---


## 🧩 Jenis Input yang Dapat Divalidasi

Library `validation_semantic` mendukung berbagai jenis input teks yang umum digunakan dalam aplikasi bisnis, akademik, maupun personal.
Berikut daftar lengkap jenis input yang dapat divalidasi beserta **fungsi atau konteks penggunaannya**:

| 🏷️ **Jenis Input**                                                           | 🧠 **Deskripsi Validasi**                                                                                  | 💡 **Contoh Input**                         |
| ----------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- | ------------------------------------------- |
| `alamat email`, `email`                                                       | Memvalidasi format dan kesahihan alamat email.                                                             | `user@example.com`                          |
| `nama institusi`, `nama lembaga`, `institusi`, `lembaga`                      | Mengecek kesesuaian nama lembaga atau institusi resmi.                                                     | `Universitas Indonesia`, `LIPI`             |
| `nama perusahaan`                                                             | Memastikan nama perusahaan valid dan umum digunakan.                                                       | `PT Sinar Mentari`                          |
| `nama produk`                                                                 | Memeriksa nama produk atau merek agar sesuai konteks industri.                                             | `Indomie`, `Aqua`, `iPhone 15`              |
| `nama lokasi`, `lokasi`, `tempat`                                             | Mengevaluasi apakah teks merupakan nama lokasi atau wilayah yang sah.                                      | `Jakarta Selatan`, `Bandung`, `Paris`       |
| `nama lengkap`, `nama`                                                        | Validasi nama lengkap pengguna sesuai pola umum nama orang.                                                | `Budi Santoso`, `Kemas Khairunsyah`         |
| `judul`                                                                       | Mengecek apakah teks sesuai untuk digunakan sebagai judul dokumen, artikel, atau karya ilmiah.             | `Analisis Dampak Teknologi AI di Indonesia` |
| `pekerjaan`                                                                   | Memastikan teks merupakan jabatan atau profesi yang dikenal umum.                                          | `Software Engineer`, `Dokter`, `Guru`       |
| `tag`                                                                         | Validasi tag pendek yang biasanya digunakan untuk pengelompokan atau kategorisasi.                         | `AI`, `Teknologi`, `Pendidikan`             |
| `alamat`                                                                      | Memeriksa struktur alamat agar sesuai dengan format umum.                                                  | `Jl. Merdeka No.10, Bandung`                |
| `text area`, `teks area`, `konten`, `deskripsi`, `blog`, `cerita`, `komentar` | Validasi teks panjang (paragraf) untuk memastikan isi bermakna, tidak kosong, dan sesuai konteks semantik. | `Saya sangat puas dengan produk ini!`       |

---

🧠 **Catatan:**

* Semua jenis input di atas **bersifat fleksibel** — sistem akan mengenali label yang mirip (misalnya `nama institusi` dan `lembaga` akan diproses sama).
* Validasi tidak hanya berdasarkan format (regex), tetapi juga **semantik dan konteks makna** dengan bantuan model bahasa.

---

---

## 🤝 Kontribusi

Kontribusi sangat diterima!
Silakan buat *issue* atau *pull request* di [GitHub Repository](https://github.com/herros27/validation_semantic).

---

## 📄 Lisensi

Proyek ini dilisensikan di bawah [MIT License](https://opensource.org/licenses/MIT).

---
