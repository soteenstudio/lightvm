# Konfigurasi TypeScript
Halaman ini menyediakan referensi komprehensif untuk semua opsi konfigurasi, batasan, dan kontrol keamanan yang tersedia saat menginisialisasi instance `LightVM`.
## Contoh Konfigurasi Lengkap
Contoh berikut mendemonstrasikan cara mengonfigurasi semua batas, bendera keamanan, dan opsi error yang tersedia menggunakan pola builder maupun object.
::: code-group
<<< @/examples/configuration-reference/builderPattern.ts{ts:line-numbers}[Builder Pattern]
<<< @/examples/configuration-reference/objectPattern.ts{ts:line-numbers}[Object Pattern]
:::
## Rincian Opsi Konfigurasi

| Method / Properti | Tipe | Default | Deskripsi |
| :--- | :--- | :--- | :--- |
| `caps` | `Capability[]` | `[Capability.Observe]` | Menentukan kapabilitas aktif yang diberikan ke virtual machine. |
| `setMaxIo` / `maxIo` | `number` | `100` | Jumlah maksimum operasi I/O yang diizinkan selama eksekusi. |
| `setMaxImport` / `maxImport` | `number` | `3` | Jumlah maksimum impor modul yang diizinkan. |
| `setMaxAlloc` / `maxAlloc` | `number` | `50` | Jumlah maksimum alokasi memori yang diizinkan. |
| `setMaxCall` / `maxCall` | `number` | `200` | Jumlah maksimum pemanggilan fungsi bersarang yang diizinkan. |
| `setMaxJump` / `maxJump` | `number` | `100` | Jumlah maksimum lompatan control flow yang diizinkan. |
| `setMaxTicks` / `maxTicks` | `number` | `1_000_000` | Jumlah maksimum tick eksekusi sebelum berhenti untuk mencegah infinite loop. |
| `setMaxStackSize` / `maxStackSize` | `number` | `128` | Jumlah maksimum item yang dapat ditampung oleh evaluation stack. |
| `setAllowedImports` / `allowedImports` | `string[]` | `[]` | Daftar putih nama modul yang diizinkan untuk diimpor. |
| `setTimeBudget` / `timeBudget` | `TimeBudget` | `TimeBudget.Cheap` | Menetapkan batas tingkatan anggaran waktu eksekusi. |
| `withUnsafeMode` / `unsafeMode` | `boolean` | `false` | Mengaktifkan atau menonaktifkan operasi tingkat sistem yang tidak aman (unsafe). |
| `withNightly` / `nightly` | `boolean` | `false` | Mengizinkan penggunaan fitur nightly eksperimental. |
| `withBacktrace` / `backtrace` | `boolean` | `false` | Menampilkan detail backtrace internal dalam pesan error. |
| `withExplain` / `explain` | `boolean` | `false` | Menampilkan petunjuk penjelasan yang lebih rinci dalam pesan error. |
| `withHint` / `hint` | `boolean` | `true` | Menampilkan petunjuk penggunaan umum pada pesan error. |
| `withDiagnosticLinks` / `diagnosticLinks` | `boolean` | `true` | Menyertakan tautan metadata yang mengarah ke dokumentasi kode error. |

::: info Dokumentasi Terkait
Perlu memeriksa kode error individual yang disebutkan dalam pesan error? Kunjungi [Referensi Kode Error](/api-reference/error-codes/lvm001-code).
:::
