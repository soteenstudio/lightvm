# Konfigurasi Rust
Halaman ini menyediakan referensi komprehensif untuk semua opsi konfigurasi, batasan, dan kontrol keamanan yang tersedia saat menginisialisasi instance `LightVM`.
## Contoh Konfigurasi Lengkap
Contoh berikut mendemonstrasikan cara mengonfigurasi semua batas, bendera keamanan, dan opsi error yang tersedia menggunakan pola builder maupun object.
::: code-group
<<< @/examples/configuration-reference/builder_pattern.rs{rs:line-numbers}[Builder Pattern]
<<< @/examples/configuration-reference/object_pattern.rs{rs:line-numbers}[Object Pattern]
:::
## Rincian Opsi Konfigurasi

| Method / Properti | Tipe | Default | Deskripsi |
| :--- | :--- | :--- | :--- |
| `caps` | `Capability[]` | `[]` | Menentukan kapabilitas aktif yang diberikan ke virtual machine. |
| `set_max_io` / `max_io` | `number` | `100` | Jumlah maksimum operasi I/O yang diizinkan selama eksekusi. |
| `set_max_import` / `max_import` | `number` | `3` | Jumlah maksimum impor modul yang diizinkan. |
| `set_max_alloc` / `max_alloc` | `number` | `50` | Jumlah maksimum alokasi memori yang diizinkan. |
| `set_max_call` / `max_call` | `number` | `200` | Jumlah maksimum pemanggilan fungsi bersarang yang diizinkan. |
| `set_maxJump` / `max_jump` | `number` | `100` | Jumlah maksimum lompatan control flow yang diizinkan. |
| `set_max_ticks` / `max_ticks` | `number` | `1_000_000` | Jumlah maksimum tick eksekusi sebelum berhenti untuk mencegah infinite loop. |
| `set_max_stack_size` / `max_stack_size` | `number` | `128` | Jumlah maksimum item yang dapat ditampung oleh evaluation stack. |
| `set_allowed_imports` / `allowed_imports` | `string[]` | `[]` | Daftar putih nama modul yang diizinkan untuk diimpor. |
| `set_time_budget` / `time_budget` | `TimeBudget` | `TimeBudget.Cheap` | Menetapkan batas tingkatan anggaran waktu eksekusi. |
| `with_unsafe_mode` / `unsafe_mode` | `boolean` | `false` | Mengaktifkan atau menonaktifkan operasi tingkat sistem yang tidak aman (unsafe). |
| `with_nightly` / `nightly` | `boolean` | `false` | Mengizinkan penggunaan fitur nightly eksperimental. |
| `with_backtrace` / `backtrace` | `boolean` | `false` | Menampilkan detail backtrace internal dalam pesan error. |
| `with_explain` / `explain` | `boolean` | `false` | Menampilkan petunjuk penjelasan yang lebih rinci dalam pesan error. |
| `with_hint` / `hint` | `boolean` | `true` | Menampilkan petunjuk penggunaan umum pada pesan error. |
| `with_diagnostic_links` / `diagnostic_links` | `boolean` | `true` | Menyertakan tautan metadata yang mengarah ke dokumentasi kode error. |

::: info Dokumentasi Terkait
Perlu memeriksa kode error individual yang disebutkan dalam pesan error? Kunjungi [Referensi Kode Error](/api-reference/error-codes/lvm001-code).
:::