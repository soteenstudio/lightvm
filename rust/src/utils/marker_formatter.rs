use colored::*;
use regex::Regex; // Perlu tambah 'colored = "2"' di Cargo.toml

pub fn marker_formatter(text: String) -> String {
  let marker_lists = vec!["NoInitExpression", "NoValueExpression"];

  // Cek apakah text mengandung salah satu marker
  let contains_marker = marker_lists.iter().any(|&m| text.contains(m));

  // Cek apakah bukan string literal (nggak diawali & diakhiri kutip)
  let is_not_quoted = !(text.starts_with('"') && text.ends_with('"'));

  if contains_marker && is_not_quoted {
    // Regex buat ngehapus pola {alphanumeric}
    // Kita pakai lazy_static atau Regex::new. Untuk simpelnya kita new di sini.
    let re = Regex::new(r"\{[a-zA-Z0-9]+\}").unwrap();

    // Hapus pattern-nya
    let cleaned_text = re.replace_all(&text, "").to_string();

    // Return dalam bentuk warna abu-abu (gray) ala chalk
    return cleaned_text.bright_black().to_string();
  }

  text
}
