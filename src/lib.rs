mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, ImageData};

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

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let mut data: Vec<u8> = Vec::new();
    for _ in 1..1000000 {
        data.push(0);
    }

    let width = 100;
    let height = 100;

    let mut data = foo(width, height);
    let data =
        ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height).unwrap();
    context.put_image_data(&data, 10.0, 0.0);

    context.begin_path();
    context.move_to(0.0, 0.0);
    context.line_to(100.0, 100.0);
    context.stroke();
}

fn foo(width: u32, height: u32) -> Vec<u8> {
    let mut data = Vec::new();
    for y in 0..height {
        for x in 0..width {
            data.push(x as u8);
            data.push(y as u8);
            data.push(0);
            data.push(127);
        }
    }
    data
}
