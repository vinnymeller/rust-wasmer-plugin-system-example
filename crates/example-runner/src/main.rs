use wasmer::{imports, Instance, Module, Store, Value};

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
}
