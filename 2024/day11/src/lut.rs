use std::ops::Range;
use std::thread;

use crate::Stones;
use crate::blink;
use crate::utils;

use LutBlinkResult::*;

pub static mut LUT: Lut = Lut { step: 0, table: Vec::new() };

pub struct Lut {
    pub step: usize,
    pub table: Vec<Stones>,
}

impl Lut {
    pub fn new(step: usize, size: usize, threads: usize) -> Self {
        let size_per_thread = size / threads;

        // Split into groups
        let mut handles = vec![];
        for i in 0..threads {
            let handle = thread::spawn(move || {
                Self::table(step, size_per_thread * i .. size_per_thread * (i + 1))
            });
            handles.push(handle);
        }
        let remaining_table = Self::table(step, size_per_thread * threads .. size);

        // Aggregate
        let mut table = handles.into_iter()
            .map(|h| h.join().unwrap())
            .fold(vec![], |mut acc, x| { acc.extend(x); acc });
        table.extend(remaining_table);

        Lut { step, table }
    }

    #[inline]
    fn table(step: usize, stone_range: Range<usize>) -> Vec<Stones> {
        let mut progress = LutInitProgress::new(stone_range.clone(), 8);
        stone_range
            .map(|stone| {
                progress.check(stone);
                utils::repeat(blink::all, vec![stone], step)
            })
            .collect()
    }

    pub fn blink_one(&self, stone: usize) -> LutBlinkResult {
        if let Some(lut_ret) = self.table.get(stone) {
            return Hit(lut_ret);
        }
        let mut blinked = vec![stone];
        for _ in 0..self.step {
            blinked = blink::all(&blinked);
        }
        Missed(blinked)
    }

    // pub fn blink_all(&self, stones: &Stones) -> Stones {
    //     let mut blinked = vec![];
    //     for &stone in stones {
    //         blinked.extend(self.blink_one(stone).as_ref());
    //     }
    //     blinked
    // }

    pub fn blink_one_recursively(&self, stone: usize, times: usize) -> usize {
        let blinked = match self.blink_one(stone) {
            Hit(lut_ret) => lut_ret,
            Missed(calculated) => &{ calculated },
        };

        if times == 1 {
            return blinked.len();
        }

        blinked.into_iter()
            .map(|&stone| self.blink_one_recursively(stone, times - 1))
            .fold(0, |acc, x| acc + x)
    }
}

pub enum LutBlinkResult<'a> {
    Hit(&'a Stones),
    Missed(Stones),
}

struct LutInitProgress {
    milestone: usize,
    phase_len: usize,
    range: Range<usize>,
}

impl LutInitProgress {
    fn new(range: Range<usize>, milestones: usize) -> Self {
        let phase_len = range.clone().count() / milestones;
        Self { milestone: range.start, phase_len, range }
    }

    fn check(&mut self, progress: usize) {
        if progress >= self.milestone {
            println!("Progress...{:>3.1}%", (self.milestone - self.range.start) as f32 / self.range.clone().count() as f32 * 100.0);
            self.milestone += self.phase_len;
        }
    }
}
