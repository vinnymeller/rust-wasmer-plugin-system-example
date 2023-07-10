use bincode::{deserialize, serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use wasmer::{imports, Instance, Memory, MemoryType, Module, Pages, Store, TypedFunction, Value};

static WASM: &[u8] =
    include_bytes!("../../../target/wasm32-unknown-unknown/debug/example_plugin.wasm");

struct WASMThing {
    instance: Instance,
    store: Store,
}

fn get_wasm() -> WASMThing {
    let mut store = Store::default();
    let module = Module::new(&store, &WASM).unwrap();
    let import_object = imports! {};
    let instance = Instance::new(&mut store, &module, &import_object).unwrap();
    WASMThing { instance, store }
}

fn run_add() {
    let mut wasm = get_wasm();
    let add = wasm.instance.exports.get_function("add").unwrap();
    let result = add
        .call(&mut wasm.store, &[Value::I32(1), Value::I32(2)])
        .unwrap();
    println!("1 + 2 = {:?}", result);
}

fn run_length() {
    let mut wasm = get_wasm();
    let memory = wasm.instance.exports.get_memory("memory").unwrap();
    let s = "supercalifragilisticexpialidocious".to_string();
    let bytes = s.as_bytes();
    let len = bytes.len();
    let len = bytes.len();

    memory.view(&wasm.store).write(1, bytes).unwrap();
    let length = wasm.instance.exports.get_function("_length").unwrap();
    let wasm_len = match length.call(&mut wasm.store, &[Value::I32(1), Value::I32(len as i32)]) {
        Ok(l) => l.get(0).unwrap().unwrap_i32(),
        Err(e) => {
            println!("error: {:?}", e);
            return;
        }
    };
    println!("original length: {}", len);
    println!("wasm length: {:?}", wasm_len);
}

fn get_double_str() {
    let mut wasm = get_wasm();
    let memory = wasm.instance.exports.get_memory("memory").unwrap();
    let s = "supercalifragilisticexpialidocious".to_string();
    let bytes = s.as_bytes();
    let len = bytes.len();
    memory.view(&wasm.store).write(1, bytes).unwrap();
    let double = wasm.instance.exports.get_function("_double").unwrap();
    let double_ptr = match double.call(&mut wasm.store, &[Value::I32(1), Value::I32(len as i32)]) {
        Ok(l) => l.get(0).unwrap().unwrap_i32(),
        Err(e) => {
            println!("error: {:?}", e);
            return;
        }
    };
    // create empty byte array of 2*len
    let mut byte_buf = vec![0; len * 2];
    memory
        .view(&wasm.store)
        .read(double_ptr as _, &mut byte_buf)
        .unwrap();
    let double_str = String::from_utf8_lossy(&byte_buf);
    println!("original: {}", s);
    println!("wasm str: {}", double_str);
    println!("2x orign: {}", s.repeat(2));
}

fn get_double_str_dont_know_length() {
    let mut wasm = get_wasm();
    let memory = wasm.instance.exports.get_memory("memory").unwrap();
    let s = "supercalifragilisticexpialidocious".to_string();
    let bytes = s.as_bytes();
    let len = bytes.len();
    // set first 4 bytes to 0
    memory.view(&wasm.store).write(1, &[0, 0, 0, 0]).unwrap();
    memory.view(&wasm.store).write(5, bytes).unwrap();
    let double = wasm.instance.exports.get_function("_double_nolen").unwrap();
    let double_ptr = match double.call(&mut wasm.store, &[Value::I32(5), Value::I32(len as i32)]) {
        Ok(l) => l.get(0).unwrap().unwrap_i32(),
        Err(e) => {
            println!("error: {:?}", e);
            return;
        }
    };
    // get the length written to the first 4 bytes
    let mut new_len_bytes = [0u8; 4];
    memory
        .view(&wasm.store)
        .read(1, &mut new_len_bytes)
        .unwrap();
    let new_len = u32::from_ne_bytes(new_len_bytes);
    // read the string now
    let mut byte_buf = vec![0; new_len as usize];
    memory
        .view(&wasm.store)
        .read(double_ptr as _, &mut byte_buf)
        .unwrap();
    let new_str = String::from_utf8_lossy(&byte_buf);
    println!("original: {}", s);
    println!("wasm str: {}", new_str);
}

fn get_multiple_with_bincode() {
    let mut wasm = get_wasm();
    let memory = wasm.instance.exports.get_memory("memory").unwrap();
    memory.view(&wasm.store).write(0, &[0, 0, 0, 0]).unwrap();
    let s = "repeated string ".to_string();
    let now = SystemTime::now();
    let diff = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let u = ((diff.as_millis() % 10) + 1) as u8;
    let pair = (u, s);
    let bytes = serialize(&pair).expect("Failed to serialize tuple");
    let len = bytes.len();
    memory.view(&wasm.store).write(4, &bytes).unwrap();
    let multiply = wasm.instance.exports.get_function("_multiply").unwrap();
    let multiply_start =
        match multiply.call(&mut wasm.store, &[Value::I32(4), Value::I32(len as _)]) {
            Ok(l) => l.get(0).unwrap().unwrap_i32(),
            Err(e) => {
                println!("error: {:?}", e);
                return;
            }
        };
    let mut new_len_bytes = [0u8; 4];
    memory
        .view(&wasm.store)
        .read(0, &mut new_len_bytes)
        .unwrap();
    let new_len = u32::from_ne_bytes(new_len_bytes);
    let mut byte_buf = vec![0; new_len as usize];
    memory
        .view(&wasm.store)
        .read(multiply_start as _, &mut byte_buf)
        .unwrap();
    let new_pair: (u8, String) = deserialize(&byte_buf).expect("Failed to deserialize tuple");
    println!("original: {:?}", pair);
    println!("wasm str: {:?}", new_pair);
}

fn main() {
    run_add();
    run_length();
    get_double_str();
    get_double_str_dont_know_length();
    get_multiple_with_bincode();
}
