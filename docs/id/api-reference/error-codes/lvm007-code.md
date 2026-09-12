# LVM007 (Fitur Dibatasi)
Tipe eror runtime: `FeatureRestricted`.

## Penyebab

Error ini terjadi ketika bytecode menggunakan opcode nightly atau eksperimental saat mode nightly dinonaktifkan.

## Pesan runtime

Pesan runtime melaporkan `feature` yang dibatasi dan instruction pointer tempat fitur tersebut digunakan.

## Penyelesaian

Aktifkan mode nightly dalam `VmConfig` jika fitur yang dibatasi memang diperlukan, atau ganti dengan instruksi stabil.
