use std::env::args;
use std::fs::File;
use std::io::read_to_string;
use std::thread;
use std::time::Instant;

mod blink;
mod lut;
mod solution;
mod utils;

use lut::{Lut, LUT};

const STAGE1_ITERS: usize = 35;

type Stones = Vec<usize>;

fn main() {
    let total_time = Instant::now();
    let filename = args().skip(1).next().unwrap();
    let input = read_to_string(File::open(filename).unwrap()).unwrap();

    let stones = utils::parse(&input);

    // Initialize LUT, while doing stage 1 calculation

    let now = std::time::Instant::now();
    let init_lut = thread::spawn(|| Lut::new(5, 40_000_000, 30));
    let stage_1 = solution::Stage1::run(stones, STAGE1_ITERS);
    println!("stage 1: {}s", now.elapsed().as_secs_f32());
    unsafe { LUT = init_lut.join().unwrap() };
    println!("LUT init thread joined: {}s", now.elapsed().as_secs_f32());

    // Stage 2 calculation

    let now = std::time::Instant::now();
    // println!("part 1: {}", solution::Stage2::run(&stage_1, 25 - STAGE1_ITERS, 17));
    println!("part 2: {}", solution::Stage2::run(&stage_1, 75 - STAGE1_ITERS, 30));
    println!("time for part 2 stage 2: {}s", now.elapsed().as_secs_f32());

    println!("total time: {}s", total_time.elapsed().as_secs_f32());
}
