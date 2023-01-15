mod utils;

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    // Alert
    alert("Hello, World!");

    // Create element
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let val = document.create_element("div").unwrap();
    val.set_text_content(Some("Hello, World!"));

    body.append_child(&val).unwrap();
}

#[wasm_bindgen(start)]
pub fn run() {
    log("Hello, World!");
}
