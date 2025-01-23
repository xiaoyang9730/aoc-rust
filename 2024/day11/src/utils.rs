use std::ops::Range;

use crate::blink;
use super::{Lut, Stones};

pub fn parse(input: &str) -> Stones {
    input.trim().split(' ').map(|s| s.parse().unwrap()).collect()
}

#[inline]
pub fn count_digits(n: u64) -> u32 {
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

struct LutCreationTracker {
    ten_percent: u64,
    progress: u64,
    len: usize,
}

impl LutCreationTracker {
    fn new(lut_size: usize) -> Self {
        Self { ten_percent: lut_size as u64 / 10, progress: 0, len: 0 }
    }

    fn update(&mut self, stone: u64, len: usize) {
        if stone > self.progress {
            println!("Process...{}%", self.progress / self.ten_percent * 10);
            self.progress += self.ten_percent;
        }
        self.len += len;
    }

    fn finish(&self) {
        println!("LUT size: {}", self.len);
    }
}

pub fn create_lut(step: usize, lut_range: Range<u64>) -> Lut {
    let mut tracker = LutCreationTracker::new(lut_range.clone().count());

    let mut table = vec![];
    let start = lut_range.start;
    for stone in lut_range {
        let mut stones = vec![stone];
        for _ in 0..step {
            stones = blink::all(&stones);
        }
        tracker.update(stone - start, stones.len());
        table.push(stones);
    }
    tracker.finish();

    Lut { step, table }
}

#[cfg(test)]
mod tests {
    #[test]
    fn count_digits() {
        use super::count_digits;

        for exp in 0..9 {
            for n in 10u64.pow(exp)..10u64.pow(exp + 1) {
                assert_eq!(exp + 1, count_digits(n));
            }
        }
    }
}
