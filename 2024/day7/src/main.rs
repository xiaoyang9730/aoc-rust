use std::env::args;
use std::io::read_to_string;
use std::fs::File;

type Equation = (u64, Vec<u64>);

fn parse(input: String) -> Vec<Equation> {
    input.lines()
        .map(|line| {
            let (target, numbers) = line.split_once(": ").unwrap();
            let target = target.parse::<u64>().unwrap();
            let numbers = numbers.split(' ').map(|n| n.parse().unwrap()).collect::<Vec<u64>>();
            (target, numbers)
        })
        .collect()
}

fn part_1(equations: &Vec<Equation>) -> u64 {
    equations.iter()
        .filter_map(|(target, numbers)| {
            for mut op in 0..(1 << (numbers.len() - 1)) {
                let mut result = numbers[0];
                for i in 1..numbers.len() {
                    if op & 1 == 0 {
                        result += numbers[i];
                    } else {
                        result *= numbers[i];
                    }
                    op = op >> 1;
                }
                if result == *target {
                    return Some(*target);
                }
            }
            return None;
        })
        .fold(0, |acc, v| acc + v)
}

fn main() {
    let filename = args().skip(1).next().unwrap();
    let input = read_to_string(File::open(filename).unwrap()).unwrap();

    let equations = parse(input);
    println!("part 1: {}", part_1(&equations));
}
