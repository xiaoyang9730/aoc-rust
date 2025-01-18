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

fn solve<F>(equations: &Vec<Equation>, num_of_ops: u32, f: F) -> u64
where
    F: Fn(u32, u64, u64) -> u64
{
    equations.iter()
        .filter_map(|(target, numbers)| {
            // Select operators based on a counter
            for mut cnt in 0..num_of_ops.pow(numbers.len() as u32 - 1) {
                let mut acc = numbers[0];
                for i in 1..numbers.len() {
                    let op = cnt % num_of_ops;
                    acc = f(op, acc, numbers[i]);
                    cnt /= num_of_ops;
                }
                if acc == *target {
                    return Some(*target);
                }
            }
            return None;
        })
        .fold(0, |acc, v| acc + v)
}

fn part_1(equations: &Vec<Equation>) -> u64 {
    solve(equations, 2, |op, acc, number| {
        if op == 0 {
            acc + number
        } else {
            acc * number
        }
    })
}

fn part_2(equations: &Vec<Equation>) -> u64 {
    solve(equations, 3, |op, mut acc, number| {
        match op % 3 {
            0 => acc + number,
            1 => acc * number,
            _ => {
                let mut shifter = number;
                while shifter != 0 {
                    acc *= 10;
                    shifter /= 10;
                }
                acc + number
            },
        }
    })
}

fn main() {
    let filename = args().skip(1).next().unwrap();
    let input = read_to_string(File::open(filename).unwrap()).unwrap();

    let equations = parse(input);
    println!("part 1: {}", part_1(&equations));
    println!("part 2: {}", part_2(&equations));
}
