use std::ops::Range;
use std::thread;

use crate::Stones;
use crate::blink;
use crate::utils;

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
