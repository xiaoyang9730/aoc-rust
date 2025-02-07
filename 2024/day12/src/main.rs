use std::collections::{HashMap, HashSet};
use std::env::args;
use std::fs::File;
use std::io::read_to_string;
use std::ops::Add;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    const UP: Self = Pos { x: -1, y: 0 };
    const DOWN: Self = Pos { x: 1, y: 0 };
    const LEFT: Self = Pos { x: 0, y: -1 };
    const RIGHT: Self = Pos { x: 0, y: 1 };

    fn new(x: usize, y: usize) -> Self {
        Self { x: x as isize, y: y as isize }
    }
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

type Region = HashSet<Pos>;

fn region(map: &mut HashMap<Pos, char>, plot: Pos, plant: char) -> Region {
    let mut plots = HashSet::new();

    let Some(&p) = map.get(&plot) else {
        return plots;
    };
    if p != plant {
        return plots;
    }

    plots.insert(plot);
    map.remove(&plot);

    plots.extend(region(map, plot + Pos::UP, plant));
    plots.extend(region(map, plot + Pos::DOWN, plant));
    plots.extend(region(map, plot + Pos::LEFT, plant));
    plots.extend(region(map, plot + Pos::RIGHT, plant));

    return plots;
}

fn parse(input: String) -> Vec<Region> {
    let mut map = HashMap::new();
    for (x, line) in input.lines().enumerate() {
        for (y, plant) in line.chars().enumerate() {
            map.insert(Pos::new(x, y), plant);
        }
    }

    let mut regions = vec![];
    while !map.is_empty() {
        let (&plot, &plant) = map.iter().next().unwrap();
        regions.push(region(&mut map, plot, plant));
    }
    regions
}

fn part_1_price_of(regions: &Vec<Region>) -> u32 {
    regions.iter()
        .map(|r| {
            let area = r.len() as u32;
            let mut sides = 0;
            for &plot in r.iter() {
                sides += 4;
                if r.contains(&(plot + Pos::UP)) {
                    sides -= 1;
                }
                if r.contains(&(plot + Pos::DOWN)) {
                    sides -= 1;
                }
                if r.contains(&(plot + Pos::LEFT)) {
                    sides -= 1;
                }
                if r.contains(&(plot + Pos::RIGHT)) {
                    sides -= 1;
                }
            }
            sides * area
        })
        .fold(0, |acc, x| acc + x)
}

fn part_2_price_of(regions: &Vec<Region>) -> u32 {
    regions.iter()
        .map(|r| {
            let area = r.len() as u32;

            let (mut hu_sides, mut hd_sides, mut vl_sides, mut vr_sides) = (HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new());
            for &plot in r.iter() {
                if !r.contains(&(plot + Pos::UP)) {
                    hu_sides.insert(plot);
                }
                if !r.contains(&(plot + Pos::DOWN)) {
                    hd_sides.insert(plot + Pos::DOWN);
                }
                if !r.contains(&(plot + Pos::LEFT)) {
                    vl_sides.insert(plot);
                }
                if !r.contains(&(plot + Pos::RIGHT)) {
                    vr_sides.insert(plot + Pos::RIGHT);
                }
            }

            let mut sides = 0;
            for &side in hu_sides.iter() {
                if !hu_sides.contains(&(side + Pos::LEFT)) {
                    sides += 1;
                }
            }
            for &side in hd_sides.iter() {
                if !hd_sides.contains(&(side + Pos::LEFT)) {
                    sides += 1;
                }
            }
            for &side in vl_sides.iter() {
                if !vl_sides.contains(&(side + Pos::UP)) {
                    sides += 1;
                }
            }
            for &side in vr_sides.iter() {
                if !vr_sides.contains(&(side + Pos::UP)) {
                    sides += 1;
                }
            }
            sides * area
        })
        .fold(0, |acc, x| acc + x)
}

fn main() {
    let filename = args().skip(1).next().unwrap();
    let input = read_to_string(File::open(filename).unwrap()).unwrap();

    let regions = parse(input);
    println!("part 1: {}", part_1_price_of(&regions));
    println!("part 2: {}", part_2_price_of(&regions));
}
