// src/main.rs

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use validation_semantic::{
    free_rust_string, validate_text_ffi, SupportedModel, ValidationResponse,
};

fn main() {
    println!("🚀 Memulai Pengujian FFI Validation Semantic");
    println!("============================================================");

    let test_inputs = [
        
        // 🔹 Email
        // ("kemas@gmail.com", "Email"),
        // ("siti.nurhaliza@uad.ac.id", "Email"),
        // ("rizky_ramadhan@yahoo.co.id", "Email"),
        // ("user.test123@domain.org", "Email"),
        // ("kemas.gmail.com", "Email"),
        // ("siti@nurhaliza", "Email"),
        // ("user@@mail.com", "Email"),
        // ("example@com", "Email"),
        // ("asdf qwerty", "Email"),

        // // ✅ Contoh inputan VALID
        // ("Universitas Ahmad Dahlan", "Nama Institusi"),
        // ("Institut Teknologi Bandung", "Nama Institusi"),
        // ("PT Telkom Indonesia Tbk", "Nama Institusi"),
        // ("SMA Negeri 1 Yogyakarta", "Nama Institusi"),
        // ("Kementerian Pendidikan dan Kebudayaan Republik Indonesia", "Nama Institusi"),
        // ("Lembaga Ilmu Pengetahuan Indonesia", "Nama Institusi"),
        // ("CV Maju Jaya Abadi", "Nama Institusi"),
        // ("Badan Nasional Penanggulangan Bencana", "Nama Institusi"),

    //     // ❌ Contoh inputan TIDAK VALID
    //     ("asdf qwerty", "Nama Institusi"),
    //     ("Nama Lembaga", "Nama Institusi"),
    //     ("Perusahaan Saya", "Nama Institusi"),
    //     ("Organisasi", "Nama Institusi"),
    //     ("PT", "Nama Institusi"),
    //     ("-", "Nama Institusi"),
    //     ("Tidak ada", "Nama Institusi"),
    //     ("Lorem Ipsum Institute", "Nama Institusi"),

    //     // ✅ Contoh inputan VALID
    //     ("PT Astra International Tbk", "Nama Perusahaan"),
    //     ("PT Garuda Indonesia", "Nama Perusahaan"),
    //     ("Tokopedia", "Nama Perusahaan"),
    //     ("CV Sukses Makmur Abadi", "Nama Perusahaan"),
    //     ("Gojek Indonesia", "Nama Perusahaan"),
    //     ("Shopee Indonesia", "Nama Perusahaan"),
    //     ("PT Bank Central Asia Tbk", "Nama Perusahaan"),
    //     ("PT Pertamina (Persero)", "Nama Perusahaan"),
    //     ("Bukalapak", "Nama Perusahaan"),
    //     ("Microsoft Corporation", "Nama Perusahaan"),

    //     // ❌ Contoh inputan TIDAK VALID
    //     ("asdf qwerty", "Nama Perusahaan"),
    //     ("Nama Perusahaan", "Nama Perusahaan"),
    //     ("Perusahaan Saya", "Nama Perusahaan"),
    //     ("PT", "Nama Perusahaan"),
    //     ("Company", "Nama Perusahaan"),
    //     ("-", "Nama Perusahaan"),
    //     ("Tidak diketahui", "Nama Perusahaan"),
    //     ("Lorem Ipsum Corp", "Nama Perusahaan"),

    //     // ✅ Contoh inputan VALID
        // ("Indomie Goreng Spesial", "Nama Produk"),
        // ("Aqua Botol 600ml", "Nama Produk"),
        // ("Samsung Galaxy S24", "Nama Produk"),
        // ("Oppo Reno 10 Pro", "Nama Produk"),
        // ("Teh Botol Sosro", "Nama Produk"),
        // ("Sabun Lifebuoy Antibacterial", "Nama Produk"),
        // ("Honda Beat Street", "Nama Produk"),
        // ("Apple MacBook Air M3", "Nama Produk"),
        // ("Laptop ASUS ROG Strix", "Nama Produk"),
        // ("Sari Roti Tawar Kupas", "Nama Produk"),

        // ❌ Contoh inputan TIDAK VALID
        // ("Produk Saya", "Nama Produk"),
        // ("Barang", "Nama Produk"),
        // ("asdf qwerty", "Nama Produk"),
        // ("Nama Produk", "Nama Produk"),
        // ("Lorem Ipsum Item", "Nama Produk"),
        // ("Test Produk", "Nama Produk"),
        ("Obat Kuat", "Nama Produk"),
        // ("Tidak ada", "Nama Produk"),
        // ("Produk Ga Jelas Nih", "Nama Produk"),
        // ("asdfgh123", "Nama Produk"),
        // ("Lorem Ipsum Dolor", "Nama Produk"),
        // ("Sepatu Anti Gravitasi Alien", "Nama Produk"),
        // ("Teh Racun Bahagia", "Nama Produk"),
        // ("Hand Sanitizer Bau Busuk", "Nama Produk"),
        // ("Makanan Tidak Enak Co.", "Nama Produk"),
        // ("Obat Sihir Instan", "Nama Produk"),
        // ("Snack Setan Tersenyum", "Nama Produk"),
        // ("Kaos Tuhan Marah", "Nama Produk"),
   
    //    // ✅ Contoh inputan VALID
    //    ("Pantai Parangtritis", "Nama Lokasi"),
    //    ("Gunung Bromo", "Nama Lokasi"),
    //    ("Candi Prambanan", "Nama Lokasi"),
    //    ("Taman Mini Indonesia Indah", "Nama Lokasi"),
    //    ("Bandara Soekarno-Hatta", "Nama Lokasi"),
    //    ("Monumen Nasional", "Nama Lokasi"),
    //    ("Kawah Ijen", "Nama Lokasi"),
    //    ("Danau Toba", "Nama Lokasi"),
    //    ("Kebun Raya Bogor", "Nama Lokasi"),
    //    ("Alun-Alun Bandung", "Nama Lokasi"),
   
       // ❌ Contoh inputan TIDAK VALID
    //    ("Lokasi", "Nama Lokasi"),
    //    ("Tempat Saya", "Nama Lokasi"),
    //    ("asdf qwerty", "Nama Lokasi"),
    //    ("Tidak diketahui", "Nama Lokasi"),
    //    ("Nama Tempat", "Nama Lokasi"),
    //    ("-", "Nama Lokasi"),
    //    ("Tempat Umum", "Nama Lokasi"),
    //    ("Lorem Ipsum Place", "Nama Lokasi"),

       

    //     // 🔹 Nama Lengkap
        // ("Kemas Khairunsyah", "Nama Lengkap"),
        // ("Siti Nurhaliza", "Nama Lengkap"),
        // ("Muhammad Al Fatih", "Nama Lengkap"),  
        // ("asdf qwerty", "Nama Lengkap"),
        // ("John123", "Nama Lengkap"),
        // ("!!! ???", "Nama Lengkap"),
        // ("Uvuvwevwevwe Onyetenyevwe Ugwemubwem Osas", "Nama Lengkap"),
        // ("XYZ", "Nama"),
    //     ("X Æ A-Xii Musk", "Nama Lengkap"),

    //     // ✅ Judul
    //     ("Analisis Pengaruh Media Sosial terhadap Perilaku Remaja", "Judul"),
    //     ("Penerapan Teknologi AI dalam Dunia Pendidikan", "Judul"),
    //     ("Dampak Perubahan Iklim terhadap Ketahanan Pangan di Indonesia", "Judul"),
    //     ("Desain Aplikasi Mobile untuk Manajemen Keuangan Pribadi", "Judul"),
    //     ("Studi Kasus Implementasi Blockchain pada Sistem Logistik", "Judul"),
    //     ("Pemanfaatan Energi Terbarukan di Sektor Transportasi", "Judul"),
    //     ("Pengaruh Kualitas Layanan terhadap Kepuasan Pelanggan", "Judul"),
    //     ("Strategi Pemasaran Digital di Era Industri 4.0", "Judul"),
    //     ("Rancang Bangun Sistem Informasi Akademik Berbasis Web", "Judul"),
    //     ("Eksplorasi Potensi Wisata Alam di Kawasan Timur Indonesia", "Judul"),

    //     // ❌ Contoh inputan TIDAK VALID
    //     ("Judul Skripsi", "Judul"),
    //     ("Penelitian Saya", "Judul"),
    //     ("asdf qwerty", "Judul"),
    //     ("Project", "Judul"),
    //     ("Tidak ada", "Judul"),
    //     ("-", "Judul"),
    //     ("Lorem Ipsum Dolor", "Judul"),
    //     ("Topik Penelitian", "Judul"),
    //     ("Untitled", "Judul"),

    //         // ✅ Contoh inputan VALID
    //     ("Guru Sekolah Dasar", "Pekerjaan"),
    //     ("Dosen Informatika", "Pekerjaan"),
    //     ("Software Engineer", "Pekerjaan"),
    //     ("Dokter Spesialis Anak", "Pekerjaan"),
    //     ("Perawat Rumah Sakit", "Pekerjaan"),
    //     ("Desainer Grafis", "Pekerjaan"),
    //     ("Analis Keuangan", "Pekerjaan"),
    //     ("Pegawai Negeri Sipil", "Pekerjaan"),
    //     ("Petani Sayur", "Pekerjaan"),
    //     ("Wirausahawan", "Pekerjaan"),

    //     // ❌ Contoh inputan TIDAK VALID
    //     ("Pekerjaan Saya", "Pekerjaan"),
    //     ("Kerja", "Pekerjaan"),
    //     ("asdf qwerty", "Pekerjaan"),
    //     ("Tidak ada", "Pekerjaan"),
    //     ("-", "Pekerjaan"),
    //     ("Lorem Ipsum", "Pekerjaan"),
    //     ("Job", "Pekerjaan"),
    //     ("Belum bekerja", "Pekerjaan"),

    //         // ✅ Contoh inputan VALID
    //     ("Teknologi", "Tag"),
    //     ("Pendidikan", "Tag"),
    //     ("Kesehatan", "Tag"),
    //     ("AI", "Tag"),
    //     ("Machine Learning", "Tag"),
    //     ("Pemrograman", "Tag"),
    //     ("Bisnis Digital", "Tag"),
    //     ("Lingkungan", "Tag"),
    //     ("Desain UI/UX", "Tag"),
    //     ("Ekonomi Kreatif", "Tag"),

    //     // ❌ Contoh inputan TIDAK VALID
    //     ("Tag", "Tag"),
    //     ("asdf qwerty", "Tag"),
    //     ("#", "Tag"),
    //     ("Tidak ada", "Tag"),
    //     ("Lorem Ipsum", "Tag"),
    //     ("12345", "Tag"),
    //     ("Topik", "Tag"),
    //     ("-", "Tag"),
    //     ("#randomtext", "Tag"),

    //     // Alamat
    //     ("Jl. Merpati No. 45, Kel. Sukamaju, Kec. Cilodong, Depok", "Alamat"),
    //     ("Perumahan Griya Asri Blok B2 No. 10, Bandung", "Alamat"),
    //     ("Jl. Raya Malioboro No. 123, Yogyakarta", "Alamat"),
    //     ("Dusun Karanganyar RT 02 RW 03, Desa Margomulyo, Sleman", "Alamat"),
    //     ("Komplek Taman Anggrek, Blok A3 No. 5, Jakarta Barat", "Alamat"),
    
    //     ("Depok", "Alamat"),
    //     ("Jalan", "Alamat"),
    //     ("asdf qwerty", "Alamat"),
    //     ("No Address", "Alamat"),
    //     ("###@@@!!!", "Alamat"),
    //     ("Jl", "Alamat"),
    //     ("Alamat tidak diketahui", "Alamat"),

            // ✅ Contoh inputan VALID
        // ("Teknologi kecerdasan buatan kini menjadi pusat perhatian di berbagai sektor. Banyak perusahaan mulai menerapkan sistem berbasis AI untuk meningkatkan efisiensi dan mengurangi kesalahan manusia. Dalam dunia pendidikan, AI digunakan untuk membantu guru menganalisis kebutuhan belajar siswa dan memberikan materi yang lebih personal.

        // Selain itu, di bidang kesehatan, algoritma AI mampu mendeteksi penyakit lebih cepat melalui analisis citra medis. Meski begitu, perkembangan ini juga menimbulkan kekhawatiran mengenai etika penggunaan data dan potensi hilangnya lapangan pekerjaan. Oleh karena itu, keseimbangan antara inovasi dan tanggung jawab menjadi hal yang sangat penting.", "Konten"),

        // ("Lingkungan hidup yang berkelanjutan kini menjadi fokus utama banyak negara. Krisis iklim telah memaksa manusia untuk berpikir ulang tentang cara mereka hidup dan berproduksi. Banyak kota besar mulai beralih ke energi terbarukan, sistem transportasi hijau, serta mengurangi penggunaan plastik sekali pakai.

        // Namun, perubahan nyata tidak hanya bergantung pada kebijakan pemerintah, tetapi juga pada kesadaran individu. Dengan memulai dari hal kecil seperti menghemat listrik dan mengurangi sampah, masyarakat bisa berkontribusi besar terhadap masa depan bumi.", "Konten"),

        // ("The rapid growth of social media has changed the way people communicate and consume information. Platforms like Instagram and TikTok have created new opportunities for content creators and businesses to reach global audiences. However, this phenomenon also raises concerns about misinformation and the decline of critical thinking among users.

        // To address these issues, digital literacy education has become crucial. By teaching people how to evaluate online information critically, society can enjoy the benefits of connectivity while minimizing its negative impacts.", "Konten"),

        // ("Mental health awareness has gained significant attention over the past decade. In a world that constantly demands productivity, many individuals struggle with burnout and anxiety. More organizations now recognize the importance of creating a supportive work environment that values rest and emotional well-being.

        // Simple actions such as listening, showing empathy, and encouraging work-life balance can make a huge difference. After all, a healthy mind is the foundation of a meaningful and sustainable life.", "Konten"),

        // ❌ Contoh inputan TIDAK VALID
        // ("asdf qwerty content example", "Konten"),
        // ("Lorem Ipsum Dolor Sit Amet just filler text", "Konten"),
        // ("Artikel saya", "Konten"),
        // ("Just writing something", "Konten"),
        // ("Konten kosong tanpa makna", "Konten"),
        // ("No content available", "Konten"),
        // ("Random words without clear message", "Konten"),
        // ("Test paragraph not real", "Konten"),
        // ("Teknologi kecerdasan buatan kini menjadi pusat perhatian di berbagai sektor. Banyak perusahaan mulai menerapkan sistem berbasis AI untuk meningkatkan efisiensi dan mengurangi kesalahan manusia. Dalam dunia pendidikan, AI digunakan untuk membantu guru menganalisis kebutuhan belajar siswa dan memberikan materi yang lebih personal.

        // Selain itu, di bidang kesehatan, algoritma AI mampu mendeteksi penyakit lebih cepat melalui analisis citra medis. Meski begitu, perkembangan ini juga menimbulkan kekhawatiran mengenai etika penggunaan data dan potensi hilangnya lapangan pekerjaan. Oleh karena itu, keseimbangan antara inovasi dan tanggung jawab menjadi hal yang sangat penting.

        // Teknologi kecerdasan buatan kini menjadi pusat perhatian di berbagai sektor. Banyak perusahaan mulai menerapkan sistem berbasis AI untuk meningkatkan efisiensi dan mengurangi kesalahan manusia. Dalam dunia pendidikan, AI digunakan untuk membantu guru menganalisis kebutuhan belajar siswa dan memberikan materi yang lebih personal.

        // Selain itu, di bidang kesehatan, algoritma AI mampu mendeteksi penyakit lebih cepat melalui analisis citra medis. Meski begitu, perkembangan ini juga menimbulkan kekhawatiran mengenai etika penggunaan data dan potensi hilangnya lapangan pekerjaan. Oleh karena itu, keseimbangan antara inovasi dan tanggung jawab menjadi hal yang sangat penting.", "Konten"),

        //     // ✅ Contoh inputan VALID Cerita
        // ("Fawwaz berlari kecil menuju halte bus di pagi yang cerah. Ia hampir terlambat ke sekolah karena semalam begadang menyelesaikan tugas kelompok. Di jalan, ia melihat seorang nenek yang kesulitan menyeberang. Tanpa pikir panjang, Fawwaz segera membantu, meski tahu bus yang ditunggunya mungkin akan terlewat.

        // Sesampainya di sekolah, ia memang datang sedikit terlambat, tetapi guru yang melihat kebaikannya di jalan justru memujinya. Hari itu Fawwaz belajar bahwa kebaikan kecil kadang datang dengan konsekuensi, tapi selalu layak dilakukan.", "Cerita"),

        // ("Eling menatap layar laptopnya yang sudah menyala sejak pagi. Ia sedang menulis naskah untuk lomba menulis tingkat nasional. Ide sudah ada, tapi kata-kata seolah menolak keluar. Setelah berjam-jam menatap kosong, ia menutup laptop dan berjalan keluar, menikmati udara sore.

        // Di taman kecil dekat rumah, Eling duduk sambil memperhatikan anak-anak bermain. Saat itulah inspirasi datang. Ia tersenyum, pulang ke rumah, dan mulai menulis tanpa henti sampai malam tiba. Kadang, ide terbaik datang saat kita berhenti memaksakan diri.", "Cerita"),

        // ("The rain poured heavily that night, hitting the old roof of Diaz’s house like thousands of tiny drums. He sat quietly by the window, sipping his favorite coffee, staring at the faint glow of the city lights in the distance. He remembered how lively those streets used to be before the flood destroyed half the town.

        // When the rain finally stopped, Diaz stepped outside. The air smelled of wet soil and broken dreams, but somewhere deep inside, he felt a spark of hope. Maybe tomorrow, he thought, he could start rebuilding — not just the house, but his life.", "Cerita"),

        // ("Alya had always wanted to travel alone, but fear kept her tied to routines. One morning, she finally booked a one-way ticket to Bali. The moment the plane took off, she felt a strange mix of excitement and anxiety. Everything was new — the people, the smell of the ocean, the rhythm of life.

        // By the end of the trip, she realized she didn’t find herself by escaping home, but by embracing uncertainty. The world, she thought, was not as scary as the doubts in her head.", "Cerita"),

        // // ❌ Contoh inputan TIDAK VALID
        // ("asdf qwerty lorem ipsum dolor sit amet consectetur adipiscing elit", "Cerita"),
        // ("Cerita saya", "Cerita"),
        // ("Hari ini sangat menyenangkan.", "Cerita"),
        // ("Once upon a time there was something.", "Cerita"),
        // ("-", "Cerita"),
        // ("Tidak ada cerita", "Cerita"),
        // ("Random words no meaning here please test", "Cerita"),

        //     // ✅ Contoh inputan VALID
        // ("Saya setuju dengan isi artikel ini. Kesadaran masyarakat tentang lingkungan memang perlu ditingkatkan sejak dini, terutama di sekolah.", "Komentar"),

        // ("Penjelasan tentang kecerdasan buatan di artikel ini sangat jelas. Saya jadi lebih paham bagaimana AI bisa membantu dalam bidang kesehatan.", "Komentar"),

        // ("Terima kasih sudah menulis topik ini. Banyak orang masih belum sadar pentingnya menjaga keseimbangan antara pekerjaan dan kesehatan mental.", "Komentar"),

        // ("This article provides a balanced perspective. I appreciate how the author includes both the benefits and the ethical challenges of using AI.", "Komentar"),

        // ("Menarik banget pembahasannya! Semoga ke depan bisa dibahas juga dampaknya terhadap lapangan kerja dan pendidikan.", "Komentar"),

        // ("Good insight! I like how you explained the social impact of technology in simple language. Keep up the great work!", "Komentar"),

        // // ❌ Contoh inputan TIDAK VALID
        // ("Artikel ini bodoh, penulisnya gak ngerti apa-apa!", "Komentar"),
        // ("Ngapain bahas hal kayak gini, buang waktu aja.", "Komentar"),
        // ("Dasar orang *** kayak gini emang gak bisa maju.", "Komentar"),
        // ("Penulisnya pasti dari golongan *** makanya bias banget.", "Komentar"),
        // ("This author is an idiot, clearly knows nothing!", "Komentar"),
        // ("Useless article. Only fools would believe this stuff.", "Komentar"),
        // ("Ngomongin lingkungan tapi masih pakai mobil bensin, munafik!", "Komentar"),
        // ("You people should just shut up about this topic.", "Komentar"),
        // ("Tulisan ini cuma propaganda buat kelompok tertentu!", "Komentar"),
        // ("Berhenti sok pintar, tulisan kamu gak ada gunanya.", "Komentar"),

        // // 🔹 Text Area
        // ("Hari ini saya belajar tentang perbedaan HTTP dan HTTPS. HTTPS lebih aman karena menggunakan enkripsi TLS.", "Text Area"),
        // ("Pendidikan karakter sangat penting di sekolah, melalui kegiatan ekstrakurikuler siswa belajar disiplin dan kerja sama.", "Text Area"),
        // ("Membaca buku setiap hari bisa meningkatkan wawasan, kosakata, dan kemampuan berpikir kritis.", "Text Area"),
        // ("Smartphone dengan baterai besar cocok untuk kegiatan luar ruangan, apalagi jika ada fast charging.", "Text Area"),
        // ("Lorem ipsum dolor sit amet", "Text Area"),
        // ("asdf qwerty zxcvbnm", "Text Area"),
        // ("halo halo halo halo halo", "Text Area"),
        // ("1234567890", "Text Area"),
        // ("Blog saya", "Text Area"),

        // // 🔹 Nomor HP
        // ("08123456789", "Nomor HP"),
        // ("+6281234567890", "Nomor HP"),
        // ("6281234567890", "Nomor HP"),
        // ("085612345678", "Nomor HP"),
        // ("1234", "Nomor HP"),
        // ("08abcd1234", "Nomor HP"),
        // ("+62 8123-456-789", "Nomor HP"),
        // ("phone number", "Nomor HP"),
        // ("", "Nomor HP"),
    ];


    let models_to_test = [
        SupportedModel::GeminiFlash,
        // SupportedModel::GeminiFlashLite,
        // SupportedModel::GeminiFlashLatest,
        // SupportedModel::Gemma, //Model Gemma
    ];

    // Iterasi untuk setiap model
    for &model_variant in &models_to_test {
        let model_name = model_variant.as_str();
        println!("\n📋 Menguji Model: {}", model_name);
        println!("----------------------------------------");

        // Iterasi untuk setiap kasus uji input
        for (i, (test_input_str, input_type_str)) in test_inputs.iter().enumerate() {
            println!("\n🔍 Test Case {}: {}", i + 1, input_type_str);
            println!("   Input: \"{}\"", test_input_str);
            println!("   Validasi Sintaksis: ");

            let c_input_text = match CString::new(*test_input_str) {
                Ok(cs) => cs,
                Err(e) => {
                    eprintln!("❌ Error membuat CString untuk input: {}", e);
                    continue;
                }
            };

            let c_input_type = match CString::new(*input_type_str) {
                Ok(cs) => cs,
                Err(e) => {
                    eprintln!("❌ Error membuat CString untuk tipe input: {}", e);
                    continue;
                }
            };

            // Panggil fungsi FFI
            let result_ptr: *mut c_char =
                validate_text_ffi(c_input_text.as_ptr(), model_variant, c_input_type.as_ptr());

            if result_ptr.is_null() {
                eprintln!("❌ FFI mengembalikan pointer null!");
                continue;
            }

            let result_rust_string = unsafe {
                match CStr::from_ptr(result_ptr).to_str() {
                    Ok(s) => s.to_owned(),
                    Err(e) => {
                        eprintln!("❌ Error konversi hasil ke string: {}", e);
                        free_rust_string(result_ptr);
                        continue;
                    }
                }
            };

            // Parse dan tampilkan hasil
            match serde_json::from_str::<ValidationResponse>(&result_rust_string) {
                Ok(parsed_response) => {
                    let status_icon = if parsed_response.valid { "✅" } else { "❌" };
                    println!("   Validasi Semantik :  ");
                    println!("      {} Valid: {}", status_icon, parsed_response.valid);
                    println!("      📝 Message: {}", parsed_response.message);
                }
                Err(e) => {
                    eprintln!("❌ Gagal parse JSON: {}", e);
                    println!("   Raw response: {}", result_rust_string);
                }
            }

            free_rust_string(result_ptr);
        }
    }

    // Pengujian kasus batas
    println!("\n🧪 Pengujian Kasus Batas");
    println!("============================================");

    let null_text_ptr: *const c_char = std::ptr::null();
    let valid_model_for_null_test = SupportedModel::GeminiFlashLatest;
    let example_input_type_str = "Contoh Jenis Input";
    let c_example_input_type = CString::new(example_input_type_str).unwrap();

    // Test dengan teks NULL
    println!("\n🔍 Test Case: Teks NULL");
    let result_ptr_null_text: *mut c_char = validate_text_ffi(
        null_text_ptr,
        valid_model_for_null_test,
        c_example_input_type.as_ptr(),
    );

    if !result_ptr_null_text.is_null() {
        let result_str = unsafe {
            CStr::from_ptr(result_ptr_null_text)
                .to_str()
                .unwrap_or_default()
        };
        println!("   📝 Hasil: {}", result_str);
        free_rust_string(result_ptr_null_text);
    } else {
        println!("   ❌ Hasil: pointer NULL");
    }

    // Test internal logic
    println!("\n🔍 Test Internal Logic");
    let invalid_selector_int_test: i32 = 99;
    match SupportedModel::from_int(invalid_selector_int_test) {
        Some(model) => {
            println!(
                "   ⚠️  from_int({}) -> {:?} (tidak diharapkan)",
                invalid_selector_int_test, model
            );
        }
        None => {
            println!(
                "   ✅ from_int({}) -> None (sesuai harapan)",
                invalid_selector_int_test
            );
        }
    }

    println!("\n🎉 Pengujian FFI Selesai");
    println!("============================================================");
}
