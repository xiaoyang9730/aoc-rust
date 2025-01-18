use std::ops::{Add, Sub};

use super::{PosLists, MapSize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}

impl Pos {
    pub fn is_inside(&self, x: isize, y: isize) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < x && self.y < y
    }
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Sub for Pos {
    type Output = Pos;

    fn sub(self, rhs: Pos) -> Self::Output {
        Pos { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

pub fn parse(input: String) -> (PosLists, MapSize) {
    let mut lists = PosLists::new();
    for (x, line) in input.lines().enumerate() {
        for (y, ch) in line.chars().enumerate() {
            let pos = Pos { x: x as isize, y: y as isize };
            match ch {
                '.' => {},
                freq => {
                    if let Some(list) = lists.get_mut(&freq) {
                        list.push(pos);
                    } else {
                        lists.insert(freq, vec![pos]);
                    }
                },
            }
        }
    }
    (lists, (input.lines().count() as _, input.lines().next().unwrap().len() as _))
}
