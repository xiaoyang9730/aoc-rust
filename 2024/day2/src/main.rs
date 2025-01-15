use std::env::args;
use std::fs::File;
use std::io::read_to_string;

fn part_1(input: String) -> u32 {
    input.lines()
        .filter(|line| {
            let levels: Vec<u32> = line.split(' ').map(|s| s.parse().unwrap()).collect();
            let initial_dir = levels[0].ge(&levels[1]);
            for w in levels.windows(2) {
                let current_dir = w[0].ge(&w[1]);
                let diff = w[0].abs_diff(w[1]);
                if initial_dir != current_dir || diff < 1 || diff > 3 {
                    return false;
                }
            }
            return true;
        })
        .count() as _
}

fn main() {
    let filename = args().skip(1).next().unwrap();
    let input = read_to_string(File::open(filename).unwrap()).unwrap();

    println!("part 1: {}", part_1(input));
}
