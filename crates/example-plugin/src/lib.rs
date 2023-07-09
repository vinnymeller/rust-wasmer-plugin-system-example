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
