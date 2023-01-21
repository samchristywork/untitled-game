mod utils;

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Serialize, Deserialize, PartialEq)]
enum Behavior {
    Controllable,
    Dynamic,
    Static,
}

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(module = "/render.js")]
extern "C" {
    fn render(s: String) -> String;
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
    behavior: Behavior,
    show_debug: bool,
}

#[wasm_bindgen(start)]
pub fn run() {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut sprites = vec![
        Sprite {
            name: "Arrow".to_string(),
            x: 10,
            y: 10,
            rotation: 1,
            idx: 1,
            behavior: Behavior::Controllable,
            show_debug: true,
        },
        Sprite {
            name: "TEST".to_string(),
            x: 100,
            y: 100,
            rotation: 0,
            idx: 0,
            behavior: Behavior::Static,
            show_debug: true,
        },
    ];

    let mut rng = rand::thread_rng();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        while sprites
            .iter()
            .filter(|e| e.behavior == Behavior::Dynamic)
            .count()
            < 100
        {
            sprites.push(Sprite {
                name: "Arrow".to_string(),
                x: rng.gen_range(-100..100) - 100,
                y: rng.gen_range(-100..100) + 150,
                rotation: 1,
                idx: 1,
                behavior: Behavior::Dynamic,
                show_debug: false,
            })
        }

        for sprite in &mut sprites {
            if sprite.behavior == Behavior::Dynamic {
                sprite.x += 10;
            }
        }

        let a = render(serde_json::to_string(&sprites).unwrap());

        for sprite in &mut sprites {
            if sprite.behavior == Behavior::Controllable {
                if a.contains("65") {
                    sprite.x -= 1;
                }
                if a.contains("68") {
                    sprite.x += 1;
                }
                if a.contains("87") {
                    sprite.y -= 1;
                }
                if a.contains("83") {
                    sprite.y += 1;
                }
            }
        }

        if a.contains("27") {
            return;
        }

        sprites.retain(|e| !(e.behavior == Behavior::Dynamic && e.x > 500));

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}
