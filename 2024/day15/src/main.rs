use core::panic;
use std::env::args;
use std::fs::read_to_string;
use std::ops::Add;

const MAP_ROWS: usize = 50;
const MAP_COLS: usize = 50;

#[derive(Clone, Copy, Debug)]
struct Vec2 {
    row: isize,
    col: isize,
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { row: self.row + rhs.row, col: self.col + rhs.col }
    }
}

#[derive(Clone, Copy)]
enum Item {
    Wall,
    Box,
}

struct Input {
    map: [[Option<Item>; MAP_COLS]; MAP_ROWS],
    movements: String,
    robot: Vec2,
}

impl Input {
    fn parse(input: &str) -> Self {
        let mut parsed = Self {
            map: [[None; MAP_COLS]; MAP_ROWS],
            movements: String::new(),
            robot: Vec2 { row: 0, col: 0 },
        };

        let lines = &mut input.lines();
        for (row, line) in lines.enumerate() {
            if line.is_empty() { break; }
            for (col, ch) in line.char_indices() {
                match ch {
                    '#' => parsed.map[row][col] = Some(Item::Wall),
                    'O' => parsed.map[row][col] = Some(Item::Box),
                    '.' => {},
                    '@' => parsed.robot = Vec2 { row: row as _, col: col as _ },
                    _ => panic!("`{}` is not valid in map", ch),
                }
            }
        }
        parsed.movements = lines.collect();
        parsed
    }

    fn part_1(&mut self) {
        for ch in self.movements.chars() {
            let dir = match ch {
                '<' => Vec2 { row:  0, col: -1 },
                '>' => Vec2 { row:  0, col:  1 },
                '^' => Vec2 { row: -1, col:  0 },
                'v' => Vec2 { row:  1, col:  0 },
                _ => panic!("`{}` is not valid movement", ch),
            };

            if try_to_push(&mut self.map, self.robot, dir) {
                self.robot = self.robot + dir;
                self.map[self.robot.row as usize][self.robot.col as usize] = None;
            }
        }
    }

    fn score(&self) -> usize {
        self.map.iter().enumerate().map(|(r, row)| {
            row.iter().enumerate().filter_map(|(c, col)| {
                if let &Some(Item::Box) = col {
                    Some(r * 100 + c)
                } else {
                    None
                }
            }).fold(0, |acc, x| acc + x)
        }).fold(0, |acc, x| acc + x)
    }
}

fn try_to_push(map: &mut [[Option<Item>; MAP_COLS]; MAP_ROWS], pos: Vec2, dir: Vec2) -> bool {
    let next_pos = pos + dir;
    match map[next_pos.row as usize][next_pos.col as usize] {
        Some(Item::Wall) => false,
        Some(Item::Box) => try_to_push(map, next_pos, dir),
        None => { map[next_pos.row as usize][next_pos.col as usize] = Some(Item::Box); true },
    }
}

fn main() {
    let filename = args().skip(1).next().unwrap();
    let input = read_to_string(filename).unwrap();

    let mut parsed = Input::parse(&input);
    parsed.part_1();
    println!("part 1 score: {}", parsed.score());
    for row in parsed.map {
        for col in row {
            match col {
                Some(Item::Wall) => print!("w"),
                Some(Item::Box) => print!("b"),
                None => print!(" "),
            }
        }
        println!();
    }
    // // println!("{}", parsed.movements);
    // println!("{:?}", parsed.robot);
}
