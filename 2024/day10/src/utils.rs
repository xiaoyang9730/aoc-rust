use std::ops::Add;

use super::Map;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}

impl Pos {
    pub fn is_inside(&self, map: &Map) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < map.len() as _ && self.y < map[0].len() as _
    }
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}
