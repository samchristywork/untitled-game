use crate::attribute::Attribute;
use crate::attribute::AttributeType;
use crate::entity::Entity;

pub fn get_sprites() -> Vec<Entity> {
    let entities = vec![
        Entity {
            name: "Human".to_string(),
            x: 10,
            y: 10,
            rotation: 0,
            scale: 1.0,
            idx: 6,
            attributes: vec![
                Attribute {
                    kind: AttributeType::Player,
                },
                Attribute {
                    kind: AttributeType::Controllable,
                },
            ],
            effects: vec![],
            show_debug: true,
            flip: false,
            invisible: false,
            size: 16,
            level_x: 0,
            level_y: 0,
            level_z: 0,
            activate_action: |e, linked| e,
        },
        Entity {
            name: "Heart".to_string(),
            x: 450,
            y: 275,
            rotation: 0,
            scale: 1.0,
            idx: 7,
            attributes: vec![
                Attribute {
                    kind: AttributeType::Healing,
                },
                Attribute {
                    kind: AttributeType::Consumable,
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
            activate_action: |e, linked| e,
        },
        Entity {
            name: "Snek".to_string(),
            x: 400,
            y: 30,
            rotation: 0,
            scale: 1.0,
            idx: 10,
            attributes: vec![
                Attribute {
                    kind: AttributeType::Moving,
                },
                Attribute {
                    kind: AttributeType::Harmful,
                },
                Attribute {
                    kind: AttributeType::Invisible,
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
            activate_action: |e, linked| e,
        },
        Entity {
            name: "Snek".to_string(),
            x: 400,
            y: 46,
            rotation: 0,
            scale: 1.0,
            idx: 10,
            attributes: vec![
                Attribute {
                    kind: AttributeType::Moving,
                },
                Attribute {
                    kind: AttributeType::Harmful,
                },
                Attribute {
                    kind: AttributeType::Hastened,
                },
                Attribute {
                    kind: AttributeType::Stunning,
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
            activate_action: |e, linked| e,
        },
        Entity {
            name: "Snek".to_string(),
            x: 400,
            y: 62,
            rotation: 0,
            scale: 1.0,
            idx: 10,
            attributes: vec![
                Attribute {
                    kind: AttributeType::Moving,
                },
                Attribute {
                    kind: AttributeType::Harmful,
                },
                Attribute {
                    kind: AttributeType::Slowed,
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
            activate_action: |e, linked| e,
        },
        Entity {
            name: "Wand".to_string(),
            x: 100,
            y: 50,
            rotation: 0,
            scale: 1.0,
            idx: 11,
            attributes: vec![Attribute {
                kind: AttributeType::Consumable,
            }],
            effects: vec![],
            show_debug: false,
            flip: false,
            invisible: false,
            size: 16,
            level_x: 0,
            level_y: 0,
            level_z: 0,
            activate_action: |e, linked| e,
        },
        Entity {
            name: "Lever".to_string(),
            x: 140,
            y: 50,
            rotation: 0,
            scale: 1.0,
            idx: 12,
            attributes: vec![
                Attribute {
                    kind: AttributeType::Off,
                },
                Attribute {
                    kind: AttributeType::Toggleable,
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
            activate_action: |mut e, mut linked| {
                e.x += 32;

                for mut x in linked {
                    x.invisible = true;
                }

                e
            },
        },
        Entity {
            name: "A".to_string(),
            x: 20,
            y: 10,
            rotation: 0,
            scale: 1.0,
            idx: 32,
            attributes: vec![],
            effects: vec![],
            show_debug: false,
            flip: false,
            invisible: false,
            size: 16,
            level_x: 0,
            level_y: 0,
            level_z: 0,
            activate_action: |e, linked| e,
        },
        Entity {
            name: "Arrow Source".to_string(),
            x: -100,
            y: 150,
            rotation: 0,
            scale: 1.0,
            idx: 0,
            attributes: vec![Attribute {
                kind: AttributeType::ArrowSource,
            }],
            effects: vec![],
            show_debug: false,
            flip: false,
            invisible: false,
            size: 16,
            level_x: 0,
            level_y: 0,
            level_z: 0,
            activate_action: |e, linked| e,
        },
    ];

    entities
}
