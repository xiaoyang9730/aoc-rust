use super::{lut::Lut, Stones};
use super::utils::digits;

fn one(stone: usize) -> Stones {
    if stone == 0 {
        return vec![1];
    }

    let digits = digits(stone);
    if digits % 2 == 0 {
        let div = 10usize.pow(digits / 2);
        return vec![stone / div, stone % div];
    }

    return vec![stone * 2024];
}

pub fn all(stones: &[usize]) -> Stones {
    let mut blinked = vec![];
    for &stone in stones {
        blinked.extend(one(stone));
    }
    blinked
}

pub fn recursive_one_len(lut: &'static Lut, stone: usize, times: usize) -> usize {
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

pub fn lut_blink_single(lut: &Lut, stone: usize) -> Stones {
    lut.table.get(stone as usize).cloned().unwrap_or({
        let mut blinked = vec![stone];
        for _ in 0..lut.step {
            blinked = all(&blinked);
        }
        blinked
    })
}

pub fn lut_blink_all(lut: &Lut, stones: &Stones) -> Stones {
    let mut blinked = vec![];
    for &stone in stones {
        blinked.extend(lut_blink_single(lut, stone));
    }
    blinked
}

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
}
