use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io::read_to_string;

mod utils;
use utils::*;

type Map = Vec<Vec<i8>>;

#[derive(Default)]
struct Solution {
    map: Map,
    trail_heads: Vec<Pos>,
}

impl Solution {
    const DIRS: [Pos; 4] = [
        Pos {x: -1, y:  0}, Pos {x:  0, y: -1},
        Pos {x:  1, y:  0}, Pos {x:  0, y:  1},
    ];

    fn new(input: String) -> Self {
        Self::default()
            .parse_map(input)
            .collect_trail_heads()
    }

    fn parse_map(mut self, input: String) -> Self {
        self.map = input.lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap_or(10) as _).collect::<Vec<_>>())
            .collect::<Map>();
        self
    }

    fn collect_trail_heads(mut self) -> Self {
        for x in 0..self.map.len() {
            for y in 0..self.map[0].len() {
                if self.map[x][y] != 0 {
                    continue;
                }
                self.trail_heads.push(
                    Pos { x: x as _, y: y as _ }
                );
            }
        }
        self
    }

    fn trails(&self, p: Pos) -> HashMap<Pos, usize> {
        let height = self.map[p.x as usize][p.y as usize];
        if height == 9 {
            let mut ret = HashMap::new();
            ret.insert(p, 1);
            return ret;
        }

        let mut sum = HashMap::new();
        for dir in Self::DIRS {
            let next_p = p + dir;
            if !next_p.is_inside(&self.map) {
                continue;
            }

            let next_height = self.map[next_p.x as usize][next_p.y as usize];
            if next_height - height != 1 {
                continue;
            }

            for (end_p, increment) in self.trails(next_p) {
                if let Some(count) = sum.get_mut(&end_p) {
                    *count += increment;
                } else {
                    sum.insert(end_p, increment);
                }
            }
        }
        sum
    }

    fn part_1(&self) -> usize {
        self.trail_heads.iter()
            .map(|&p| self.trails(p).len())
            .fold(0, |acc, v| acc + v)
    }

    fn part_2(&self) -> usize {
        self.trail_heads.iter()
            .map(|&p| {
                self.trails(p).iter()
                    .map(|(_end_p, count)| count)
                    .fold(0, |acc, v| acc + v)
            })
            .fold(0, |acc, v| acc + v)
    }
}

fn main() {
    let filename = args().skip(1).next().unwrap();
    let input = read_to_string(File::open(filename).unwrap()).unwrap();

    let solution = Solution::new(input);
    println!("part 1: {}", solution.part_1());
    println!("part 2: {}", solution.part_2());
}
