# LVM007 (Fitur Dibatasi)
Runtime error type: `FeatureRestricted`.

Error ini terjadi ketika bytecode menggunakan opcode nightly atau eksperimental saat mode nightly dinonaktifkan. Pesan runtime melaporkan `feature` yang dibatasi dan instruction pointer tempat fitur tersebut digunakan.

Aktifkan mode nightly dalam `VmConfig` jika fitur yang dibatasi memang diperlukan, atau ganti dengan instruksi stabil.
