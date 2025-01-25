use std::collections::HashMap;
use std::thread;

use crate::Stones;
use crate::blink;
use crate::lut::LUT;
use crate::utils;

type Deduped = HashMap<usize, usize>; // Stone -> repeated times
type Batches = Vec<Stones>;
type ThreadRet = Vec<(usize, usize)>; // Stone -> calc result

pub struct Stage1 {
    #[allow(unused)]
    pre_iters: usize,
    deduped: Deduped,
}

impl Stage1 {
    pub fn run(stones: Stones, pre_iters: usize) -> Self {
        let pre_itered = utils::repeat(blink::all, stones, pre_iters);

        let mut deduped = HashMap::new();
        for stone in pre_itered {
            let Some(count) = deduped.get_mut(&stone) else {
                deduped.insert(stone, 1);
                continue;
            };
            *count += 1;
        }

        Self { pre_iters, deduped }
    }
}

pub struct Stage2;

impl Stage2 {
    pub fn run(stage1: &Stage1, iters: usize, threads: usize) -> usize {
        let stones = stage1.deduped.keys().cloned().collect();
        let batches = Self::batch(stones, threads);

        let mut handles = vec![];
        for (i, stones) in batches.into_iter().enumerate() {
            handles.push(thread::spawn(move || Self::thread_func(i, stones, iters)));
        }

        let mut counts = vec![];
        for handle in handles {
            counts.extend(handle.join().unwrap());
        }

        let mut total = 0;
        for (stone, count) in counts {
            total += count * stage1.deduped.get(&stone).unwrap();
        }

        total
    }

    fn batch(mut stones: Stones, threads: usize) -> Batches {
        // TODO
        let batch_size = (stones.len() as f32 / threads as f32).ceil() as usize;

        let mut batches = vec![];
        while stones.len() > batch_size {
            batches.push(stones.split_off(stones.len() - batch_size));
        }
        batches.push(stones);

        batches
    }

    fn thread_func(i_batch: usize, stones: Stones, iters: usize) -> ThreadRet {
        stones.into_iter().enumerate()
            .map(|(i_stone, stone)| {
                let result = unsafe { (stone, LUT.blink_one_recursively(stone, iters / LUT.step)) };
                println!("batch {i_batch:>2}  stone no.{i_stone:<5} result: {}", result.1);
                result
            })
            .collect()
    }
}
