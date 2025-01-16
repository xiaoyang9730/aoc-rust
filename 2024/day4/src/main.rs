use std::fs::File;
use std::io::read_to_string;
use std::env::args;

fn parse(input: String) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn part_1(chars: Vec<Vec<char>>) -> usize {
    const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
    const SAMX: [char; 4] = ['S', 'A', 'M', 'X'];

    let lr = |x: usize, y: usize, target: [char; 4]| -> bool {
        (0..4).fold(true, |acc, i| {acc & (chars[y][x+i] == target[i])})
    };
    let tb = |x: usize, y: usize, target: [char; 4]| -> bool {
        (0..4).fold(true, |acc, i| {acc & (chars[y+i][x] == target[i])})
    };
    let ltrb = |x: usize, y: usize, target: [char; 4]| -> bool {
        (0..4).fold(true, |acc, i| {acc & (chars[y+i][x+i] == target[i])})
    };
    let lbrt = |x: usize, y: usize, target: [char; 4]| -> bool {
        (0..4).fold(true, |acc, i| {acc & (chars[y+i][x+3-i] == target[i])})
    };

    let len = chars.len();
    let mut total = 0;
    for y in 0..len {
        for x in 0..len {
            if x < len-3 && (lr(x, y, XMAS) || lr(x, y, SAMX)) {
                total += 1;
            }
            if y < len-3 && (tb(x, y, XMAS) || tb(x, y, SAMX)) {
                total += 1;
            }
            if x < len-3 && y < len-3 {
                if ltrb(x, y, XMAS) || ltrb(x, y, SAMX) {
                    total += 1;
                }
                if lbrt(x, y, XMAS) || lbrt(x, y, SAMX) {
                    total += 1;
                }
            }
        }
    }
    return total;
}

fn main() {
    let filename = args().skip(1).next().unwrap();
    let input = read_to_string(File::open(filename).unwrap()).unwrap();

    let chars = parse(input);
    println!("part 1: {}", part_1(chars));
}
