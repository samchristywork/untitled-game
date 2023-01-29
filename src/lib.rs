pub mod attribute;
pub mod entity;
pub mod utils;
pub mod world;

use crate::attribute::Attribute;
use crate::attribute::AttributeType;
use crate::entity::Entity;
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

#[derive(Serialize, Deserialize)]
struct Sprite {
    name: String,
    x: i32,
    y: i32,
    rotation: i32,
    scale: f32,
    idx: u32,
    show_debug: bool,
    flip: bool,
    invisible: bool,
    size: u32,
    level_x: i32,
    level_y: i32,
    level_z: i32,
}

impl Sprite {
    fn new_from_entity(e: &Entity) -> Self {
        Self {
            name: format!("{}", e.name),
            x: e.x,
            y: e.y,
            rotation: e.rotation,
            scale: e.scale,
            idx: e.idx,
            show_debug: e.show_debug,
            flip: e.flip,
            invisible: e.invisible,
            size: e.size,
            level_x: e.level_x,
            level_y: e.level_y,
            level_z: e.level_z,
        }
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    let mut current_health = 100;
    let mut level_x = 0;
    let mut level_y = 0;
    let mut level_z = 0;
    let mut statusline = String::new();
    let mut previous_keyboard_state = String::new();

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut entities = world::get_sprites();

    for i in 0..14 {
        entities.push(Entity {
            name: "TEST".to_string(),
            x: 0 + 16 * i,
            y: 275 - 16,
            rotation: 0,
            scale: 1.0,
            idx: i as u32,
            attributes: vec![],
            effects: vec![],
            show_debug: false,
            flip: false,
            invisible: false,
            size: 16,
            level_x: 1,
            level_y: 0,
            level_z: 0,
            activate_action: |e| e,
        });
    }

    for i in 0..4 {
        entities.push(Entity {
            name: "Stone".to_string(),
            x: 200,
            y: 100 + 16 * i,
            rotation: 0,
            scale: 1.0,
            idx: 9,
            attributes: vec![Attribute {
                kind: AttributeType::Blocking,
            }],
            effects: vec![],
            show_debug: false,
            flip: false,
            invisible: false,
            size: 16,
            level_x: 0,
            level_y: 0,
            level_z: 0,
            activate_action: |e| e,
        });
    }

    let mut rng = rand::thread_rng();
    let mut frame = 0;

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        for idx in 0..entities.len() {
            if entities[idx]
                .attributes
                .iter()
                .any(|e| e.kind == AttributeType::ArrowSource)
            {
                while entities
                    .iter()
                    .filter(|e| {
                        e.attributes
                            .iter()
                            .any(|e| e.kind == AttributeType::Dynamic)
                    })
                    .count()
                    < 10
                {
                    entities.push(Entity {
                        name: "Arrow".to_string(),
                        x: rng.gen_range(-100..100) + entities[idx].x,
                        y: rng.gen_range(-33..33) + entities[idx].y,
                        rotation: 1,
                        scale: 1.0,
                        idx: 1,
                        attributes: vec![
                            Attribute {
                                kind: AttributeType::Harmful,
                            },
                            Attribute {
                                kind: AttributeType::Consumable,
                            },
                            Attribute {
                                kind: AttributeType::Dynamic,
                            },
                        ],
                        effects: vec![],
                        show_debug: false,
                        flip: false,
                        invisible: false,
                        size: 16,
                        level_x: 0,
                        level_y: 0,
                        level_z: 0,
                        activate_action: |e| e,
                    })
                }
            }

            for idx2 in 0..entities[idx].effects.len() {
                entities[idx].effects[idx2].1 -= 1;
            }

            entities[idx].effects.retain(|e| e.1 > 0);

            if entities[idx]
                .attributes
                .iter()
                .any(|e| e.kind == AttributeType::Invisible)
            {
                entities[idx].invisible = true;
            }

            if entities[idx]
                .attributes
                .iter()
                .any(|e| e.kind == AttributeType::Moving)
            {
                let mut speed = 1;

                let mut period = 128;

                if entities[idx]
                    .attributes
                    .iter()
                    .any(|e| e.kind == AttributeType::Hastened)
                {
                    speed = 2;
                    period = 64;
                }

                if entities[idx]
                    .attributes
                    .iter()
                    .any(|e| e.kind == AttributeType::Slowed)
                {
                    period = 256;
                    if frame % 2 == 0 {
                        continue;
                    }
                }

                if frame % (period * 2) >= period {
                    entities[idx].x += speed;
                    entities[idx].flip = false;
                } else {
                    entities[idx].x -= speed;
                    entities[idx].flip = true;
                }
            }

            // Dynamic
            if entities[idx]
                .attributes
                .iter()
                .any(|e| e.kind == AttributeType::Dynamic)
            {
                let mut speed = 10;

                if entities[idx]
                    .attributes
                    .iter()
                    .any(|e| e.kind == AttributeType::Hastened)
                {
                    speed *= 2;
                }

                if entities[idx]
                    .attributes
                    .iter()
                    .any(|e| e.kind == AttributeType::Slowed)
                {
                    speed /= 2;
                }

                entities[idx].x += speed;
                for idx2 in 0..entities.len() {
                    if entities[idx2]
                        .attributes
                        .iter()
                        .any(|e| e.kind == AttributeType::Blocking)
                    {
                        if entities[idx].collides_with(&entities[idx2]) {
                            entities[idx].attributes.push(Attribute {
                                kind: AttributeType::Consumed,
                            });
                        }
                    }
                }
            }

            // Player
            if entities[idx]
                .attributes
                .iter()
                .any(|e| e.kind == AttributeType::Player)
            {
                // Stunning
                for idx2 in 0..entities.len() {
                    if entities[idx2]
                        .attributes
                        .iter()
                        .any(|e| e.kind == AttributeType::Stunning)
                    {
                        if entities[idx].collides_with(&entities[idx2]) {
                            if !entities[idx]
                                .effects
                                .iter()
                                .any(|e| e.0.kind == AttributeType::Stunned)
                            {
                                entities[idx].effects.push((
                                    Attribute {
                                        kind: AttributeType::Stunned,
                                    },
                                    50,
                                ));
                            }
                        }
                    }
                }

                // Harm
                for idx2 in 0..entities.len() {
                    if entities[idx2]
                        .attributes
                        .iter()
                        .any(|e| e.kind == AttributeType::Harmful)
                    {
                        if entities[idx].collides_with(&entities[idx2]) {
                            current_health -= 1;
                        }
                    }
                }

                // Healing
                for idx2 in 0..entities.len() {
                    if entities[idx2]
                        .attributes
                        .iter()
                        .any(|e| e.kind == AttributeType::Healing)
                    {
                        if entities[idx].collides_with(&entities[idx2]) {
                            current_health = 100;
                        }
                    }
                }

                // Consuming
                for idx2 in 0..entities.len() {
                    if entities[idx2]
                        .attributes
                        .iter()
                        .any(|e| e.kind == AttributeType::Consumable)
                    {
                        if entities[idx].collides_with(&entities[idx2]) {
                            entities[idx2].attributes.push(Attribute {
                                kind: AttributeType::Consumed,
                            });
                        }
                    }
                }
            }
        }

        entities.retain(|e| {
            !e.attributes
                .iter()
                .any(|e| e.kind == AttributeType::Consumed)
        });

        let text = [
            Text {
                text: format!("Health: {current_health}"),
                x: 430,
                y: 16,
            },
            Text {
                text: format!("Pos: {level_x}, {level_y}, {level_z}"),
                x: 430,
                y: 32,
            },
            Text {
                text: format!("{statusline}"),
                x: 0,
                y: 290,
            },
        ];

        let keyboard_state = render(
            serde_json::to_string(
                &entities
                    .iter()
                    .filter(|e| {
                        e.level_x == level_x && e.level_y == level_y && e.level_z == level_z
                    })
                    .collect::<Vec<&Entity>>()
                    .iter()
                    .map(|e| Sprite::new_from_entity(e))
                    .collect::<Vec<Sprite>>(),
            )
            .unwrap(),
            serde_json::to_string(&text).unwrap(),
        );

        for idx in 0..entities.len() {
            if entities[idx]
                .attributes
                .iter()
                .any(|e| e.kind == AttributeType::Controllable)
            {
                let mut speed = 1;

                let current_x = entities[idx].x;
                let current_y = entities[idx].y;

                if keyboard_state.contains("16") {
                    speed = 3;
                }

                if entities[idx]
                    .attributes
                    .iter()
                    .any(|e| e.kind == AttributeType::Hastened)
                {
                    speed *= 2;
                }

                if entities[idx]
                    .attributes
                    .iter()
                    .any(|e| e.kind == AttributeType::Slowed)
                {
                    speed /= 2;
                }

                if entities[idx]
                    .effects
                    .iter()
                    .any(|e| e.0.kind == AttributeType::Stunned)
                {
                    speed = 0;
                }

                if keyboard_state.contains("65") {
                    entities[idx].x -= speed;
                    entities[idx].flip = true;
                }
                if keyboard_state.contains("68") {
                    entities[idx].x += speed;
                    entities[idx].flip = false;
                }
                if keyboard_state.contains("87") {
                    entities[idx].y -= speed;
                }
                if keyboard_state.contains("83") {
                    entities[idx].y += speed;
                }

                if entities[idx].level_x == level_x
                    && entities[idx].level_y == level_y
                    && entities[idx].level_z == level_z
                {
                    if entities[idx].y > 300 {
                        entities[idx].y = 0;
                        level_y += 1;
                    }
                    if entities[idx].y < 0 {
                        entities[idx].y = 300;
                        level_y -= 1;
                    }
                    if entities[idx].x > 500 {
                        entities[idx].x = 0;
                        level_x += 1;
                    }
                    if entities[idx].x < 0 {
                        entities[idx].x = 500;
                        level_x -= 1;
                    }

                    entities[idx].level_x = level_x;
                    entities[idx].level_y = level_y;
                    entities[idx].level_z = level_z;
                }

                // Change Level
                if keyboard_state.contains("78") && !previous_keyboard_state.contains("78") {
                    level_x += 1;
                }
                if keyboard_state.contains("80") && !previous_keyboard_state.contains("80") {
                    level_x -= 1;
                }

                // Action
                if keyboard_state.contains("69") && !previous_keyboard_state.contains("69") {
                    for idx2 in 0..entities.len() {
                        if entities[idx].collides_with(&entities[idx2]) {
                            if entities[idx2]
                                .attributes
                                .iter()
                                .any(|e| e.kind == AttributeType::Toggleable)
                            {
                                if entities[idx2]
                                    .attributes
                                    .iter()
                                    .any(|e| e.kind == AttributeType::Off)
                                {
                                    entities[idx2].idx += 1;
                                    entities[idx2].attributes.push(Attribute {
                                        kind: AttributeType::On,
                                    });
                                    entities[idx2]
                                        .attributes
                                        .retain(|e| !(e.kind == AttributeType::Off));

                                    let e =
                                        (entities[idx2].activate_action)(entities[idx2].clone());
                                    entities[idx2] = e;
                                } else {
                                    entities[idx2].idx -= 1;
                                    entities[idx2].attributes.push(Attribute {
                                        kind: AttributeType::Off,
                                    });
                                    entities[idx2]
                                        .attributes
                                        .retain(|e| !(e.kind == AttributeType::On));
                                }
                            }
                        }
                    }
                }

                // Blocking
                for idx2 in 0..entities.len() {
                    if entities[idx2]
                        .attributes
                        .iter()
                        .any(|e| e.kind == AttributeType::Blocking)
                    {
                        if entities[idx].collides_with(&entities[idx2]) {
                            entities[idx].x = current_x;
                            entities[idx].y = current_y;
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

        entities.retain(|e| {
            !(e.attributes
                .iter()
                .any(|e| e.kind == AttributeType::Dynamic)
                && e.x > 500)
        });

        frame += 1;

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}
