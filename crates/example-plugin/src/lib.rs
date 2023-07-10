use wasmer_plugin_example_rust::*;

#[no_mangle]
pub fn add(one: i32, two: i32) -> i32 {
    one + two
}

/// this is what we'd write if we were using pure rust, not interacting with wasm
pub fn length(s: &str) -> u32 {
    s.len() as u32
}

/// since we need to translate from wasm to rust, its a big uglier
#[no_mangle]
pub fn _length(ptr: i32, len: u32) -> u32 {
    // extract the string from memory
    let value = unsafe {
        let slice = std::slice::from_raw_parts(ptr as _, len as _);
        String::from_utf8_lossy(slice)
    };
    length(&value)
}

// this is how we'd double a str in pure rust
pub fn double(s: &str) -> String {
    s.repeat(2)
}

// but DIRTY wasm can only use fucking ints so here we go
#[no_mangle]
pub fn _double(ptr: i32, len: u32) -> i32 {
    let value = unsafe {
        let slice = std::slice::from_raw_parts(ptr as _, len as _);
        String::from_utf8_lossy(slice)
    };
    double(&value).as_ptr() as i32
}

#[no_mangle]
pub fn _double_nolen(ptr: i32, len: u32) -> i32 {
    let value = unsafe {
        let slice = std::slice::from_raw_parts(ptr as _, len as _);
        String::from_utf8_lossy(slice)
    };
    let ret = double(&value);
    let len = ret.len() as u32;
    unsafe {
        std::ptr::write(1 as _, len);
    }
    ret.as_ptr() as _
}

// rust version
#[plugin_helper]
pub fn multiply(pair: (u8, String)) -> (u8, String) {
    let s = pair.1.repeat(pair.0 as usize);
    let u = pair.0.wrapping_mul(s.len() as u8);
    (u, s)
}

// commented out because its the version before we made a macro to do the serialize and deserialize for us
// // the wasmified version
// #[no_mangle]
// pub fn _multiply(ptr: i32, len: u32) -> i32 {
//     let slice = unsafe { std::slice::from_raw_parts(ptr as _, len as _) };
//     let pair = deserialize(slice).expect("failed to deserialize tuple");
//     let updated = multiply(pair);
//     let ret = serialize(&updated).expect("failed to serialize tuple");
//     let len = ret.len() as u32;
//     unsafe {
//         std::ptr::write(1 as _, len);
//     }
//     ret.as_ptr() as _
// }
