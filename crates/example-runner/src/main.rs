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

    memory.view(&wasm.store).write(0, bytes).unwrap();
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

fn main() {
    run_add();
    run_length();
    get_double_str();
}
