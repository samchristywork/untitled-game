use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub enum Attribute {
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
