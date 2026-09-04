# LVM013 (Batas Lompatan Terlampaui)
Tipe eror runtime: `JumpLimitExceeded`.

Error ini terjadi ketika bytecode melampaui jumlah lompatan alur kontrol yang diizinkan, termasuk instruksi `Jump`, `IfFalse`, dan `Break`. Runtime melaporkan instruction pointer tempat batas terlampaui.

Sederhanakan percabangan dan perulangan bersarang. Jika kompleksitas alur kontrol tersebut diperlukan, tingkatkan `max_jump` dalam `SecurityConfig`.
