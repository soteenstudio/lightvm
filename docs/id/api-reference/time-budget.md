# Batas Waktu Eksekusi
Batas eksekusi di LightVM dikelola melalui prasetel batas waktu untuk mencegah *infinite loop* (pengulangan tanpa henti) dan skrip yang berjalan di luar kendali. Gunakan tabel berikut untuk memahami tingkat durasi dan batasan eksekusi yang tersedia:

| Level Batas Waktu | Maks Ticks | Deskripsi |
| :--- | :--- | :--- |
| `Cheap` **(Default)** | 200 ticks | Dioptimalkan untuk eksekusi skrip yang cepat, ringan, dan validasi kilat. |
| `Normal` | 1000 ticks | Batas standar yang cocok untuk sebagian besar aplikasi umum. |
| `Expensive` | 5000 ticks | Jendela eksekusi diperpanjang untuk komputasi berat atau logika kompleks. |

::: info
**Time Budget** diberlakukan selama eksekusi oleh GasMonitor, yang memeriksa jumlah tick pada setiap instruksi VM. Baik `set_time_budget` maupun `set_max_ticks` mengatur batas `max_ticks` yang sama, sehingga metode yang dipanggil paling terakhir akan menentukan batas eksekusi efektif.
:::

::: warning Peringatan Performa
Pilih level batas waktu dengan bijak. Mengatur batas waktu yang terlalu tinggi untuk skrip yang tidak tepercaya dapat berisiko menyebabkan kehabisan sumber daya atau *thread* yang menggantung.
:::