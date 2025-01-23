use std::env::args;
use std::io::read_to_string;
use std::fs::File;
use std::thread;

mod blink;
mod utils;

// const LUT_STEP: usize = 15;

type Stones = Vec<u64>;
// type Lut = Vec<Stones>;

struct Lut {
    step: usize,
    table: Vec<Stones>,
}

// impl Lut {
//     fn blink_single(stone: u64) -> Stones {
//         todo!()
//     }

//     fn blink_all(stones: Stones) -> Stones {
//         todo!()
//     }
// }

// fn blink_repeatedly_all_len(stones: Stones, steps: usize) -> usize {
//     todo!()
// }

// fn solve(lut: &Lut, stones: &Stones, steps: usize) -> usize {
//     if steps == 0 {
//         // return blink::lut_blink_all(lut, stones).len();
//         let mut total = 0;
//         for &stone in stones {
//             total += lut.table.get(stone as usize).unwrap_or(&{
//                 let mut blinked = vec![stone];
//                 for _ in 0..lut.step {
//                     blinked = blink::all(blinked);
//                 }
//                 blinked
//             }).len();
//         }
//         return total;
//     }

//     let mut total = 0;
//     for &stone in stones {
//         total += solve(lut, lut.table.get(stone as usize).unwrap_or(&{
//             let mut blinked = vec![stone];
//             for _ in 0..lut.step {
//                 blinked = blink::all(blinked);
//             }
//             blinked
//         }), steps - 15);
//     }
//     total
// }

fn mt_create_lut(step: usize, lut_size: usize, thread_num: usize) -> Lut {
    let mut handles = vec![];
    let group_size = lut_size / thread_num;
    for i in 0..thread_num {
        let handle = thread::spawn(move || {
            let lut_range = (i * group_size) as u64 .. ((i + 1) * group_size) as u64;
            utils::create_lut(step, lut_range)
        });
        handles.push(handle);
    }

    let mut lut = Lut { step, table: Vec::default() };
    for handle in handles {
        let group = handle.join().unwrap();
        lut.table.extend(group.table);
    }
    lut
}

static mut LUT: Lut = Lut { step: 5, table: Vec::new() };

fn main() {
    let filename = args().skip(1).next().unwrap();
    let input = read_to_string(File::open(filename).unwrap()).unwrap();

    // Iterate vs. Recursive

    let mut stones = utils::parse(&input);
    // // for _ in 0..45 {
    // //     stones = blink::all(stones);
    // // }
    // // println!("stones: {}", stones.len());
    // let mut total = 0;
    // for stone in stones {
    //     // println!("calculating: {stone}");
    //     total += blink::repeatedly_single_len(stone, 45);
    // }
    // println!("total: {total}");

    let now = std::time::Instant::now();
    // let lut = utils::create_lut(15, 0..100_000);
    // let lut = mt_create_lut(5, 60_000_000, 25);
    unsafe { LUT = mt_create_lut(5, 60_000_000, 25) };
    println!("LUT creation time: {}", now.elapsed().as_secs_f32());

    let now = std::time::Instant::now();
    // println!("total: {}", solve(&lut, &stones, 45));
    // println!("total: {}", blink::recursive_len(&stones, 45));

    // Stage 1

    const PRE_ROUNDS: usize = 3;

    for _ in 0..PRE_ROUNDS {
        stones = unsafe { blink::lut_blink_all(&LUT, &stones) };
        println!("stage 1 len: {}", stones.len());
    }

    // Stage 2

    const THREADS: usize = 24;
    
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
            let mut batch_total = 0;
            for (i_stone, stone) in batch.into_iter().enumerate() {
                let count = unsafe { blink::recursive_one_len(&LUT, stone, 9 - PRE_ROUNDS) };
                batch_total += count;
                println!("batch: {i_batch:>2},  stone: {i_stone:>3},  count: {count:>12},  batch_total: {batch_total:>12},  time: {}s", now.elapsed().as_secs_f32());
            }
            batch_total
        });
        handles.push(handle);
        println!("spawned batch: {i_batch:>2}");
    }
    for handle in handles {
        total += handle.join().unwrap();
    }
    println!("total: {}", total);
    println!("Calculation time: {}", now.elapsed().as_secs_f32());


    // Vanilla

    // let mut stones = parse(&input);
    // for i in 0..75 {
    //     println!("Iteration: {:>2},  len: {}", i + 1, stones.len());
    //     stones = blink_all(stones);
    // }
    // println!("vanilla answer: {}", stones.len());

    // LUT

    // println!("Initializing LUT");
    // let lut = create_lut(LUT_STEP, 100_000);

    // let mut max = 0;
    // let mut avg = 0;
    // for stones in &lut {
    //     if stones.len() > max {
    //         max = stones.len();
    //     }
    //     avg += stones.len();
    // }
    // println!("max length in lut: {}", max);
    // println!("avg length in lut: {}", avg / 100_000);

    // let mut stones = parse(&input);
    // for i in 0..75 / LUT_STEP {
    //     println!("Iteration: {:>2} -> {:>2},  len: {}", i * LUT_STEP + 1, (i + 1) * LUT_STEP, stones.len());
    //     stones = lut_blink_all(&lut, stones);
    // }
    // println!("LUT answer: {}", stones.len());
}
