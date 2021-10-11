mod utils;

use std::panic;
use utils::*;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn gray_scale(_array: &[u8]) -> Result<(), JsValue> {
    let mut img = load_image_from_array(_array);
    img = img.grayscale();
    let base64_str = get_image_as_base64(img);
    return append_img(base64_str);
}

/**
 * 入口方法
 */
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    set_panic_hook();
    console::log_1(&JsValue::from_str("Hello wasm!"));

    Ok(())
}
