use crate::attribute::Attribute;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Sprite {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub rotation: i32,
    pub scale: f32,
    pub idx: u32,
    pub attributes: Vec<Attribute>,
    pub show_debug: bool,
    pub flip: bool,
    pub invisible: bool,
    pub size: u32,
    pub level_number: u32,
}

fn dist_squared(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    let dx = x1 - x2;
    let dy = y1 - y2;

    dx * dx + dy * dy
}

impl Sprite {
    pub fn collides_with(&self, s: &Sprite) -> bool {
        if self.level_number == s.level_number && dist_squared(self.x, self.y, s.x, s.y) < 100 {
            true
        } else {
            false
        }
    }
}
