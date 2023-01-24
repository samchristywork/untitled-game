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
enum Attribute {
    Blocking,
    Consumable,
    Consumed,
    Harmful,
    Healing,
    Player,
    Static,
}

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(module = "/public/render.js")]
extern "C" {
    fn render(s: String, text: String) -> String;
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
struct Text {
    text: String,
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize)]
struct Sprite {
    name: String,
    x: i32,
    y: i32,
    rotation: i32,
    scale: f32,
    idx: u32,
    behavior: Behavior,
    attributes: Vec<Attribute>,
    show_debug: bool,
    flip: bool,
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
    let mut current_health = 100;

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut sprites = vec![
        Sprite {
            name: "Human".to_string(),
            x: 10,
            y: 10,
            rotation: 0,
            scale: 1.0,
            idx: 6,
            behavior: Behavior::Controllable,
            attributes: vec![Attribute::Player],
            show_debug: true,
            flip: false,
        },
        Sprite {
            name: "Heart".to_string(),
            x: 450,
            y: 275,
            rotation: 0,
            scale: 1.0,
            idx: 7,
            behavior: Behavior::Static,
            attributes: vec![Attribute::Healing, Attribute::Consumable],
            show_debug: false,
            flip: false,
        },
    ];

    for i in 0..12 {
        sprites.push(Sprite {
            name: "TEST".to_string(),
            x: 0 + 16 * i,
            y: 275,
            rotation: 0,
            scale: 1.0,
            idx: i as u32,
            behavior: Behavior::Static,
            attributes: vec![Attribute::Static],
            show_debug: false,
            flip: false,
        });
    }

    for i in 0..4 {
        sprites.push(Sprite {
            name: "Stone".to_string(),
            x: 200,
            y: 100 + 16 * i,
            rotation: 0,
            scale: 1.0,
            idx: 9,
            behavior: Behavior::Static,
            attributes: vec![Attribute::Blocking],
            show_debug: false,
            flip: false,
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
                y: rng.gen_range(-33..33) + 150,
                rotation: 1,
                scale: 1.0,
                idx: 1,
                behavior: Behavior::Dynamic,
                attributes: vec![Attribute::Harmful, Attribute::Consumable],
                show_debug: false,
                flip: false,
            })
        }

        for idx in 0..sprites.len() {
            if sprites[idx].behavior == Behavior::Dynamic {
                sprites[idx].x += 10;
                for idx2 in 0..sprites.len() {
                    if sprites[idx2].attributes.contains(&Attribute::Blocking) {
                        if sprites[idx].collides_with(&sprites[idx2]) {
                            sprites[idx].attributes.push(Attribute::Consumed);
                        }
                    }
                }
            }

            if sprites[idx].attributes.contains(&Attribute::Player) {
                for idx2 in 0..sprites.len() {
                    if sprites[idx2].attributes.contains(&Attribute::Consumable) {
                        if sprites[idx].collides_with(&sprites[idx2]) {
                            sprites[idx2].attributes.push(Attribute::Consumed);
                            if sprites[idx2].attributes.contains(&Attribute::Harmful) {
                                current_health -= 1;
                            }
                            if sprites[idx2].attributes.contains(&Attribute::Healing) {
                                current_health = 100;
                            }
                        }
                    }
                }
            }
        }

        sprites.retain(|e| !e.attributes.contains(&Attribute::Consumed));

        let a = render(
            serde_json::to_string(&sprites).unwrap(),
            serde_json::to_string(&[Text {
                text: format!("Health: {current_health}"),
                x: 430,
                y: 16,
            }])
            .unwrap(),
        );

        for idx in 0..sprites.len() {
            if sprites[idx].behavior == Behavior::Controllable {
                let mut speed = 1;

                let current_x = sprites[idx].x;
                let current_y = sprites[idx].y;

                if a.contains("16") {
                    speed = 3;
                }

                if a.contains("65") {
                    sprites[idx].x -= speed;
                    sprites[idx].flip = true;
                }
                if a.contains("68") {
                    sprites[idx].x += speed;
                    sprites[idx].flip = false;
                }
                if a.contains("87") {
                    sprites[idx].y -= speed;
                }
                if a.contains("83") {
                    sprites[idx].y += speed;
                }

                for idx2 in 0..sprites.len() {
                    if sprites[idx2].attributes.contains(&Attribute::Blocking) {
                        if sprites[idx].collides_with(&sprites[idx2]) {
                            sprites[idx].x = current_x;
                            sprites[idx].y = current_y;
                        }
                    }
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
