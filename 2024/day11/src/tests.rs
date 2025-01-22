mod utils {
    use super::super::Stones;

    pub fn iterations() -> [Stones; 7] {
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
}

#[test]
fn test_count_digits() {
    use super::count_digits;

    for exp in 0..9 {
    // for exp in 0..19 {
        for n in 10u64.pow(exp)..10u64.pow(exp + 1) {
            assert_eq!(exp + 1, count_digits(n));
        }
    }
    // for n in 10u64.pow(19)..=u64::MAX {
    //     assert_eq!(19 + 1, count_digits(n));
    // }
}

#[test]
fn test_blink_single() {
    use super::blink_single;

    assert_eq!(blink_single(0), vec![1]);
    assert_eq!(blink_single(1), vec![1 * 2024]);
    assert_eq!(blink_single(10), vec![1, 0]);
    assert_eq!(blink_single(99), vec![9, 9]);
    assert_eq!(blink_single(999), vec![999 * 2024]);
}

#[test]
fn test_blink_all() {
    use super::blink_all;

    let iterations = utils::iterations();

    for i in 0..iterations.len()-1 {
        let blinked = blink_all(iterations[i].clone());
        assert_eq!(blinked, iterations[i+1]);
    }
}

#[test]
fn test_lut_blink_all() {
    use super::{create_lut, lut_blink_all};

    let iterations = utils::iterations();

    for step in 1..iterations.len() {
        let lut = create_lut(step, 1000_0000);
        let blinked = lut_blink_all(&lut, iterations[0].clone());
        assert_eq!(blinked, iterations[step]);
    }
}
