use std::env::args;
use std::fs::File;
use std::io::read_to_string;
use std::thread;

mod blink;
mod lut;
mod solution;
mod utils;

use lut::{Lut, LUT};

const STAGE1_ITERS: usize = 15;

type Stones = Vec<usize>;

fn main() {
    let filename = args().skip(1).next().unwrap();
    let input = read_to_string(File::open(filename).unwrap()).unwrap();

    let stones = utils::parse(&input);

    // Initialize LUT, while doing stage 1 calculation

    let init_lut = thread::spawn(|| Lut::new(5, 60_000_000, 30));
    let stage_1 = solution::Stage1::run(stones, STAGE1_ITERS);
    unsafe { LUT = init_lut.join().unwrap() };

    // Stage 2 calculation

    let now = std::time::Instant::now();
    println!("part 1: {}", solution::Stage2::run(&stage_1, 25 - STAGE1_ITERS, 17));
    println!("part 2: {}", solution::Stage2::run(&stage_1, 75 - STAGE1_ITERS, 17));
    println!("time for part 1 & part 2 stage 2: {}s", now.elapsed().as_secs_f32());
}
