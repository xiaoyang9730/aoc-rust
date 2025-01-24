use super::Stones;
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
    stones.into_iter()
        .map(|&stone| one(stone))
        .fold(vec![], |mut acc, x| { acc.extend(x); acc })
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
