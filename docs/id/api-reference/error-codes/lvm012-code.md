# LVM012 (Batas Panggilan Terlampaui)
Runtime error type: `CallLimitExceeded`.

Error ini terjadi ketika bytecode melampaui jumlah instruksi pemanggilan fungsi yang diizinkan. Runtime melaporkan instruction pointer tempat batas terlampaui.

Kurangi jumlah instruksi `Call` dalam bytecode. Jika pola panggilan tersebut sah, tingkatkan `max_call` dalam `SecurityConfig`.
