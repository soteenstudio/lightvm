use crate::types::instructions::Instructions;
use serde_json::Value;

pub fn parse_ltc(code: &str) -> Vec<Instructions> {
    let re = regex::Regex::new(r"\s;; IP=(\d+)").unwrap();
    let cleaned_code = re.replace_all(code, "");

    cleaned_code
        .split(';')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let op = parts[0].to_string();
            
            let mut args: Vec<Value> = parts[1..]
                .iter()
                .map(|&arg| {
                    if let Ok(num) = arg.parse::<f64>() {
                        Value::from(num)
                    } else {
                        Value::from(arg)
                    }
                })
                .collect();

            while args.len() < 4 {
                args.push(Value::from(""));
            }

            // Di sini lo perlu sesuaikan mapping ke enum Instructions lo
            // Ini contoh mapping kasarnya:
            Instructions::from_parts(op, args)
        })
        .collect()
}

pub fn stringify_ltc(instructions: Vec<Instructions>) -> String {
    instructions
        .iter()
        .enumerate()
        .map(|(i, instr)| {
            let parts = instr.to_parts(); // Asumsi lo punya method buat balikin Vec<String>
            format!("{}; ;; IP={}", parts.join(" "), i)
        })
        .collect::<Vec<String>>()
        .join("\n")
}
