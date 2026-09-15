# LVM017 (Nilai Tidak Valid)
Tipe eror runtime: `InvalidValue`.

## Penyebab

Error ini terjadi ketika bytecode mencoba mendorong atau mengoperasikan nilai yang tidak dikenali atau didukung oleh VM. Nilai tersebut mungkin menggunakan tipe data yang tidak didukung, rusak, atau memiliki format literal yang tidak valid.

## Pesan runtime

Runtime melaporkan `Invalid value '<value>' encountered`, dengan `<value>` sebagai nama tipe nilai yang tidak valid. Metadata diagnostik menyertakan instruction pointer tempat nilai ditemukan dan tipe eror `InvalidValue`.

## Penyelesaian

Tinjau nilai tersebut dan pastikan tipe data, integritas, dan format literalnya sesuai dengan tipe dan format yang didukung oleh VM.
