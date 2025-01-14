use std::env::args;
use std::fs::File;
use std::io::read_to_string;
use std::collections::HashMap;

fn parse(input: String) -> (Vec<u32>, Vec<u32>) {
    let (mut left, mut right) = (vec![], vec![]);
    input.lines().for_each(|line| {
        let (l, r) = line.split_once("   ").unwrap();
        left.push(l.parse::<u32>().unwrap());
        right.push(r.parse::<u32>().unwrap());
    });
    (left, right)
}

fn part_1(mut left: Vec<u32>, mut right: Vec<u32>) -> u32 {
    left.sort();
    right.sort();
    left.iter().zip(right).fold(0, |acc, (nl, nr)| acc + nl.abs_diff(nr))
}

fn part_2(left: Vec<u32>, right: Vec<u32>) -> u32 {
    let mut freqs = HashMap::new();
    let mut total = 0;
    for nl in left {
        total += nl * match freqs.get(&nl) {
            Some(f) => *f,
            None => {
                let f = right.iter().filter(|&nr| *nr == nl).count() as u32;
                freqs.insert(nl, f);
                f
            },
        };
    }
    total
}

fn main() {
    let filename = args().skip(1).next().unwrap();
    let input = read_to_string(File::open(filename).unwrap()).unwrap();
    let (left, right) = parse(input);
    println!("part 1: {}", part_1(left.clone(), right.clone()));
    println!("part 2: {}", part_2(left, right));
}
