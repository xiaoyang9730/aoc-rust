use std::env::args;
use std::io::read_to_string;
use std::fs::File;

type Stones = Vec<u64>;

fn parse(input: String) -> Stones {
    input.trim().split(' ').map(|s| s.parse().unwrap()).collect()
}

fn count_digits(mut n: u64) -> u32 {
    let mut cnt = 0;
    while n != 0 {
        cnt += 1;
        n /= 10;
    }
    cnt.max(1)
}

fn blink_at(stones: Stones) -> Stones {
    let mut result = Stones::new();

    for stone in stones {
        if stone == 0 {
            result.push(1);
            continue;
        }

        let digits = count_digits(stone);
        if digits % 2 == 0 {
            let div = 10u64.pow(digits / 2);
            result.push(stone / div);
            result.push(stone % div);
            continue;
        }

        result.push(stone * 2024);
    }

    result
}

fn main() {
    let filename = args().skip(1).next().unwrap();
    let input = read_to_string(File::open(filename).unwrap()).unwrap();

    let mut stones = parse(input);
    for _ in 0..25 {
        stones = blink_at(stones);
    }
    println!("part 1: {}", stones.len());
}
