pub const COMMON_TEST_CASES: &[(&str, &str)] = &[
    ("test@example.com", "email"),
    ("John Doe", "nama"),
    ("08123456789", "nomor hp indonesia"),
    ("","")
];

pub const COMMON_EMAIL_TESTS: &[(&str, &str, bool)] = &[
    ("test@example.com", "email", true),
    ("user@domain.org", "email", true),
    ("invalid-email", "email", false),
    ("test@", "email", false),
    ("@domain.com", "email", false),
    ("", "email", false), // kosong
];

pub const COMMON_NAME_TESTS: &[(&str, &str, bool)] = &[
    ("John Doe", "nama", true),
    ("Mary Jane", "nama", true),
    ("Jo", "nama", false),       // valid (minimal satu huruf, tidak ada dua spasi berurutan)
    ("A", "nama", false),        // valid (minimal satu huruf, tidak ada dua spasi berurutan)
    ("John123", "nama", true),  // valid (memiliki setidaknya satu huruf)
    ("John  Doe", "nama", false), // tidak valid (dua spasi berurutan)
    ("123", "nama", false),     // tidak valid (tidak ada huruf sama sekali)
    ("  ", "nama", false),      // tidak valid (tidak ada huruf, hanya spasi)
    ("", "nama", false),        // tidak valid (kosong)
];

pub const COMMON_PHONE_TESTS: &[(&str, &str, bool)] = &[
    ("08123456789", "nomor hp indonesia", true),
    ("+628123456789", "nomor hp indonesia", true),
    ("123", "nomor hp", false), // terlalu pendek
    ("081234567890123", "phone", true), // terlalu panjang
    ("0812345678a", "mobile", false), // mengandung huruf
    ("", "nomor hp indonesia", false), // kosong
];

