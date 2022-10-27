use blake2b_rs::blake2b;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let mut hash = [0u8; 64];
    blake2b(&[], name.as_bytes(), &mut hash);

    alert(&format!(
        "Hello, {}! The blake2b of your name is: {:x?}",
        name, hash
    ));
}
