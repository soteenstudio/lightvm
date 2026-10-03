# Carzy
**Carzy** adalah modul pembangunan dan perakitan kode tingkat rendah (Low-Level Assembly & Code Generation) bawaan di dalam **LightVM** yang dirancang untuk menghasilkan instruksi assembly yang bersih, terstruktur, dan aman. Modul ini menyediakan fondasi arsitektur yang fleksibel dan modular guna menangani pembuatan simbol, bagian memori (*text*, *data*, *rodata*), injeksi konstanta I/O, serta manipulasi instruksi khusus arsitektur tanpa memerlukan perombakan dokumentasi yang drastis ketika arsitektur baru ditambahkan.

## Cara Kerja Carzy
Carzy menggunakan mekanisme arsitektur dua lapis (*two-tier architecture*): **Core Assembly Engine (`AsmBuilder`)** sebagai fondasi umum yang menangani format teks, penyandian string, dan penulisan buffer, serta **Architecture-Specific Builder (misalnya `AArch64Builder`)** sebagai lapisan pembungkus (*wrapper*) untuk menerjemahkan perintah spesifik perangkat keras. Pemisahan ini memastikan bahwa logika inti penulisan assembly tetap terpusat, sementara ekspansi ke arsitektur lain cukup dilakukan dengan memperluas kelas pembungkus tanpa mengubah struktur dokumentasi utama.

  * **Buffer Assembly Inti (`AsmBuilder`)**: Mengoptimalkan kinerja pembuatan teks assembly menggunakan manajemen buffer memori langsung berbasis `String` dan trait `writeln!`, secara signifikan mengurangi alokasi ulang memori (*reallocation overhead*) dibandingkan metode pemformatan string sementara (*temporary formatting*).
  * **Sanitasi Nama Simbol Otomatis**: Secara otomatis membersihkan dan memetakan karakter non-alfanumerik pada nama label atau variabel menjadi garis bawah (`_`) untuk mencegah error sintaks pada assembler tingkat rendah.
  * **Injeksi Konstanta I/O Global**: Menyediakan fungsi terpusat untuk menyuntikkan konstanta sistem standar secara instan (seperti karakter baris baru, penanda tipe objek/array, dan kode pembersihan layar ANSI).
  * **Abstraksi Instruksi Arsitektur (`AArch64Builder`, dll.)**: Membungkus instruksi khusus perangkat keras (seperti `mov`, `add`, `sub`, `ldr`, `str`, dan `ret`) ke dalam metode pembantu yang bersih, membuat penulisan kode assembly menyerupai pemanggilan fungsi tingkat tinggi yang aman.
  * **Manajemen Bagian Memori Modular**: Mendefinisikan pemisahan segmen memori secara deklaratif melalui `.text` (kode instruksi), `.data` (variabel termodifikasi), dan `.section .rodata` (data baca-saja/konstanta).
  * **Pelarian String Aman (*Escaping*)**: Menangani karakter khusus seperti garis miring terbalik, tanda kutip, tab, dan baris baru secara otomatis pada tipe data primitif string agar aman saat dikompilasi oleh assembler eksternal.

::: info
Kamu bisa menemukan cara memperluas dan menggunakan Carzy pada halaman [Metode Compile](../api-reference/method-functions/compile-method).
:::
