pub mod instructions;
pub mod optimizer;
pub mod types;
pub mod utils;
pub mod vm;
use std::collections::{HashMap, HashSet};
use std::fs;
use napi::threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode};
use napi_derive::napi;
use crate::types::{
    instructions::Instructions,
    vmevent::VmEvent,
    vmstate::VmState,
    capability::Capability,
    value::{Value,RunOptions, FuncMetadata},
};
use crate::vm::execute::execute;

#[napi(js_name = "LightVM")]
pub struct LightVM {
    bytecode: Vec<Instructions>,
    listeners: HashMap<VmEvent, Vec<ThreadsafeFunction<String>>>,
    caps: HashSet<Capability>,
    state: VmState,
    _outputs: Vec<String>,
    last_value: Value,
    functions: HashMap<String, FuncMetadata>,
    exported: HashSet<String>,
    _imports: HashMap<String, Value>,
}

#[napi]
impl LightVM {
    #[napi(constructor)]
    pub fn new(caps: Vec<Capability>) -> Self {
        let mut caps_set = HashSet::new();
        if caps.is_empty() {
            caps_set.insert(Capability::Observe);
        } else {
          for c in caps { caps_set.insert(c); }
        }

        Self {
            bytecode: Vec::new(),
            listeners: HashMap::new(),
            caps: caps_set,
            state: VmState::Idle,
            _outputs: Vec::new(),
            last_value: Value::Undefined,
            functions: HashMap::new(),
            exported: HashSet::new(),
            _imports: HashMap::new(),
        }
    }

    fn require(&self, cap: Capability) -> Result<(), napi::Error> {
        if !self.caps.contains(&cap) {
            return Err(napi::Error::from_reason(format!("Capability '{:?}' not granted", cap)));
        }
        Ok(())
    }

#[napi]
pub fn load(&mut self, source: napi::JsUnknown) -> Result<(), napi::Error> {
    // Paksa jadi string dulu biar aman di Termux
    let json_str = source.coerce_to_string()?.into_utf8()?.as_str()?.to_string();
    
    let val: serde_json::Value = serde_json::from_str(&json_str)
        .map_err(|e| napi::Error::from_reason(format!("JSON Parse Error: {}", e)))?;

    if let Some(s) = val.as_str() {
        let path = std::path::Path::new(s);
        let code = if path.exists() {
            fs::read_to_string(path).map_err(|e| napi::Error::from_reason(e.to_string()))?
        } else {
            s.to_string()
        };
        self.bytecode = crate::utils::loader::parse_ltc(&code);
    } else {
        self.bytecode = serde_json::from_value(val)
            .map_err(|e| napi::Error::from_reason(format!("Invalid bytecode: {}", e)))?;
    }

    self.index_metadata();
    Ok(())
}

