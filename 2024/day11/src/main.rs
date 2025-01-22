use std::env::args;
use std::io::read_to_string;
use std::fs::File;

#[cfg(test)]
mod tests;

const LUT_STEP: usize = 15;

type Stones = Vec<u64>;
type LUT = Vec<Stones>;

fn parse(input: &str) -> Stones {
    input.trim().split(' ').map(|s| s.parse().unwrap()).collect()
}

#[inline]
fn count_digits(n: u64) -> u32 {
    let mut cmp = 10u64;
    let mut cnt = 1u32;
    loop {
        if n < cmp {
            break cnt;
        }
        cmp *= 10;
        cnt += 1;
    }
}

fn blink_single(stone: u64) -> Stones {
    if stone == 0 {
        return vec![1];
    }

    let digits = count_digits(stone);
    if digits % 2 == 0 {
        let div = 10u64.pow(digits / 2);
        return vec![stone / div, stone % div];
    }

    return vec![stone * 2024];
}

fn blink_all(stones: Stones) -> Stones {
    let mut blinked = vec![];
    for stone in stones {
        blinked.extend(blink_single(stone));
    }
    blinked
}

fn create_lut(step: usize, lut_size: usize) -> LUT {
    let ten_percent = lut_size as u64 / 10;
    let mut progress = 0;
    let mut total = 0;

    let mut lut = vec![];
    for stone in 0..lut_size as u64 {
        if stone > progress {
            println!("Process...{}%", progress / ten_percent * 10);
            progress += ten_percent;
        }

        let mut stones = vec![stone];
        for _ in 0..step {
            stones = blink_all(stones);
        }
        total += stones.len();
        lut.push(stones);
    }
    println!("LUT size: {total}\n");

    lut
}

fn lut_blink_single(lut: &LUT, stone: u64) -> Stones {
    lut.get(stone as usize).cloned().unwrap_or({
        let mut blinked = vec![stone];
        for _ in 0..LUT_STEP {
            blinked = blink_all(blinked);
        }
        blinked
    })
}

fn lut_blink_all(lut: &LUT, stones: Stones) -> Stones {
    let mut blinked = vec![];
    for stone in stones {
        blinked.extend(lut_blink_single(lut, stone));
    }
    blinked
}

fn main() {
    let filename = args().skip(1).next().unwrap();
    let input = read_to_string(File::open(filename).unwrap()).unwrap();

    // Vanilla

    let mut stones = parse(&input);
    for i in 0..75 {
        println!("Iteration: {:>2},  len: {}", i + 1, stones.len());
        stones = blink_all(stones);
    }
    println!("vanilla answer: {}", stones.len());

    // LUT

    println!("Initializing LUT");
    let lut = create_lut(LUT_STEP, 100_000);

    let mut max = 0;
    let mut avg = 0;
    for stones in &lut {
        if stones.len() > max {
            max = stones.len();
        }
        avg += stones.len();
    }
    println!("max length in lut: {}", max);
    println!("avg length in lut: {}", avg / 100_000);

    let mut stones = parse(&input);
    for i in 0..75 / LUT_STEP {
        println!("Iteration: {:>2} -> {:>2},  len: {}", i * LUT_STEP + 1, (i + 1) * LUT_STEP, stones.len());
        stones = lut_blink_all(&lut, stones);
    }
    println!("LUT answer: {}", stones.len());
}
