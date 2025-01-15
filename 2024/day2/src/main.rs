use std::env::args;
use std::fs::File;
use std::io::read_to_string;

fn parse(input: String) -> Vec<Vec<u32>> {
    input.lines().map(|l| l.split(' ').map(|s| s.parse().unwrap()).collect()).collect()
}

fn predicate(levels: &&Vec<u32>) -> bool {
    let initial_dir = levels[0].ge(&levels[1]);
    for w in levels.windows(2) {
        let current_dir = w[0].ge(&w[1]);
        let diff = w[0].abs_diff(w[1]);
        if initial_dir != current_dir || diff < 1 || diff > 3 {
            return false;
        }
    }
    return true;
}

fn part_1(reports: &Vec<Vec<u32>>) -> usize {
    reports.iter().filter(predicate).count()
}

fn part_2(reports: &Vec<Vec<u32>>) -> usize {
    reports.iter()
        .filter(|levels| {
            if predicate(&&levels) {
                return true;
            }
            for i in 0..levels.len() {
                let mut cloned = (*levels).clone();
                cloned.remove(i);
                if predicate(&&cloned) {
                    return true;
                }
            }
            return false;
        })
        .count()
}

fn main() {
    let filename = args().skip(1).next().unwrap();
    let input = read_to_string(File::open(filename).unwrap()).unwrap();

    let reports = parse(input);
    println!("part 1: {}", part_1(&reports));
    println!("part 2: {}", part_2(&reports));
}
