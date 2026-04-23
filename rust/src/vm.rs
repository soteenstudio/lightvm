use crate::instructions::{
  io::{
    print::print_func,
    println::println_func,
  },
  math::{
    add_func::add_func,
    div_func::div_func,
    mod_func::mod_func,
    mul_func::mul_func,
    sub_func::sub_func,
  },
  comparison::{
    ge_func::ge_func,
    gt_func::gt_func,
    le_func::le_func,
    lt_func::lt_func,
    eq_func::eq_func,
  },
  control_flow::{
    if_false_func::if_false_func,
  },
};
use crate::types::{
  instructions::Instructions,
  value::{FuncMetadata, RunOptions, Value},
};
use crate::utils::compute_hot_threshold::compute_hot_threshold;
use std::collections::{HashMap, HashSet};

pub fn run_bytecode(bytecode: Vec<Instructions>, options: Option<RunOptions>) -> Result<Value, String> {
  let mut last_return = Value::Undefined;
  let mut stack: Vec<Value> = Vec::with_capacity(100);
  let mut vars: HashMap<String, Value> = HashMap::new();
  let mut _call_stack: Vec<usize> = Vec::new();
  let mut hit_counter: HashMap<usize, u32> = HashMap::new();
  let mut ip: usize = options.as_ref().and_then(|o| o.entry).unwrap_or(0);

  let mut functions: HashMap<String, FuncMetadata> = HashMap::new();
  let mut exported: HashSet<String> = HashSet::new();

  for (_i, instr) in bytecode.iter().enumerate() {
    if let Instructions::Func(name, params, start, end, names) = instr {
      functions.insert(
        name.clone(),
        FuncMetadata {
          params_count: *params,
          param_names: names.clone(),
          start: *start,
          end: *end,
        },
      );
    }
    if let Instructions::Export(name) = instr {
      exported.insert(name.clone());
    }
  }

  if let Some(opts) = &options {
    if let Some(entry_ip) = opts.entry {
      let entry_fn = functions.values().find(|f| f.start == entry_ip);
      if let Some(fn_meta) = entry_fn {
        for i in 0..fn_meta.params_count as usize {
          let name = fn_meta
            .param_names
            .get(i)
            .cloned()
            .unwrap_or(format!("param{}", i));
          let val = opts.args.get(i).cloned().unwrap_or(Value::Undefined);
          vars.insert(name, val);
        }
      }
    }
  }

  while ip < bytecode.len() {
    let instr = &bytecode[ip];

    *hit_counter.entry(ip).or_insert(0) += 1;
    let _hot_threshold = compute_hot_threshold(stack.len());

    match instr {
      Instructions::Push(val) => {
        stack.push(val.clone());
      }

      Instructions::Val(name) => {
        if !vars.contains_key(name) {
          vars.insert(name.clone(), Value::Marker("NoInitExpression".to_string()));
        }
      }

      Instructions::Set(name) => {
        if let Some(val) = stack.pop() {
          vars.insert(name.clone(), val);

          if stack.len() > 50 {
            stack.truncate(50);
          }
        } else {
          panic!("Stack underflow on SET");
        }
      }

      Instructions::Get(name) => {
        let val = vars.get(name).cloned().unwrap_or(Value::Undefined);
        stack.push(val);
      }

      Instructions::Add(num_type) => {
        println!("Add jalan");
        let b = stack.pop().ok_or("Stack underflow on ADD (b)")?;
        let a = stack.pop().ok_or("Stack underflow on ADD (a)")?;
        let result = add_func(a, b, num_type.clone());
        stack.push(result);
      }

      Instructions::Sub(num_type) => {
        println!("Sub jalan");
        let b = stack.pop().ok_or("Stack underflow on SUB (b)")?;
        let a = stack.pop().ok_or("Stack underflow on SUB (a)")?;
        let result = sub_func(a, b, num_type.clone());
        stack.push(result);
      }

      Instructions::Mul(num_type) => {
        let b = stack.pop().ok_or("Stack underflow on MUL (b)")?;
        let a = stack.pop().ok_or("Stack underflow on MUL (a)")?;
        let result = mul_func(a, b, num_type.clone());
        stack.push(result);
      }

      Instructions::Div(num_type) => {
        let b = stack.pop().ok_or("Stack underflow on DIV (b)")?;
        let a = stack.pop().ok_or("Stack underflow on DIV (a)")?;
        let result = div_func(a, b, num_type.clone());
        stack.push(result);
      }

      Instructions::Mod(num_type) => {
        let b = stack.pop().ok_or("Stack underflow on MOD (b)")?;
        let a = stack.pop().ok_or("Stack underflow on MOD (a)")?;
        let result = mod_func(a, b, num_type.clone());
        stack.push(result);
      }

      Instructions::Gt(num_type) => {
        let b = stack.pop().ok_or("Stack underflow on GT (b)")?;
        let a = stack.pop().ok_or("Stack underflow on GT (a)")?;
        let result = gt_func(a, b, num_type.clone());
        stack.push(result);
      }

      Instructions::Lt(num_type) => {
        let b = stack.pop().ok_or("Stack underflow on LT (b)")?;
        let a = stack.pop().ok_or("Stack underflow on LT (a)")?;
        let result = lt_func(a, b, num_type.clone());
        stack.push(result);
      }

      Instructions::Ge(num_type) => {
        let b = stack.pop().ok_or("Stack underflow on GE (b)")?;
        let a = stack.pop().ok_or("Stack underflow on GE (a)")?;
        let result = ge_func(a, b, num_type.clone());
        stack.push(result);
      }

      Instructions::Le(num_type) => {
        let b = stack.pop().ok_or("Stack underflow on LE (b)")?;
        let a = stack.pop().ok_or("Stack underflow on LE (a)")?;
        let result = le_func(a, b, num_type.clone());
        stack.push(result);
      }
      
      Instructions::Eq(num_type) => {
        let b = stack.pop().ok_or("Stack underflow on EQ (b)")?;
        let a = stack.pop().ok_or("Stack underflow on EQ (a)")?;
        let result = eq_func(a, b, num_type.clone());
        stack.push(result);
      }

      Instructions::Print => {
        if let Some(val) = stack.pop() {
          print_func(val);
        } else {
          panic!("Stack underflow on PRINT");
        }
      }

      Instructions::Println => {
        if let Some(val) = stack.pop() {
          println_func(val);
        } else {
          panic!("Stack underflow on PRINTLN");
        }
      }
      
      Instructions::IfFalse(target_ip) => {
        let cond = stack.pop().ok_or("Stack underflow on IF_FALSE")?;
        
        if if_false_func(cond) {
          ip = *target_ip;
          continue;
        }
      }
      
      Instructions::Jump(target_ip) => {
          // Logika cleanStack lo: batasi stack biar gak bengkak pas looping
          if stack.len() > 50 {
              stack.truncate(50);
          }
      
          ip = *target_ip;
          continue; // Langsung gass ke alamat baru
      }
      
      Instructions::Return => {
          // Ambil hasil perhitungan terakhir dari stack
          if let Some(result) = stack.pop() {
              last_return = result.clone();
              
              // Bersihin sisa-sisa "sampah" di stack milik scope fungsi ini
              // biar balik ke pemanggil dengan stack yang bersih + hasil return
              stack.push(result); 
          }
      
          if let Some(return_addr) = _call_stack.pop() {
              ip = return_addr + 1;
              continue;
          } else {
              break;
          }
      }

      Instructions::Call(name, argc) => {
          // 1. Cari metadata fungsinya
          println!("{} {}", name, argc);
          let fn_meta = functions.get(name).ok_or(&format!("Function {} not found", name))?
            .clone();
      
          // 2. Ambil argumen dari stack (LIFO, jadi pake pop)
          let mut args = Vec::new();
          for _ in 0..*argc {
              args.push(stack.pop().ok_or("Stack underflow on CALL arguments")?);
          }
          // Karena di-pop, urutannya kebalik. Kita reverse biar sesuai index param.
          args.reverse();
      
          // 3. Simpan posisi IP sekarang ke call_stack buat balik nanti
          _call_stack.push(ip);
      
          // 4. CleanStack logic
          if stack.len() > 50 {
              stack.truncate(50);
          }
      
          // 5. Masukin argumen ke vars lokal
          for i in 0..fn_meta.params_count as usize {
              let param_name = fn_meta.param_names.get(i).cloned()
                  .unwrap_or(format!("param{}", i));
              
              let val = args.get(i).cloned().unwrap_or(Value::Undefined);
              vars.insert(param_name, val);
          }
      
          // 6. Loncat ke alamat awal fungsi
          // Di Rust loop lo pake ip += 1 di akhir, jadi kita set ke (start - 1)
          // supaya pas iterasi berikutnya pas di instruksi pertama.
          ip = fn_meta.start; 
          continue; // Langsung gass ke instruksi pertama fungsi
      }

      Instructions::Stop => {
        // Logic cleanStack()
        if stack.len() > 50 {
          stack.truncate(50);
        }

        // Logic pop call_stack dan pindah IP
        if let Some(return_addr) = _call_stack.pop() {
          ip = return_addr + 1;
          continue; 
        } else {
          break; // Kalau call_stack abis, stop eksekusi
        }
      }
      
      _ => { /* Handle opcode lainnya */ }
    }

    ip += 1;
  }

  if options.as_ref().map_or(false, |o| o.capture_return) {
    return Ok(last_return);
  }
  Ok(Value::Undefined)
}
