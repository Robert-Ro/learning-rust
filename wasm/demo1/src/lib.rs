use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn digest(str: &str) -> String {
    let digest_str = md5::compute(str);
    format!("{:x}", digest_str)
}
