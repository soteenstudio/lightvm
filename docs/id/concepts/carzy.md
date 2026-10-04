# Carzy (Pembuat Assembly)
**Carzy** adalah modul assembly dan pembuatan kode tingkat rendah bawaan **LightVM**. Modul ini menyediakan pembantu untuk teks assembly, simbol, bagian memori (*text*, *data*, *rodata*), konstanta I/O, dan instruksi khusus arsitektur.

## Cara Kerja Carzy
Carzy menggunakan arsitektur dua lapis: **Core Assembly Engine (`AsmBuilder`)** mengelola pemformatan teks dan buffer string, sedangkan **Architecture-Specific Builder (`AArch64Builder`)** membungkusnya dengan pembantu instruksi. Builder ini menghasilkan teks assembly tanpa memvalidasi instruksi atau operand.

  * **Buffer Assembly Inti (`AsmBuilder`)**: Menulis langsung ke buffer `String` menggunakan makro `write!` dan `writeln!` serta trait `std::fmt::Write`. `build()` mengembalikan teks, dan `write_to_file()` menulisnya ke file.
  * **Sanitasi Nama Alokasi**: `AsmBuilder::alloc()` mengganti karakter yang bukan alfanumerik atau garis bawah dengan `_` pada nama alokasi. `label()`, `global()`, dan `symbol_type()` menulis nama sesuai masukan.
  * **Injeksi Konstanta I/O Global**: Menghasilkan konstanta untuk baris baru, teks `16`, penanda objek/array, dan urutan pembersihan layar ANSI.
  * **Pembantu Instruksi Arsitektur**: `AArch64Builder` menyediakan metode seperti `mov`, `add`, `sub`, `ldr`, `str`, dan `ret` untuk memformat teks instruksi.
  * **Manajemen Bagian Memori**: Menghasilkan direktif `.text`, `.data`, dan `.section .rodata`.
  * **Pelarian String (*Escaping*)**: `alloc()` melakukan escaping pada garis miring terbalik, tanda kutip, baris baru, carriage return, dan tab untuk nilai `PrimitiveTypes::Str`.

::: info
Kamu bisa menemukan cara menggunakan Carzy pada halaman [Metode Compile](../api-reference/method-functions/compile-method).
:::
