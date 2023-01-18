mod utils;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(module = "/render.js")]
extern "C" {
    fn render(counter: i32) -> String;
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

#[wasm_bindgen(start)]
pub fn run() {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut counter = 0;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let a = render(counter);
        if a.contains("65") {
            counter = 0;
        }
        counter += 1;

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}
