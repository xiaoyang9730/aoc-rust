use super::{Map, Guard};

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}

impl Pos {
    pub fn is_inside(&self, map: &Map) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < map.len() as _ && self.y < map[0].len() as _
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir { #[default] Up, Down, Left, Right, }

pub fn parse(input: String) -> (Map, Guard) {
    let mut obstacles = vec![];
    let mut guard = Guard::default();
    for (x, line) in input.lines().enumerate() {
        let mut obstacles_line = vec![];
        for (y, ch) in line.chars().enumerate() {
            match ch {
                '#' => {
                    obstacles_line.push(true);
                },
                '.' => {
                    obstacles_line.push(false);
                },
                '^' => {
                    obstacles_line.push(false);
                    guard.pos = Pos { x: x as _, y: y as _ };
                },
                _ => panic!("Wrong format for map"),
            }
        }
        obstacles.push(obstacles_line);
    }
    (obstacles, guard)
}
