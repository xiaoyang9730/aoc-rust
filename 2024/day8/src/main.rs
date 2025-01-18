use std::env::args;
use std::fs::File;
use std::io::read_to_string;
use std::collections::{HashMap, HashSet};

mod utils;
use utils::*;

type Freq = char;
type PosLists = HashMap<Freq, Vec<Pos>>;
type MapSize = (isize, isize);

fn model_1(pa: Pos, pb: Pos, (map_x, map_y): MapSize) -> Option<Pos> {
    let antinode = pa + (pa - pb);
    if antinode.is_inside(map_x, map_y) {
        return Some(antinode);
    }
    return None;
}

fn model_2(mut pa: Pos, pb: Pos, (map_x, map_y): MapSize) -> Vec<Pos> {
    let mut antinodes = vec![];
    let diff = pa - pb;
    while pa.is_inside(map_x, map_y) {
        antinodes.push(pa);
        pa = pa + diff;
    }
    antinodes
}

fn solve(lists: &PosLists, map_size: MapSize) {
    let mut model_1_antinodes = HashSet::new();
    let mut model_2_antinodes = HashSet::new();
    for list in lists.values() {
        if list.len() < 2 {
            continue;
        }

        for (i, &pa) in list[..list.len()-1].iter().enumerate() {
            for &pb in list[i+1..list.len()].iter() {
                if let Some(antinode) = model_1(pa, pb, map_size) {
                    model_1_antinodes.insert(antinode);
                }
                if let Some(antinode) = model_1(pb, pa, map_size) {
                    model_1_antinodes.insert(antinode);
                }
                for antinode in model_2(pa, pb, map_size) {
                    model_2_antinodes.insert(antinode);
                }
                for antinode in model_2(pb, pa, map_size) {
                    model_2_antinodes.insert(antinode);
                }
            }
        }
    }
    println!("part 1: {}", model_1_antinodes.len());
    println!("part 2: {}", model_2_antinodes.len());
}

fn main() {
    let filename = args().skip(1).next().unwrap();
    let input = read_to_string(File::open(filename).unwrap()).unwrap();

    let (lists, map_size) = parse(input);
    solve(&lists, map_size);
}
