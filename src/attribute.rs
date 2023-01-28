use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Attribute {
    pub kind: AttributeType,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum AttributeType {
    ArrowSource,
    Blocking,
    Consumable,
    Consumed,
    Controllable,
    Dynamic,
    Harmful,
    Hastened,
    Healing,
    Invisible,
    Moving,
    Player,
    Slowed,
    Stunned,
    Stunning,
}
