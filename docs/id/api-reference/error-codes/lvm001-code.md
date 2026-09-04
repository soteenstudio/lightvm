# LVM001 (Stack Overflow)
Runtime error type: `StackOverflow`.

Error ini terjadi ketika tumpukan mencapai batas maksimum yang ditentukan oleh `InitStack` atau batas tumpukan bawaan. Pesan runtime melaporkan `limit` yang dikonfigurasi dan instruction pointer tempat batas tersebut tercapai.

Periksa panggilan fungsi rekursif tanpa henti. Jika kedalaman tumpukan yang diperlukan valid, konfigurasikan `InitStack` agar menyediakan ruang tumpukan yang memadai.
