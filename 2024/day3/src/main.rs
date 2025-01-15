use std::env::args;
use std::fs::File;
use std::io::read_to_string;

fn part_1(input: String) -> u32 {
    let mut total = 0;
    for mut line in input.lines() {
        loop {
            let Some((_, s)) = line.split_once("mul(") else  {
                break;
            };
            line = s;
            let Some((s, remaining)) = s.split_once(')') else {
                continue;
            };
            if s.contains("mul(") {
                continue;
            };
            line = remaining;
            let Some((n1, n2)) = s.split_once(',') else {
                continue;
            };
            let (Ok(n1), Ok(n2)) = (n1.parse::<u32>(), n2.parse::<u32>()) else {
                continue;
            };
            total += n1 * n2;
        }
    }
    total
}

fn main() {
    let filename = args().skip(1).next().unwrap();
    let input = read_to_string(File::open(filename).unwrap()).unwrap();

    println!("part 1: {}", part_1(input));
}
