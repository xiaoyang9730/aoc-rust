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

fn sides_of(region: &Region) -> u32 {
    let mut sides = 0;
    for &plot in region.iter() {
        sides += 4;
        if region.contains(&(plot + Pos::UP)) {
            sides -= 1;
        }
        if region.contains(&(plot + Pos::DOWN)) {
            sides -= 1;
        }
        if region.contains(&(plot + Pos::LEFT)) {
            sides -= 1;
        }
        if region.contains(&(plot + Pos::RIGHT)) {
            sides -= 1;
        }
    }
    sides
}

fn price_of(regions: &Vec<Region>) -> u32 {
    regions.iter()
        .map(|r| sides_of(r) * r.len() as u32)
        .fold(0, |acc, x| acc + x)
}

fn main() {
    let filename = args().skip(1).next().unwrap();
    let input = read_to_string(File::open(filename).unwrap()).unwrap();

    let regions = parse(input);
    println!("part 1: {}", price_of(&regions));
}
