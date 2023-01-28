pub mod attribute;
pub mod sprite;
pub mod utils;
pub mod world;

use crate::attribute::Attribute;
use crate::sprite::Sprite;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

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

#[wasm_bindgen(start)]
pub fn run() {
    let mut current_health = 100;
    let mut level_number = 1;
    let mut statusline = String::new();
    let mut previous_keyboard_state = String::new();

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut sprites = world::get_sprites();

    for i in 0..12 {
        sprites.push(Sprite {
            name: "TEST".to_string(),
            x: 0 + 16 * i,
            y: 275 - 16,
            rotation: 0,
            scale: 1.0,
            idx: i as u32,
            attributes: vec![],
            show_debug: false,
            flip: false,
            invisible: false,
            size: 16,
            level_number: 2,
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
            attributes: vec![Attribute::Blocking],
            show_debug: false,
            flip: false,
            invisible: false,
            size: 16,
            level_number: 1,
        });
    }

    let mut rng = rand::thread_rng();
    let mut frame = 0;

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        while sprites
            .iter()
            .filter(|e| e.attributes.contains(&Attribute::Dynamic))
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
                attributes: vec![
                    Attribute::Harmful,
                    Attribute::Consumable,
                    Attribute::Dynamic,
                ],
                show_debug: false,
                flip: false,
                invisible: false,
                size: 16,
                level_number: 1,
            })
        }

        for idx in 0..sprites.len() {
            if sprites[idx].attributes.contains(&Attribute::Invisible) {
                sprites[idx].invisible = true;
            }

            if sprites[idx].attributes.contains(&Attribute::Moving) {
                let mut speed = 1;

                let mut period = 128;

                if sprites[idx].attributes.contains(&Attribute::Hastened) {
                    speed = 2;
                    period = 64;
                }

                if sprites[idx].attributes.contains(&Attribute::Slowed) {
                    period = 256;
                    if frame % 2 == 0 {
                        continue;
                    }
                }

                if frame % (period * 2) >= period {
                    sprites[idx].x += speed;
                    sprites[idx].flip = false;
                } else {
                    sprites[idx].x -= speed;
                    sprites[idx].flip = true;
                }
            }

            // Dynamic
            if sprites[idx].attributes.contains(&Attribute::Dynamic) {
                let mut speed = 10;

                if sprites[idx].attributes.contains(&Attribute::Hastened) {
                    speed *= 2;
                }

                if sprites[idx].attributes.contains(&Attribute::Slowed) {
                    speed /= 2;
                }

                sprites[idx].x += speed;
                for idx2 in 0..sprites.len() {
                    if sprites[idx2].attributes.contains(&Attribute::Blocking) {
                        if sprites[idx].collides_with(&sprites[idx2]) {
                            sprites[idx].attributes.push(Attribute::Consumed);
                        }
                    }
                }
            }

            // Player
            if sprites[idx].attributes.contains(&Attribute::Player) {
                // Stunning
                for idx2 in 0..sprites.len() {
                    if sprites[idx2].attributes.contains(&Attribute::Stunning) {
                        if sprites[idx].collides_with(&sprites[idx2]) {
                            if !sprites[idx].attributes.contains(&Attribute::Stunned) {
                                sprites[idx].attributes.push(Attribute::Stunned);
                            }
                        }
                    }
                }

                // Harm
                for idx2 in 0..sprites.len() {
                    if sprites[idx2].attributes.contains(&Attribute::Harmful) {
                        if sprites[idx].collides_with(&sprites[idx2]) {
                            current_health -= 1;
                        }
                    }
                }

                // Healing
                for idx2 in 0..sprites.len() {
                    if sprites[idx2].attributes.contains(&Attribute::Healing) {
                        if sprites[idx].collides_with(&sprites[idx2]) {
                            current_health = 100;
                        }
                    }
                }

                // Consuming
                for idx2 in 0..sprites.len() {
                    if sprites[idx2].attributes.contains(&Attribute::Consumable) {
                        if sprites[idx].collides_with(&sprites[idx2]) {
                            sprites[idx2].attributes.push(Attribute::Consumed);
                        }
                    }
                }
            }
        }

        sprites.retain(|e| !e.attributes.contains(&Attribute::Consumed));

        let text = [
            Text {
                text: format!("Health: {current_health}"),
                x: 430,
                y: 16,
            },
            Text {
                text: format!("{statusline}"),
                x: 0,
                y: 290,
            },
        ];

        let keyboard_state = render(
            serde_json::to_string(
                &sprites
                    .iter()
                    .filter(|e| e.level_number == level_number)
                    .collect::<Vec<&Sprite>>(),
            )
            .unwrap(),
            serde_json::to_string(&text).unwrap(),
        );

        for idx in 0..sprites.len() {
            if sprites[idx].attributes.contains(&Attribute::Controllable) {
                let mut speed = 1;

                let current_x = sprites[idx].x;
                let current_y = sprites[idx].y;

                if keyboard_state.contains("16") {
                    speed = 3;
                }

                if sprites[idx].attributes.contains(&Attribute::Hastened) {
                    speed *= 2;
                }

                if sprites[idx].attributes.contains(&Attribute::Slowed) {
                    speed /= 2;
                }

                if keyboard_state.contains("65") {
                    sprites[idx].x -= speed;
                    sprites[idx].flip = true;
                }
                if keyboard_state.contains("68") {
                    sprites[idx].x += speed;
                    sprites[idx].flip = false;
                }
                if keyboard_state.contains("87") {
                    sprites[idx].y -= speed;
                }
                if keyboard_state.contains("83") {
                    sprites[idx].y += speed;
                }

                if keyboard_state.contains("78") && !previous_keyboard_state.contains("78") {
                    level_number += 1;
                }
                if keyboard_state.contains("80") && !previous_keyboard_state.contains("80") {
                    level_number -= 1;
                }

                // Blocking
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

        statusline = String::new() + keyboard_state.as_str();

        if keyboard_state.contains("27") {
            return;
        }

        previous_keyboard_state = String::new() + keyboard_state.as_str();

        sprites.retain(|e| !(e.attributes.contains(&Attribute::Dynamic) && e.x > 500));

        frame += 1;

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}
