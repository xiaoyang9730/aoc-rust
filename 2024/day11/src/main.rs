use std::env::args;
use std::io::read_to_string;
use std::fs::File;
use std::thread;
use std::collections::HashMap;

mod blink;
mod utils;
mod lut;

type Stones = Vec<usize>;

fn main() {
    let filename = args().skip(1).next().unwrap();
    let input = read_to_string(File::open(filename).unwrap()).unwrap();

    let mut stones = utils::parse(&input);

    let now = std::time::Instant::now();
    unsafe { lut::LUT = lut::Lut::new(5, 60_000_000, 25) };
    println!("LUT creation time: {}", now.elapsed().as_secs_f32());

    let now = std::time::Instant::now();

    // Stage 1

    const PRE_ROUNDS: usize = 3;

    for _ in 0..PRE_ROUNDS {
        stones = unsafe { blink::lut_blink_all(&lut::LUT, &stones) };
        println!("stage 1 len: {}", stones.len());
    }

    // Stage 2.0

    let mut repetition_count = HashMap::new();
    for stone in &stones {
        match repetition_count.get_mut(stone) {
            Some(count) => { *count += 1; },
            None => { repetition_count.insert(*stone, 1); },
        }
    }

    let mut stones = repetition_count.keys().map(|s| *s).collect::<Vec<usize>>();
    println!("after repetition_count, stones: {}", stones.len());

    // Stage 2.1

    const THREADS: usize = 17;
    
    let mut total = 0;
    let mut handles = vec![];

    let batch_size = stones.len() / THREADS;
    println!("threads = {THREADS}, batch_size = {batch_size}, remaining = {}", stones.len() % THREADS);
    let mut batches = vec![];
    while stones.len() > batch_size {
        batches.push(stones.split_off(stones.len() - batch_size));
    }
    batches.push(stones);
    println!("splitted into {} batches", batches.len());

    for (i_batch, batch) in batches.into_iter().enumerate() {
        let handle = thread::spawn(move || {
            let mut thread_result = vec![];
            // let mut batch_total = 0;
            for (i_stone, stone) in batch.into_iter().enumerate() {
                let count = unsafe { blink::recursive_one_len(&lut::LUT, stone, 12 - PRE_ROUNDS) };
                // batch_total += count;
                thread_result.push((stone, count));
                println!("batch: {i_batch:>2},  stone: {i_stone:>3},  count: {count:>12},  time: {}s", now.elapsed().as_secs_f32());
            }
            thread_result
        });
        handles.push(handle);
        println!("spawned batch: {i_batch:>2}");
    }
    let mut counts = vec![];
    for handle in handles {
        counts.extend(handle.join().unwrap());
    }
    for (stone, count) in counts {
        total += count * repetition_count.get(&stone).unwrap();
    }
    println!("total: {}", total);
    println!("Calculation time: {}", now.elapsed().as_secs_f32());
}
