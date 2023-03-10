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
            activate_action: (format!(""), |e, _| (e, Vec::new())),
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
            activate_action: (format!(""), |e, _| (e, Vec::new())),
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
            activate_action: (format!(""), |e, _| (e, Vec::new())),
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
            activate_action: (format!(""), |e, _| (e, Vec::new())),
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
            activate_action: (format!(""), |e, _| (e, Vec::new())),
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
            activate_action: (format!(""), |e, _| (e, Vec::new())),
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
            activate_action: (format!("Stone"), |e, linked| {
                let mut ret = Vec::new();

                for x in linked {
                    let mut y = x.clone();

                    if e.has(AttributeType::On) {
                        y.invisible = true;
                        y.attributes
                            .retain(|e| !(e.kind == AttributeType::Blocking));
                    } else {
                        y.invisible = false;
                        y.attributes.push(Attribute {
                            kind: AttributeType::Blocking,
                        });
                    }
                    ret.push(y);
                }

                (e, ret)
            }),
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
            activate_action: (format!(""), |e, _| (e, Vec::new())),
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
            activate_action: (format!(""), |e, _| (e, Vec::new())),
        },
    ];

    entities
}