    fn index_metadata(&mut self) {
        self.functions.clear();
        self.exported.clear();

        for instr in &self.bytecode {
            match instr {
                Instructions::Func(name, params, start, end, names) => {
                    self.functions.insert(name.clone(), FuncMetadata {
                        params_count: *params,
                        param_names: names.clone(),
                        start: *start,
                        end: *end,
                    });
                }
                Instructions::Export(name) => {
                    self.exported.insert(name.clone());
                }
                _ => {}
            }
        }
    }

#[napi]
pub fn run(&mut self, options: napi::JsUnknown) -> Result<(), napi::Error> {
    self.require(Capability::Control)?;
    
    // Cara "Ninja" buat dapet RunOptions tanpa berantem sama trait NapiValue
    let run_opts: Option<RunOptions> = match options.get_type()? {
        napi::ValueType::Null | napi::ValueType::Undefined => None,
        _ => {
            // Ubah JsUnknown jadi JSON string, terus parse pake serde_json ke RunOptions
            let json_str = options.coerce_to_string()?.into_utf8()?.as_str()?.to_string();
            serde_json::from_str::<RunOptions>(&json_str).ok()
        }
    };

    if self.bytecode.is_empty() {
        return Err(napi::Error::from_reason("No bytecode loaded"));
    }

    self.state = VmState::Running;
    self.emit(VmEvent::Tick, serde_json::json!({ "state": "start" }));

    // execute sekarang nerima Option<RunOptions> hasil parse manual kita
    match execute(self.bytecode.clone(), run_opts) {
        Ok(val) => {
            self.last_value = val;
            self.state = VmState::Halted;
            self.emit(VmEvent::Halt, serde_json::Value::Null);
            Ok(())
        }
        Err(e) => {
            self.emit(VmEvent::Panic, serde_json::json!({ "error": e }));
            Err(napi::Error::from_reason(e))
        }
    }
}

#[napi]
pub fn on(&mut self, event: VmEvent, callback: napi::JsFunction) -> Result<(), napi::Error> {
    // Jangan pake Fatal, pake yang default aja biar cocok sama isi HashMap lo
    let tsfn: ThreadsafeFunction<String, napi::threadsafe_function::ErrorStrategy::CalleeHandled> = 
        callback.create_threadsafe_function(0, |ctx| {
            Ok(vec![ctx.value])
        })?;
    
    self.listeners.entry(event).or_insert_with(Vec::new).push(tsfn);
    Ok(())
}
#[napi]
pub fn provide(&mut self, name: String, value: napi::JsUnknown) -> Result<(), napi::Error> {
    self.require(Capability::Control)?;
    
    // Convert JsUnknown ke internal Value lo (pake JSON stringify/parse biar aman)
    let json_str = value.coerce_to_string()?.into_utf8()?.as_str()?.to_string();
    let val: Value = serde_json::from_str(&json_str)
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        
    self._imports.insert(name, val);
    Ok(())
}
    // Private helper, nggak perlu #[napi]
    fn emit(&self, event: VmEvent, payload: serde_json::Value) {
        if let Some(list) = self.listeners.get(&event) {
            let json_payload = payload.to_string();
            for tsfn in list {
                tsfn.call(Ok(json_payload.clone()), ThreadsafeFunctionCallMode::Blocking);
            }
        }
    }
#[napi]
pub fn inspect(&self) -> Result<String, napi::Error> {
    self.require(Capability::Observe)?;
    let info = serde_json::json!({
        "state": format!("{:?}", self.state),
        "instructions": self.bytecode.len(),
        "capabilities": self.caps.iter().collect::<Vec<_>>(),
        "functions": self.functions.len(),
        "exported": self.exported.iter().collect::<Vec<_>>()
    });
    Ok(info.to_string())
}

#[napi]
pub fn halt(&mut self) -> Result<(), napi::Error> {
    self.require(Capability::Unsafe)?; // Sesuaikan dengan acuan lo
    self.state = VmState::Halted;
    self.emit(VmEvent::Halt, serde_json::Value::Null);
    Ok(())
}
#[napi]
pub fn call_exported(&mut self, name: String, args_raw: napi::JsUnknown) -> Result<String, napi::Error> {
    self.require(Capability::Control)?;
    
    if !self.exported.contains(&name) {
        return Err(napi::Error::from_reason(format!("Function '{}' is not exported", name)));
    }

    let fn_meta = self.functions.get(&name)
        .ok_or_else(|| napi::Error::from_reason(format!("Function '{}' not found", name)))?;

    // Parse args dari JS
    let json_args = args_raw.coerce_to_string()?.into_utf8()?.as_str()?.to_string();
    let args: Vec<Value> = serde_json::from_str(&json_args)
        .map_err(|e| napi::Error::from_reason(format!("Invalid args: {}", e)))?;

    self.state = VmState::Running;
    
    let options = RunOptions {
        entry: Some(fn_meta.start as usize),
        args,            // Langsung masukin Vec-nya, jangan pake Some() kalau di struct-nya bukan Option
        capture_return: true, // Langsung bool kalau di struct-nya bool
    };

    match execute(self.bytecode.clone(), Some(options)) {
        Ok(val) => {
            self.state = VmState::Halted;
            self.last_value = val.clone();
            Ok(serde_json::to_string(&val).unwrap())
        }
        Err(e) => {
            self.state = VmState::Halted; // Pake Halted dulu kalau Panic belum ada di enum
            Err(napi::Error::from_reason(e))
        }
    }
}

#[napi]
pub fn get_outputs(&self) -> Result<Vec<String>, napi::Error> {
    self.require(Capability::Observe)?;
    Ok(self._outputs.clone())
}

#[napi]
pub fn clear_outputs(&mut self) -> Result<(), napi::Error> {
    self.require(Capability::Control)?;
    self._outputs.clear();
    Ok(())
}

}
