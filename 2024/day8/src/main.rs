use std::env::args;
use std::fs::File;
use std::io::read_to_string;
use std::collections::{HashMap, HashSet};

mod utils;
use utils::*;

type Freq = char;
type PosLists = HashMap<Freq, Vec<Pos>>;
type MapSize = (isize, isize);

fn antinode_of(pa: Pos, pb: Pos, (map_x, map_y): MapSize) -> Option<Pos> {
    let antinode = pa + (pa - pb);
    if antinode.is_inside(map_x, map_y) {
        return Some(antinode);
    }
    return None;
}

fn part_1(lists: &PosLists, map_size: MapSize) -> usize {
    let mut antinodes = HashSet::new();
    for list in lists.values() {
        if list.len() < 2 {
            continue;
        }

        for (i, &pa) in list[..list.len()-1].iter().enumerate() {
            for &pb in list[i+1..list.len()].iter() {
                if let Some(antinode) = antinode_of(pa, pb, map_size) {
                    antinodes.insert(antinode);
                }
                if let Some(antinode) = antinode_of(pb, pa, map_size) {
                    antinodes.insert(antinode);
                }
            }
        }
    }
    antinodes.len()
}

fn main() {
    let filename = args().skip(1).next().unwrap();
    let input = read_to_string(File::open(filename).unwrap()).unwrap();

    let (lists, map_size) = parse(input);
    println!("part 1: {}", part_1(&lists, map_size));
}
