use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn hello(name: &str) {
    alert(&format!("안녕하세요!, {}!", name));
}