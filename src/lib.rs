mod utils;

use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(module = "/render.js")]
extern "C" {
    fn render(x: i32, y: i32, s: String) -> String;
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn status() {
    log("OK");
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}

#[derive(Serialize, Deserialize)]
struct Sprite {
    name: String,
    x: i32,
    y: i32,
    rotation: i32,
    idx: u32,
}

#[wasm_bindgen(start)]
pub fn run() {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut x = 0;
    let mut y = 0;
    let mut sprites = vec![
        Sprite {
            name: "Arrow".to_string(),
            x: 10,
            y: 10,
            rotation: 1,
            idx: 1,
        },
        Sprite {
            name: "TEST".to_string(),
            x: 100,
            y: 100,
            rotation: 0,
            idx: 0,
        },
    ];

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let a = render(x, y, serde_json::to_string(&sprites).unwrap());

        if a.contains("65") {
            sprites[0].x -= 1;
        }
        if a.contains("68") {
            sprites[0].x += 1;
        }
        if a.contains("87") {
            sprites[0].y -= 1;
        }
        if a.contains("83") {
            sprites[0].y += 1;
        }

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}
