#[derive(PartialEq)]
pub struct Attribute {
    pub kind: AttributeType,
}

#[derive(PartialEq)]
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
