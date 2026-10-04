# Dying (Pewarnaan)
**Dying** adalah modul bawaan yang menyediakan konstanta warna dan gaya ANSI bersama untuk diagnostik **LightVM**.

## Cara Kerja Dying
Dying mendefinisikan konstanta string yang dapat digunakan kembali. [VMError](./vmerror) menggunakan konstanta ini untuk memformat diagnostik dan backtrace internal. Pemanggil menentukan cara menangani error dan apakah proses dihentikan.

  * **Konstanta Warna Bersama**: Menyediakan `RED`, `CYAN`, `GREEN`, `YELLOW`, `BLUE`, `MAGENTA`, `BRIGHT_GREEN`, dan `DARK_GRAY` untuk digunakan pemanggil pada keluaran terminal.
  * **Gaya Teks**: Menyediakan `BOLD` untuk teks tebal dan `RESET` untuk mengatur ulang gaya. Pemanggil menyisipkan urutan ini sesuai kebutuhan.
