use crate::attribute::Attribute;

#[derive(Clone)]
pub struct Entity {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub rotation: i32,
    pub scale: f32,
    pub idx: u32,
    pub attributes: Vec<Attribute>,
    pub effects: Vec<(Attribute, i32)>,
    pub show_debug: bool,
    pub flip: bool,
    pub invisible: bool,
    pub size: u32,
    pub level_x: i32,
    pub level_y: i32,
    pub level_z: i32,
    pub activate_action: fn(e: Entity) -> Entity,
}

fn dist_squared(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    let dx = x1 - x2;
    let dy = y1 - y2;

    dx * dx + dy * dy
}

impl Entity {
    pub fn level_matches(&self, s: &Entity) -> bool {
        if self.level_x == s.level_x && self.level_y == s.level_y && self.level_z == s.level_z {
            true
        } else {
            false
        }
    }

    pub fn collides_with(&self, s: &Entity) -> bool {
        if self.level_matches(s) && dist_squared(self.x, self.y, s.x, s.y) < 100 {
            true
        } else {
            false
        }
    }
}
