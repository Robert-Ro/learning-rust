extern crate base64;
extern crate image;

use base64::encode;
use image::{DynamicImage, ImageFormat};
use std::io::{Cursor, Read, Seek, SeekFrom};
use wasm_bindgen::prelude::*;
use web_sys::window;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
pub fn load_image_from_array(_array: &[u8]) -> DynamicImage {
    let img = match image::load_from_memory_with_format(_array, ImageFormat::Png) {
        Ok(img) => img,
        Err(e) => panic!("{:?}", e),
    };
    return img;
}
pub fn get_image_as_base64(img: DynamicImage) -> String {
    let mut c = Cursor::new(Vec::new());
    match img.write_to(&mut c, ImageFormat::Png) {
        Ok(c) => c,
        Err(e) => panic!("There was a problem writing the resulting buffer: {:?}", e),
    }
    c.seek(SeekFrom::Start(0)).unwrap();
    let mut out = Vec::new();
    c.read_to_end(&mut out).unwrap();
    format!("{}{}", "data:image/png;base64,", encode(&mut out))
}
pub fn append_img(src: String) -> Result<(), JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let img = document.create_element("img").unwrap();
    img.set_attribute("src", &src)?;
    img.set_attribute("style", "height: 200px")?;
    body.append_child(&img)?;

    Ok(())
}
