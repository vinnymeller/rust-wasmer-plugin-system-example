use wasmer::{imports, Instance, Module, Pages, Store, TypedFunction, Value};

static WASM: &[u8] =
    include_bytes!("../../../target/wasm32-unknown-unknown/debug/example_plugin.wasm");

fn main() {
    let mut store = Store::default();
    let module = Module::new(&store, &WASM).unwrap();
    // module doesn't import anything, so we can just create an empty import object
    let import_object = imports! {};
    let instance = Instance::new(&mut store, &module, &import_object).unwrap();

    let add = instance.exports.get_function("add").unwrap();
    let result = add
        .call(&mut store, &[Value::I32(1), Value::I32(2)])
        .unwrap();
    println!("1 + 2 = {:?}", result);

    let memory = instance.exports.get_memory("memory").unwrap();
    let view = memory.view(&store);
    let s = "supercalifragilisticexpialidocious".to_string();
    let bytes = s.as_bytes();
    let len = bytes.len();
    let len = bytes.len();

    view.write(0, bytes).unwrap();
    let length = instance.exports.get_function("_length").unwrap();
    println!("here");
    let wasm_len = match length.call(&mut store, &[Value::I32(1), Value::I32(len as i32)]) {
        Ok(l) => l.get(0).unwrap().unwrap_i32(),
        Err(e) => {
            println!("error: {:?}", e);
            return;
        }
    };
    println!("original length: {}", len);
    println!("wasm length: {:?}", wasm_len);

    let view2 = memory.view(&store);
    view2.write(0, bytes).unwrap();
    let double = instance.exports.get_function("_double").unwrap();
    let wasm_double_start = match double.call(&mut store, &[Value::I32(1), Value::I32(len as i32)])
    {
        Ok(l) => l.get(0).unwrap().unwrap_i32(),
        Err(e) => {
            println!("error: {:?}", e);
            return;
        }
    };
    println!("original: {}", s);

    let mut byte_buf = vec![0; len * 2];
    // read the wasm string from memory with our start and end
    memory
        .view(&store)
        .read(wasm_double_start as u64, &mut byte_buf)
        .unwrap();

    // convert the bytes to a string
    let wasm_string = String::from_utf8_lossy(&byte_buf);
    println!("wasm: {}", wasm_string);
}
