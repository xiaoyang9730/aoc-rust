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
        stone_range
            .map(|stone| utils::repeat(blink::all, vec![stone], step))
            .collect()
    }
}
