# Torja (Penyelesai Simbol)
**Torja** adalah Symbol Resolver dari **LightVM**. Torja memetakan nama variabel ke indeks numerik yang digunakan mesin eksekusi.

## Cara Kerja Torja
Setiap pemanggilan `resolve_symbols()` membuat satu tabel simbol, memuat nama impor, dan menulis ulang instruksi simbolik yang didukung. Indeks berlaku untuk pemanggilan resolusi tersebut; indeks tidak dijamin sama di antara pemanggilan.

  * **Pemetaan Simbol & Impor**: Memuat nama impor ke tabel simbol di awal dan memberikan indeks pada nama tambahan yang ditemukan dalam bytecode.
  * **Resolusi Dinamis**: `get_or_insert_idx` menggunakan kembali indeks yang ada atau memberikan indeks baru menggunakan `next_idx`.
  * **Instruksi Berbasis Indeks**: Mengonversi `val`, `get`, `set`, `inc`, dan `dec` menjadi pasangan berbasis indeksnya. Spesialisasi `push` berdasarkan tipe dilakukan oleh Gazle.
  * **Nama Parameter Fungsi**: Mendaftarkan nama parameter dari instruksi `Func` dalam tabel simbol yang sama dengan nama lainnya. Torja tidak membuat tabel cakupan leksikal terpisah; nama identik berbagi indeks dalam pemanggilan tersebut.
