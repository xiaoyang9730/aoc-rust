use std::env::args;
use std::fs::File;
use std::io::read_to_string;
use std::ops::Add;
use std::collections::HashSet;

type Map = Vec<Vec<bool>>;

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2 { x: isize, y: isize }

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

#[derive(Debug, Default, Clone, Copy)]
enum Dir { #[default] Up, Down, Left, Right, }

impl Dir {
    fn turn_right(&mut self) {
        *self = match self {
            Dir::Up    => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down  => Dir::Left,
            Dir::Left  => Dir::Up,
        };
    }
}

impl Into<Vec2> for Dir {
    fn into(self) -> Vec2 {
        match self {
            Self::Up    => Vec2 { x: -1, y:  0 },
            Self::Down  => Vec2 { x:  1, y:  0 },
            Self::Left  => Vec2 { x:  0, y: -1 },
            Self::Right => Vec2 { x:  0, y:  1 },
        }
    }
}

#[derive(Default)]
struct Guard {
    pos: Vec2,
    dir: Dir,
}

impl Guard {
    fn step(&mut self, map: &Map) -> bool {
        let next_pos = self.pos + self.dir.into();
        if contains(map, &next_pos) {
            if map[next_pos.x as usize][next_pos.y as usize] {
                self.dir.turn_right();
            } else {
                self.pos = next_pos;
            }
            return true;
        } else {
            return false;
        }
    }
}

fn contains(map: &Map, pos: &Vec2) -> bool {
    pos.x >= 0 && pos.y >= 0 && pos.x < map.len() as _ && pos.y < map[0].len() as _
}

fn parse_map(input: String) -> (Map, Guard) {
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
                    guard.pos = Vec2 { x: x as _, y: y as _ };
                },
                _ => panic!("Wrong format for map"),
            }
        }
        obstacles.push(obstacles_line);
    }
    (obstacles, guard)
}

fn main() {
    let filename = args().skip(1).next().unwrap();
    let input = read_to_string(File::open(filename).unwrap()).unwrap();

    let (map, mut guard) = parse_map(input);
    let mut history = HashSet::new();
    history.insert(guard.pos);
    while guard.step(&map) {
        history.insert(guard.pos);
    }

    println!("part 1: {}", history.len());
}
