use crate::attribute::Attribute;
use crate::sprite::Sprite;

pub fn get_sprites() -> Vec<Sprite> {
    let sprites = vec![
        Sprite {
            name: "Human".to_string(),
            x: 10,
            y: 10,
            rotation: 0,
            scale: 1.0,
            idx: 6,
            attributes: vec![Attribute::Player, Attribute::Controllable],
            show_debug: true,
            flip: false,
            invisible: false,
            size: 16,
            level_x: 1,
            level_y: 1,
            level_z: 1,
        },
        Sprite {
            name: "Heart".to_string(),
            x: 450,
            y: 275,
            rotation: 0,
            scale: 1.0,
            idx: 7,
            attributes: vec![Attribute::Healing, Attribute::Consumable],
            show_debug: false,
            flip: false,
            invisible: false,
            size: 16,
            level_x: 1,
            level_y: 1,
            level_z: 1,
        },
        Sprite {
            name: "Snek".to_string(),
            x: 400,
            y: 30,
            rotation: 0,
            scale: 1.0,
            idx: 10,
            attributes: vec![Attribute::Moving, Attribute::Harmful, Attribute::Invisible],
            show_debug: false,
            flip: false,
            invisible: false,
            size: 16,
            level_x: 1,
            level_y: 1,
            level_z: 1,
        },
        Sprite {
            name: "Snek".to_string(),
            x: 400,
            y: 46,
            rotation: 0,
            scale: 1.0,
            idx: 10,
            attributes: vec![
                Attribute::Moving,
                Attribute::Harmful,
                Attribute::Hastened,
                Attribute::Stunning,
            ],
            show_debug: false,
            flip: false,
            invisible: false,
            size: 16,
            level_x: 1,
            level_y: 1,
            level_z: 1,
        },
        Sprite {
            name: "Snek".to_string(),
            x: 400,
            y: 62,
            rotation: 0,
            scale: 1.0,
            idx: 10,
            attributes: vec![Attribute::Moving, Attribute::Harmful, Attribute::Slowed],
            show_debug: false,
            flip: false,
            invisible: false,
            size: 16,
            level_x: 1,
            level_y: 1,
            level_z: 1,
        },
        Sprite {
            name: "Wand".to_string(),
            x: 100,
            y: 50,
            rotation: 0,
            scale: 1.0,
            idx: 11,
            attributes: vec![Attribute::Consumable],
            show_debug: false,
            flip: false,
            invisible: false,
            size: 16,
            level_x: 1,
            level_y: 1,
            level_z: 1,
        },
        Sprite {
            name: "A".to_string(),
            x: 20,
            y: 10,
            rotation: 0,
            scale: 1.0,
            idx: 32,
            attributes: vec![],
            show_debug: false,
            flip: false,
            invisible: false,
            size: 16,
            level_x: 1,
            level_y: 1,
            level_z: 1,
        },
    ];

    sprites
}
