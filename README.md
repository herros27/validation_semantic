
# 🤖 `semantic_validation`

![GitHub last commit](https://img.shields.io/github/last-commit/herros27/validation_semantic)
![GitHub stars](https://img.shields.io/github/stars/herros27/validation_semantic?style=social)
![TestPyPI](https://img.shields.io/badge/TestPyPI-validation--semantic-blue?logo=pypi)
![Rust Build Status](https://github.com/herros27/validation_semantic/actions/workflows/release.yml/badge.svg)

`semantic_validation` adalah *library* **validasi semantik** yang tangguh dan berkinerja tinggi, ditulis sepenuhnya dalam **Rust**. Fokus utamanya adalah menyediakan **core logic** yang andal untuk memastikan data Anda tidak hanya valid secara format, tetapi juga **bermakna dan konsisten** sesuai dengan aturan bisnis yang Anda tetapkan.

Kekuatan utama *library* ini terletak pada kemampuannya untuk diintegrasikan ke berbagai *platform* melalui **bindings**. Ini berarti Anda bisa memanfaatkan kecepatan dan keamanan Rust di lingkungan seperti **WebAssembly (WASM)** untuk *browser*, **Python (menggunakan PyO3/Maturin)** untuk *backend* atau *data science*, dan *platform* lainnya di masa mendatang.

---

---

## 📑 Daftar Isi

* [🌟 Fitur Utama](#-fitur-utama)
* [🚀 Memulai](#-memulai)
* [⚙️ Instalasi Untuk Python](#️-instalasi-untuk-python)
* [🔑 Konfigurasi](#-konfigurasi)
* [🚀 Cara Penggunaan Untuk Python](#-cara-penggunaan-untuk-python)
* [📦 Validasi Banyak Input Sekaligus (Batch Validation)](#-validasi-banyak-input-sekaligus-batch-validation)
* [🖥️ Menjalankan Contoh Aplikasi GUI](#️-menjalankan-contoh-aplikasi-gui)
* [🤝 Kontribusi](#-kontribusi)
* [📄 Lisensi](#-lisensi)

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

`semantic_validation` dirancang agar dapat digunakan lintas platform — Anda dapat memanfaatkan *core logic*-nya yang ditulis dalam **Rust** melalui *binding* ke berbagai bahasa dan lingkungan pemrograman.

Saat ini, library ini dapat digunakan di dua platform utama:

---

### 🐍 **Python**

Untuk Python, Anda dapat menginstal library ini langsung dari **TestPyPI** menggunakan `pip`.

> Pastikan Anda sudah menginstal **Python 3.8+** dan `pip` terbaru.

```bash
pip install -i https://test.pypi.org/simple/ validation-semantic
```

Setelah terinstal, Anda bisa langsung mengimpor dan menggunakan fungsi `validate_input_py` di kode Python Anda:

```python
from validation_semantic import validate_input_py, SupportedModel
```

---

---

## ⚛️ Menggunakan Library di React (Vite)

Library ini dapat digunakan di **React (Vite)** dengan memanfaatkan **WebAssembly (WASM)** yang dibangun menggunakan Rust.
Langkah-langkah berikut menjelaskan cara instalasi dan penggunaannya.

---

### 🧩 1️⃣ Instalasi Library dan Plugin Pendukung

Jalankan perintah berikut di terminal proyek React kamu:

```bash
# Instal library utama
npm install validation_semantic

# Instal plugin WASM untuk Vite
npm install vite-plugin-wasm vite-plugin-top-level-await
```

> Plugin ini diperlukan agar Vite bisa memuat file `.wasm` dengan benar dan mendukung penggunaan `await` di level atas module.

---

### ⚙️ 2️⃣ Konfigurasi Vite

Edit file `vite.config.ts` (atau `vite.config.js`) agar mendukung WebAssembly dan top-level await.
Gunakan konfigurasi berikut:

```ts
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'
import wasm from "vite-plugin-wasm"
import topLevelAwait from "vite-plugin-top-level-await"

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    react(),
    wasm(),                 // Aktifkan dukungan untuk WebAssembly
    topLevelAwait(),        // Izinkan penggunaan "await" di top-level
    tailwindcss(),
  ],
})
```

> `vite-plugin-wasm` memastikan file `.wasm` dapat dimuat dinamis.
> `vite-plugin-top-level-await` memungkinkan kita memakai `await` di luar fungsi async — berguna untuk inisialisasi modul WASM.

---

### 🚀 3️⃣ Gunakan Modul WASM di React

Berikut contoh sederhana pemanggilan fungsi dari modul `validation_semantic`:

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
  "valid": false,
  "message": "Input 'PT Sinar Mentari' adalah nama perusahaan yang tidak valid dan umum di Indonesia."
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



## 🚀 Cara Penggunaan Untuk Python

## 🔑 Konfigurasi

Library ini memerlukan API Key dari Google AI Studio untuk dapat berinteraksi dengan model Gemini.

1.  **Dapatkan API Key Anda**:  
    Kunjungi [Google AI Studio](https://aistudio.google.com/app/apikey) untuk membuat API Key baru.

2.  **Atur Environment Variable**:

    - **Linux/macOS:**
        ```bash
        export GEMINI_API_KEY="API_KEY_ANDA_DISINI"
        ```

    - **Windows (Command Prompt):**
        ```bash
        set GEMINI_API_KEY="API_KEY_ANDA_DISINI"
        ```

---

Penggunaan library ini sangat mudah. Anda hanya perlu mengimpor fungsi `validate_input_py` dan enum `SupportedModel`.

### Contoh Kode Sederhana

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


### Hasil Output

```json
{
    "valid": false,
    "message": "Input 'PT Mencari Cinta Sejati' adalah nama perusahaan yang tidak valid dan umum di Indonesia."
}
```

---

## 📦 Validasi Banyak Input Sekaligus (Batch Validation)

Selain validasi tunggal, proyek ini juga mendukung **validasi banyak input secara bersamaan (batch)** melalui GUI berbasis **PySide6**.
Semua input yang dimasukkan di form akan dikirim **bersamaan ke worker thread**, lalu setiap input divalidasi menggunakan `validate_input_py()`.

### 🔍 Kode Utama yang Mengirim Semua Input

1. **Pengambilan semua input pengguna**

   ```python
   user_inputs = {}
   for label, widget in self.inputs.items():
       text = widget.toPlainText() if isinstance(widget, QTextEdit) else widget.text()
       if text.strip():
           user_inputs[label] = text
   ```

2. **Menjalankan worker thread batch**

   ```python
   self.thread = QThread()
   self.worker = BatchValidationWorker(user_inputs, model)
   self.worker.moveToThread(self.thread)

   self.thread.started.connect(self.worker.run)
   self.worker.finished.connect(self.on_batch_finished)
   self.worker.error.connect(self.on_batch_error)
   self.thread.start()
   ```

3. **Worker yang memproses semua input sekaligus**

   ```python
   class BatchValidationWorker(QObject):
       finished = Signal(dict)
       error = Signal(str)

       def __init__(self, inputs, model):
           super().__init__()
           self.inputs = inputs
           self.model = model

       def run(self):
           results = {}
           for label, text in self.inputs.items():
               if not text.strip():
                   continue
               try:
                   result = validate_input_py(text.strip(), self.model, label)
                   results[label] = {"input": text.strip(), "result": result, "error": None}
               except Exception as e:
                   results[label] = {"input": text.strip(), "result": None, "error": str(e)}
           self.finished.emit(results)
   ```

4. **Menampilkan hasil batch ke GUI**

   ```python
   def on_batch_finished(self, results):
       for label, data in results.items():
           print(f"{label}: {data['result']}")
   ```

### 🧠 Ringkasan Alur

```
[Semua Field Input di GUI]
      ↓
run_batch_validation()
      ↓
BatchValidationWorker.run()
      ↓
validate_input_py() dipanggil untuk setiap input
      ↓
Emit hasil ke on_batch_finished()
      ↓
Tampilkan semua hasil validasi di GUI
```

---

## 🖥️ Menjalankan Contoh Aplikasi GUI

1. **Instal dependensi:**

   ```bash
   pip install PySide6
   ```

2. **Jalankan aplikasi:**

   ```bash
   python main_app.py
   ```

Aplikasi menyediakan dua mode:

* **Form Tes Tunggal** – Validasi satu input.
* **Form untuk Developer** – Jalankan batch validation semua input sekaligus.

---

## 🤝 Kontribusi

Kontribusi dalam bentuk apapun sangat kami hargai!
Jika Anda menemukan bug atau memiliki ide fitur baru, silakan buka *issue* di repositori GitHub proyek ini.

---

## 📄 Lisensi

Proyek ini dilisensikan di bawah [Lisensi MIT](https://opensource.org/licenses/MIT).

