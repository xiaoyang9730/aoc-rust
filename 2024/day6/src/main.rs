use std::env::args;
use std::fs::File;
use std::io::read_to_string;
use std::collections::HashSet;

mod utils;
use utils::*;

type Map = Vec<Vec<bool>>;

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Guard {
    pos: Pos,
    dir: Dir,
}

impl Guard {
    fn step(&mut self, map: &Map) {
        let next_pos = match self.dir {
            Dir::Up    => Pos { x: self.pos.x-1, y: self.pos.y   },
            Dir::Down  => Pos { x: self.pos.x+1, y: self.pos.y   },
            Dir::Left  => Pos { x: self.pos.x  , y: self.pos.y-1 },
            Dir::Right => Pos { x: self.pos.x  , y: self.pos.y+1 },
        };

        if next_pos.is_inside(map) && map[next_pos.x as usize][next_pos.y as usize] {
            self.turn_right();
        } else {
            self.pos = next_pos;
        }
    }

    fn turn_right(&mut self) {
        self.dir = match self.dir {
            Dir::Up    => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down  => Dir::Left,
            Dir::Left  => Dir::Up,
        }
    }
}

#[derive(Default)]
struct Solution {
    init_map: Map,
    init_guard: Guard,
    init_history: HashSet<Pos>,
}

impl Solution {
    fn parse(input: String) -> Self {
        let (init_map, init_guard) = parse(input);
        Self { init_map, init_guard, ..Self::default() }
    }

    fn part_1(&mut self) -> usize {
        let mut guard = self.init_guard;

        while guard.pos.is_inside(&self.init_map) {
            self.init_history.insert(guard.pos);
            guard.step(&self.init_map);
        }

        self.init_history.len()
    }

    fn part_2(&self) -> usize {
        let mut total = 0;
        for &Pos { x, y } in self.init_history.iter() {
            // Skip the init guard position
            if self.init_guard.pos.x == x && self.init_guard.pos.y == y {
                continue;
            }

            // Place a new obstruction
            let mut new_map = self.init_map.clone();
            new_map[x as usize][y as usize] = true;

            // Test if the new map can cause a loop
            let mut history_w_dir = HashSet::new();
            let mut guard = self.init_guard.clone();
            let map_is_good = loop {
                history_w_dir.insert(guard);
                guard.step(&new_map);
                if history_w_dir.contains(&guard) {
                    break true;
                }
                if !guard.pos.is_inside(&new_map) {
                    break false;
                }
            };

            // Count
            if map_is_good {
                total += 1;
            }
        }
        total
    }
}

fn main() {
    let filename = args().skip(1).next().unwrap();
    let input = read_to_string(File::open(filename).unwrap()).unwrap();

    let mut solution = Solution::parse(input);
    println!("part 1: {}", solution.part_1());
    println!("part 2: {}", solution.part_2());
}
