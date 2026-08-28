# Batas Waktu Eksekusi
Batas eksekusi di LightVM dikelola melalui prasetel batas waktu untuk mencegah *infinite loop* (pengulangan tanpa henti) dan skrip yang berjalan di luar kendali. Gunakan tabel berikut untuk memahami tingkat durasi dan batasan eksekusi yang tersedia:

| Level Batas Waktu | Durasi | Deskripsi |
| :--- | :--- | :--- |
| `Cheap` **(Default)** | ~200ms | Dioptimalkan untuk eksekusi skrip yang cepat, ringan, dan validasi kilat. |
| `Normal` | ~1000ms | Batas standar yang cocok untuk sebagian besar aplikasi umum. |
| `Expensive` | ~5000ms | Jendela eksekusi diperpanjang untuk komputasi berat atau logika kompleks. |

::: info
**Time Budget** dan **Max Ticks** tidak akan saling berkonflik, karena **Time Budget** tidak digunakan pada saat eksekusi berlangsung.
:::

::: warning Peringatan Performa
Pilih level batas waktu dengan bijak. Mengatur batas waktu yang terlalu tinggi untuk skrip yang tidak tepercaya dapat berisiko menyebabkan kehabisan sumber daya atau *thread* yang menggantung.
:::