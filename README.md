---

# 🤖 Library `Semantic Validation` Dengan Gemini API

![GitHub last commit](https://img.shields.io/github/last-commit/herros27/validation_semantic)
![GitHub stars](https://img.shields.io/github/stars/herros27/validation_semantic?style=social)
![TestPyPI](https://img.shields.io/badge/TestPyPI-validation--semantic-blue?logo=pypi)
![Rust Build Status](https://github.com/herros27/validation_semantic/actions/workflows/release.yml/badge.svg)
![npm version](https://img.shields.io/npm/v/validation_semantic?logo=npm)
![npm downloads](https://img.shields.io/npm/dt/validation_semantic?logo=npm)


`validation_semantic` adalah *library* **validasi semantik** yang cepat, aman, dan cerdas — dibangun sepenuhnya dengan **Rust** dan didukung oleh **model Gemini dari Google AI Studio**.

Library ini tidak hanya memeriksa validitas data secara **sintaksis** (misalnya format email atau nomor telepon), tetapi juga melakukan **analisis semantik** untuk memahami *makna dan konteks* dari input pengguna. Dengan integrasi **Gemini API**, proses validasi menjadi lebih kontekstual dan adaptif terhadap berbagai jenis data maupun bahasa.

Berbeda dari validator konvensional, `validation_semantic` berfokus pada **pemahaman arti dan tujuan data**, bukan sekadar pola teks.
Sebagai contoh, library ini dapat membedakan apakah sebuah input lebih sesuai dikategorikan sebagai nama institusi, alamat email, deskripsi, atau teks naratif — menghasilkan validasi yang jauh lebih presisi dan bermakna.

---

## 🧠 **Catatan Penelitian:**

> Pustaka ini dikembangkan sebagai bagian dari proyek penelitian akademis.
> Pengembang didorong untuk mencobanya dan memberikan umpan balik mengenai kinerja dan kemudahan penggunaannya.
> Untuk informasi selengkapnya, lihat bagian [📊 Permintaan Partisipasi Dan Umpan Balik Riset](#-permintaan-partisipasi-dan-umpan-balik-riset) di bawah ini.

---

---

## 🧠 **Catatan Penelitian:**

> Library ini dikembangkan sebagai bagian dari proyek penelitian akademik.
> Pengembang didorong untuk mencoba dan memberikan umpan balik terkait kinerja serta kemudahan penggunaannya.
> Untuk informasi lebih lanjut, lihat bagian [Partisipasi Penelitian & Permintaan Umpan Balik](#-partisipasi-penelitian--permintaan-umpan-balik) di bawah ini.

---
### 🌍 Dukungan Multiplatform
---
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
7. [Research Participation & Feedback](#-research-participation--feedback-request)
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

`validation_semantic` dirancang agar dapat digunakan lintas platform — Anda dapat memanfaatkan *core logic*-nya yang ditulis dalam **Rust** melalui *binding* ke berbagai bahasa dan lingkungan pemrograman.

Saat ini, library ini dapat digunakan di dua platform utama:

* **Frontend:** React (Vite) menggunakan WebAssembly (WASM)
* **Backend / Desktop:** Python (via PyO3 / Maturin)

---

## ⚛️ Menggunakan Library di React (Vite) / Next JS

Library ini dapat digunakan di **React (Vite)** dengan memanfaatkan **WebAssembly (WASM)** yang dibangun menggunakan Rust.
Langkah-langkah berikut menjelaskan cara instalasi dan penggunaannya. Untuk Next JS, sudah otomatis support WASM, tidak memerlukan setting seperti di vite, tinggal menggunakan tag WasmProvider yang membungkus tag lain. Library ini hanya bisa di gunakan dengan file yang di bagian atas nya mempunyai "use client".

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
## 🔑 Konfigurasi

Library ini memerlukan API Key dari Google AI Studio.

```bash
# Buat file .env dan buat envirovment variabel seperti di bawah:
VITE_GEMINI_API_KEY="API_KEY_ANDA"
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

    const result = await wasmModule.validateInput(
      "PT Sinar Mentari",
      model,
      "Nama Perusahaan",
      import.meta.env.VITE_GEMINI_API_KEY
    );

    console.log(result);
  }

  if (error) console.error(error);
  else runValidation();
}
```

---

### 📋 Hasil Contoh Output (di Console)

```json
{
  "valid": true,
  "message": "Input 'PT Sinar Mentari' adalah nama perusahaan yang valid dan umum di Indonesia."
}
```

---

### 🧠 5️⃣ Contoh Validasi Banyak Input Sekaligus (Batch Validation)

Kamu dapat melakukan **validasi beberapa input sekaligus** menggunakan `validateInput` dari modul WASM.
Setiap input diproses secara **asynchronous dan paralel** untuk efisiensi.

```tsx
import React, { useState } from "react";
import { useWasm } from "validation_semantic";

type InputType =
  | "email"
  | "institution name"
  | "company name"
  | "product name"
  | "location name"
  | "full name"
  | "title"
  | "occupation"
  | "tag"
  | "address"
  | "text area";

export default function BatchValidationExample() {
  const { wasmReady, wasmModule, error: wasmError } = useWasm();
  const modelToUseKey = "GEMINI_FLASH"; //GEMINI_FLASH_LITE, GEMINI_FLASH_LATEST, GEMMA

  const [formData, setFormData] = useState<Record<InputType, string>>({
    email: "",
    "full name": "",
    address: "",
    "product name": "",
    "institution name": "",
    "company name": "",
    "location name": "",
    title: "",
    occupation: "",
    tag: "",
    "text area": "",
  });

  const [results, setResults] = useState<Record<string, any> | null>(null);
  const [loading, setLoading] = useState(false);

  // Handler perubahan input
  const handleChange = (key: InputType, value: string) => {
    setFormData((prev) => ({
      ...prev,
      [key]: value,
    }));
  };

  async function validateBatchInputs() {
    if (!wasmReady || !wasmModule) {
      alert("WASM module is not ready.");
      return;
    }

    const supportedModels = wasmModule.getSupportedModelSelectors();
    const modelSelectorInt = supportedModels[modelToUseKey];

    if (typeof modelSelectorInt === "undefined") {
      alert(`Model ${modelToUseKey} not found.`);
      return;
    }

    setLoading(true);
    try {
      const validationPromises = Object.entries(formData)
        .filter(([_, value]) => value.trim() !== "") // hanya input yang diisi
        .map(async ([inputType, inputValue]) => {
          try {
            const result = await wasmModule.validateInput(
              inputValue,
              modelSelectorInt,
              inputType as InputType,
              import.meta.env.VITE_GEMINI_API_KEY
            );
            return { inputType, inputValue, result, error: null };
          } catch (err: any) {
            return {
              inputType,
              inputValue,
              result: null,
              error: err?.message ?? "Validation error occurred.",
            };
          }
        });

      const results = await Promise.all(validationPromises);
      const batchResults = Object.fromEntries(
        results.map((r) => [
          r.inputType,
          { input: r.inputValue, result: r.result, error: r.error },
        ])
      );

      setResults(batchResults);
      console.log("Batch Validation Results:", batchResults);
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className='max-w-xl mx-auto p-4 space-y-6'>
      <h1 className='text-xl font-bold text-center'>Batch Validation Form</h1>

      {/* Form Input */}
      <div className='space-y-4'>
        {Object.keys(formData).map((key) => (
          <div key={key} className='flex flex-col'>
            <label className='font-semibold capitalize'>{key}</label>
            <input
              type='text'
              className='border border-gray-300 rounded-md p-2'
              value={formData[key as InputType]}
              onChange={(e) => handleChange(key as InputType, e.target.value)}
              placeholder={`Masukkan ${key}`}
            />
          </div>
        ))}
      </div>

      {/* Tombol Validasi */}
      <button
        onClick={validateBatchInputs}
        disabled={loading || !wasmReady}
        className='bg-blue-600 text-white px-4 py-2 rounded-md w-full disabled:opacity-50'>
        {loading ? "Validating..." : "Validate All Inputs"}
      </button>

      {/* Hasil */}
      {results && (
        <div className='mt-6 bg-gray-100 p-4 rounded-md'>
          <h2 className='font-semibold mb-2'>Validation Results:</h2>
          <pre className='text-sm bg-white p-2 rounded-md overflow-x-auto'>
            {JSON.stringify(results, null, 2)}
          </pre>
        </div>
      )}

      {wasmError && (
        <p className='text-red-500 text-sm text-center mt-4'>
          Error loading WASM: {wasmError}
        </p>
      )}
    </div>
  );
}

```

---

### 📋 Hasil Contoh Output (di Console)

```json
{
  "email": {
    "input": "khairunsyah8935@gmail.com",
    "result": {
      "valid": true,
      "message": "Alamat email valid. Format dan domain sudah benar, bukan domain contoh atau domain sekali pakai, dan panjangnya tidak melebihi batas."
    },
    "error": null
  },
  "full name": {
    "input": "XYZ",
    "result": {
      "valid": false,
      "message": "Input 'XYZ' tidak terlihat seperti nama manusia, institusi, atau entitas yang realistis. Ini lebih menyerupai placeholder atau singkatan generik."
    },
    "error": null
  },
  "address": {
    "input": "My House",
    "result": {
      "valid": false,
      "message": "Input 'My House' terlalu umum dan tidak mengandung elemen geografis yang spesifik dan realistis seperti nama jalan, nomor, kota, atau kode pos. Ini tidak dapat digunakan sebagai alamat yang valid."
    },
    "error": null
  },
  "company name": {
    "input": "Companyy",
    "result": {
      "valid": false,
      "message": "Input 'Companyy' terlalu generik dan tidak terdengar seperti nama perusahaan yang spesifik atau realistis. Penulisan dengan dua 'y' di akhir juga terlihat tidak lazim untuk nama entitas asli, menyerupai placeholder atau nama uji coba. Mohon gunakan nama perusahaan yang lebih spesifik dan realistis."
    },
    "error": null
  }
}

```

---

### 💡 Catatan

* Fungsi `validateInput()` tetap digunakan seperti pada validasi tunggal.
* Perbedaan utamanya adalah semua input dikirim **sekaligus** menggunakan `Promise.all()` agar berjalan paralel.
* Kamu bisa menyesuaikan daftar input sesuai kebutuhan form atau dataset kamu.

---

---

### 📘 5️⃣ Ringkasan Fungsi Utama

| Fungsi                                           | Deskripsi                                            |
| ------------------------------------------------ | ---------------------------------------------------- |
| `useWasm()`                                      | *Hook* untuk memuat dan menginisialisasi modul WASM. |
| `wasmModule.getSupportedModelSelectors()`        | Mengambil daftar model yang tersedia.                |
| `validateInput(text, model, type,youur_api_key)` | Melakukan validasi semantik teks.                    |

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
# Membuat .env dengan variabel berisi API key dari google studio anda :
GEMINI_API_KEY="API_KEY_ANDA"
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
### JSON Output:

```json
{
  "email": {
    "input": "khairunsyah8935@gmail.com",
    "result": {
      "valid": true,
      "message": "Alamat email valid. Format dan domain sudah benar, bukan domain contoh atau domain sekali pakai, dan panjangnya tidak melebihi batas."
    },
    "error": null
  },
  "full name": {
    "input": "XYZ",
    "result": {
      "valid": false,
      "message": "Input 'XYZ' tidak terlihat seperti nama manusia, institusi, atau entitas yang realistis. Ini lebih menyerupai placeholder atau singkatan generik."
    },
    "error": null
  },
  "address": {
    "input": "My House",
    "result": {
      "valid": false,
      "message": "Input 'My House' terlalu umum dan tidak mengandung elemen geografis yang spesifik dan realistis seperti nama jalan, nomor, kota, atau kode pos. Ini tidak dapat digunakan sebagai alamat yang valid."
    },
    "error": null
  },
  "company name": {
    "input": "Companyy",
    "result": {
      "valid": false,
      "message": "Input 'Companyy' terlalu generik dan tidak terdengar seperti nama perusahaan yang spesifik atau realistis. Penulisan dengan dua 'y' di akhir juga terlihat tidak lazim untuk nama entitas asli, menyerupai placeholder atau nama uji coba. Mohon gunakan nama perusahaan yang lebih spesifik dan realistis."
    },
    "error": null
  }
}

```


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

## 📊 Permintaan Partisipasi Dan Umpan Balik Riset

Pustaka **`Validasi Semantik`** dikembangkan sebagai bagian dari **proyek riset akademis** yang berfokus pada evaluasi performa dan kegunaan sistem validasi semantik berbasis AI.

Jika Anda seorang **pengembang** yang menggunakan pustaka ini, umpan balik Anda sangat berharga untuk riset ini.
Silakan coba gunakan pustaka ini dengan berbagai jenis masukan seperti **nama**, **alamat**, **judul**, **deskripsi**, atau **kolom teks**, dan bagikan pengalaman Anda.

Anda dapat menyertakan:

- Pendapat Anda tentang **kemudahan penggunaan** dan **pengalaman pengembang**
- **kinerja** atau **akurasi** hasil validasi
- **Masalah atau saran perbaikan** yang ingin Anda laporkan
- (Opsional) **Contoh atau bukti** tentang bagaimana Anda mengintegrasikan pustaka ini ke dalam proyek Anda

Kontribusi Anda akan secara langsung mendukung evaluasi dan pengembangan lebih lanjut dari proyek riset ini.

📩 Anda dapat memberikan masukan dengan **membuka Masalah di repositori GitHub resmi**:
👉 [GitHub Issues for PyPi users](https://github.com/herros27/validation_semantic/issues)
👉 [GitHub Issues for NPM users](https://github.com/herros27/React-Library-Semantic-Validation/issues) 

Terima kasih banyak telah meluangkan waktu untuk berpartisipasi dan berkontribusi dalam penelitian ini. 🙏

---

---

## 🤝 Kontribusi

Kontribusi sangat diterima!
Silakan buat *issue* atau *pull request* di [GitHub Repository](https://github.com/herros27/validation_semantic).

---

---

## 📊 Partisipasi Penelitian & Permintaan Umpan Balik

Library **`validation_semantic`** ini dikembangkan sebagai bagian dari **proyek penelitian akademik** yang berfokus pada evaluasi kinerja dan kemudahan penggunaan sistem validasi semantik berbasis AI.

Jika Anda adalah seorang **pengembang** yang menggunakan library ini, umpan balik Anda sangat berharga bagi penelitian ini.
Silakan coba gunakan library ini dengan berbagai jenis input seperti **nama**, **alamat**, **judul**, **deskripsi**, atau **kolom teks**, dan bagikan pengalaman Anda.

Anda dapat menyertakan:

* Pendapat Anda mengenai **kemudahan penggunaan** dan **pengalaman pengembang**
* **Kinerja** atau **akurasi** dari hasil validasi
* Setiap **masalah atau saran perbaikan** yang ingin Anda ajukan
* (Opsional) **Bukti atau contoh** bagaimana Anda mengintegrasikan library ini ke dalam proyek Anda

Kontribusi Anda akan secara langsung mendukung evaluasi dan pengembangan lebih lanjut dari proyek penelitian ini.

📬 Untuk memberikan umpan balik, silakan buat **issue baru** di halaman GitHub repository berikut:
👉 [Buka Issues di GitHub](https://github.com/herros27/validation_semantic/issues)

Terima kasih banyak telah meluangkan waktu untuk berpartisipasi dan berkontribusi dalam penelitian ini. 🙏

---


## 📄 Lisensi

Proyek ini dilisensikan di bawah [MIT License](https://opensource.org/licenses/MIT).

---
