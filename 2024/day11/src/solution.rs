use std::collections::HashMap;
use std::time::Instant;
use std::thread;

use crate::Stones;
use crate::blink;
use crate::lut::{Lut, LUT};
use crate::utils;

const LUT_STEPS: usize = 5;
const LUT_SIZE: usize = 60_000_000;
const LUT_INIT_THREADS: usize = 25;

const TOTAL_ITERS: usize = 60;
const PRE_ITERS: usize = 15;
const STAGE_2_THREADS: usize = 17;

pub fn solve(stones: Stones) -> usize {
    // LUT init & Stage 1
    let lut_init = thread::spawn(|| Lut::new(LUT_STEPS, LUT_SIZE, LUT_INIT_THREADS));
    let deduped = dedup(pre_iterate(stones));
    unsafe { LUT = lut_init.join().unwrap() };

    // Stage 2
    let time = Instant::now();
    let result = stage_2(deduped);
    println!("result: {result}, stage_2 time: {}s", time.elapsed().as_secs_f32());
    result
}

#[inline]
fn pre_iterate(stones: Stones) -> Stones {
    utils::repeat(blink::all, stones, PRE_ITERS)
}

type Deduped = HashMap<usize, usize>; // Stone -> repeated times

fn dedup(stones: Stones) -> Deduped {
    let mut deduped = HashMap::new();
    for stone in &stones {
        let Some(count) = deduped.get_mut(stone) else {
            deduped.insert(*stone, 1);
            continue;
        };
        *count += 1;
    }
    deduped
}

type Batches = Vec<Stones>;

fn split_into_batches(mut stones: Stones) -> Batches {
    // TODO
    let batch_size = (stones.len() as f32 / STAGE_2_THREADS as f32).ceil() as usize;

    let mut batches = vec![];
    while stones.len() > batch_size {
        batches.push(stones.split_off(stones.len() - batch_size));
    }
    batches.push(stones);

    batches
}

type ThreadRet = Vec<(usize, usize)>; // Stone -> calc result

fn thread_func(stones: Stones) -> ThreadRet {
    stones.into_iter()
        .map(|stone| unsafe {
            (stone, LUT.blink_one_recursively(stone, (TOTAL_ITERS - PRE_ITERS) / LUT_STEPS))
        })
        .collect()
}

fn stage_2(deduped: Deduped) -> usize {
    let stones = deduped.keys().cloned().collect();
    let batches = split_into_batches(stones);

    let mut handles = vec![];
    for batch in batches {
        handles.push(thread::spawn(move || thread_func(batch)));
    }

    let mut counts = vec![];
    for handle in handles {
        counts.extend(handle.join().unwrap());
    }

    let mut total = 0;
    for (stone, count) in counts {
        total += count * deduped.get(&stone).unwrap();
    }

    total
}
