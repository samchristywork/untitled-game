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

#[derive(Serialize, Deserialize, PartialEq)]
enum Attributes {
    Consumable,
    Consumed,
    Harmful,
    Player,
    Static,
}

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(module = "/public/render.js")]
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
    attributes: Vec<Attributes>,
    show_debug: bool,
}

fn dist_squared(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    let dx = x1 - x2;
    let dy = y1 - y2;

    dx * dx + dy * dy
}

impl Sprite {
    fn collides_with(&self, s: &Sprite) -> bool {
        if dist_squared(self.x, self.y, s.x, s.y) < 100 {
            true
        } else {
            false
        }
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut sprites = vec![
        Sprite {
            name: "Human".to_string(),
            x: 10,
            y: 10,
            rotation: 0,
            idx: 6,
            behavior: Behavior::Controllable,
            attributes: vec![Attributes::Player],
            show_debug: true,
        },
        Sprite {
            name: "Heart".to_string(),
            x: 100,
            y: 20,
            rotation: 0,
            idx: 7,
            behavior: Behavior::Static,
            attributes: vec![Attributes::Consumable],
            show_debug: false,
        },
    ];
    for i in 0..10 {
        sprites.push(Sprite {
            name: "TEST".to_string(),
            x: 100 + 40 * i,
            y: 250,
            rotation: 0,
            idx: i as u32,
            behavior: Behavior::Static,
            attributes: vec![Attributes::Static],
            show_debug: false,
        });
    }

    let mut rng = rand::thread_rng();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        while sprites
            .iter()
            .filter(|e| e.behavior == Behavior::Dynamic)
            .count()
            < 10
        {
            sprites.push(Sprite {
                name: "Arrow".to_string(),
                x: rng.gen_range(-100..100) - 100,
                y: rng.gen_range(-100..100) + 150,
                rotation: 1,
                idx: 1,
                behavior: Behavior::Dynamic,
                attributes: vec![Attributes::Harmful, Attributes::Consumable],
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
