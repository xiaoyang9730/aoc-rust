use std::collections::HashSet;
use std::env::args;
use std::fs::File;
use std::io::read_to_string;

mod utils;
use utils::*;

type Map<T> = Vec<Vec<T>>;

struct Solution {
    map: Map<i8>,
    trail_heads: Vec<Pos>,
}

impl Solution {
    const DIRS: [Pos; 4] = [
        Pos {x: -1, y:  0},
        Pos {x:  0, y: -1},
        Pos {x:  1, y:  0},
        Pos {x:  0, y:  1},
    ];

    fn new(input: String) -> Self {
        let map = input.lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as _).collect::<Vec<_>>())
            .collect::<Map<_>>();

        let mut trail_heads = vec![];
        for x in 0..map.len() {
            for y in 0..map[0].len() {
                if map[x][y] != 0 {
                    continue;
                }
                trail_heads.push(Pos { x: x as _, y: y as _ });
            }
        }
        
        Self { map, trail_heads }
    }

    fn score(&self, trail_head: Pos) -> u32 {
        let mut total = 0;
        let (mut c_track, mut n_track) = (HashSet::new(), HashSet::new());
        c_track.insert(trail_head);

        while !c_track.is_empty() {
            for &p in c_track.iter() {
                let c_height = self.map[p.x as usize][p.y as usize];
                if c_height == 9 {
                    total += 1;
                    continue;
                }

                for dir in Self::DIRS {
                    let np = p + dir;
                    if !np.is_inside(&self.map) {
                        continue;
                    }

                    let n_height = self.map[np.x as usize][np.y as usize];
                    if n_height - c_height != 1 {
                        continue;
                    }

                    n_track.insert(np);
                }
            }
            c_track.clear();
            n_track.drain().for_each(|np| { c_track.insert(np); });
        }
        total
    }

    fn part_1(&self) -> u32 {
        self.trail_heads.iter().map(|&th| self.score(th)).fold(0, u32::wrapping_add)
    }
}

fn main() {
    let filename = args().skip(1).next().unwrap();
    let input = read_to_string(File::open(filename).unwrap()).unwrap();

    let solution = Solution::new(input);
    println!("part 1: {}", solution.part_1());
}
