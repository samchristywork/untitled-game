#[derive(PartialEq, Clone)]
pub struct Attribute {
    pub kind: AttributeType,
}

#[derive(PartialEq, Clone)]
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
    Off,
    On,
    Player,
    Slowed,
    Stunned,
    Stunning,
    Toggleable,
}
