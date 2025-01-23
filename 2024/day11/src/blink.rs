use super::{Lut, Stones};
use super::utils::count_digits;

fn one(stone: u64) -> Stones {
    if stone == 0 {
        return vec![1];
    }

    let digits = count_digits(stone);
    if digits % 2 == 0 {
        let div = 10u64.pow(digits / 2);
        return vec![stone / div, stone % div];
    }

    return vec![stone * 2024];
}

pub fn all(stones: &[u64]) -> Stones {
    let mut blinked = vec![];
    for &stone in stones {
        blinked.extend(one(stone));
    }
    blinked
}

// pub fn recursive_len(stones: &[u64], times: usize) -> usize {
//     const TH: usize = 10_000_000;

//     if stones.len() < TH {
//         // if times == 0 {
//         //     return stones.len();
//         // }

//         let blinked = all(stones);

//         if times == 1 {
//             return blinked.len();
//         }
//         return recursive_len(&blinked, times - 1);
//     }

//     let sep = stones.len() / 2;
//     return recursive_len(&stones[..sep], times) + recursive_len(&stones[sep..], times);
// }

pub fn recursive_one_len(lut: &Lut, stone: u64, times: usize) -> usize {
    let next = match lut.table.get(stone as usize) {
        Some(result) => result,
        None => &{
            let mut stones = vec![stone];
            for _ in 0..lut.step {
                stones = all(&stones);
            }
            stones
        },
    };

    if times == 1 {
        return next.len();
    }

    let mut len = 0;
    for &stone in next {
        len += recursive_one_len(lut, stone, times - 1);
    }
    len
}

// pub fn repeatedly_single_len(stone: u64, times: usize) -> usize {
//     // println!("times: {times}");

//     let next = one(stone);
//     if times == 1 {
//         return next.len();
//     }

//     let mut len = 0;
//     for stone in next {
//         len += repeatedly_single_len(stone, times - 1);
//     }
//     len
// }

// pub fn repeatedly_single(stone: u64, times: usize) -> Stones {
//     let mut stones = vec![stone];
//     for _ in 0..times {
//         stones = all(stones);
//     }
//     stones
// }

// pub fn repeatedly_all(mut stones: Stones, times: usize) -> Stones {
//     for _ in 0..times {
//         stones = all(stones);
//     }
//     stones
// }

// pub fn lut_blink_single(lut: &Lut, stone: u64) -> Stones {
//     lut.table.get(stone as usize).cloned().unwrap_or({
//         let mut blinked = vec![stone];
//         for _ in 0..lut.step {
//             blinked = all(blinked);
//         }
//         blinked
//     })
// }

// pub fn lut_blink_all(lut: &Lut, stones: &Stones) -> Stones {
//     let mut blinked = vec![];
//     for &stone in stones {
//         blinked.extend(lut_blink_single(lut, stone));
//     }
//     blinked
// }

#[cfg(test)]
mod tests {
    use super::Stones;
    use crate::blink;

    fn iterations() -> [Stones; 7] {
        [
            vec![125, 17],
            vec![253000, 1, 7],
            vec![253, 0, 2024, 14168],
            vec![512072, 1, 20, 24, 28676032],
            vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032],
            vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32],
            vec![2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6, 0, 3, 2],
        ]
    }

    #[test]
    fn one() {
        assert_eq!(blink::one(0), vec![1]);
        assert_eq!(blink::one(1), vec![1 * 2024]);
        assert_eq!(blink::one(10), vec![1, 0]);
        assert_eq!(blink::one(99), vec![9, 9]);
        assert_eq!(blink::one(999), vec![999 * 2024]);
    }

    #[test]
    fn all() {
        let iterations = iterations();

        for i in 0..iterations.len()-1 {
            let blinked = blink::all(&iterations[i]);
            assert_eq!(blinked, iterations[i+1]);
        }
    }

    #[test]
    fn recursive_len() {
        let iterations = iterations();
        for i in 1..iterations.len() {
            assert_eq!(blink::recursive_len(&iterations[0], i), iterations[i].len());
        }
    }

    // #[test]
    // fn repeatedly_single() {
    //     assert_eq!(blink::repeatedly_single(0, 4), vec![2, 0, 2, 4]);
    // }

    // #[test]
    // fn repeatedly_all() {
    //     let iterations = iterations();
    //     assert_eq!(blink::repeatedly_all(iterations[0].clone(), iterations.len()-1), iterations[iterations.len()-1]);
    // }

    // #[test]
    // fn lut_blink_all() {
    //     // use super::{create_lut, lut_blink_all};
    //     use crate::utils::create_lut;
    //     use crate::blink::lut_blink_all;

    //     let iterations = iterations();

    //     for step in 1..iterations.len() {
    //         let lut = create_lut(step, 1000_0000);
    //         let blinked = lut_blink_all(&lut, iterations[0].clone());
    //         assert_eq!(blinked, iterations[step]);
    //     }
    // }
}
