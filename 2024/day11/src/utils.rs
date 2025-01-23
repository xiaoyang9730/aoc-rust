use super::Stones;

pub fn parse(input: &str) -> Stones {
    input.trim().split(' ').map(|s| s.parse().unwrap()).collect()
}

pub fn repeat<F, P>(f: F, mut param: Vec<P>, times: usize) -> Vec<P>
where
    F: Fn(&[P]) -> Vec<P>,
{
    (0..times).for_each(|_| param = f(&param));
    param
}

#[inline]
pub fn digits(n: usize) -> u32 {
    let mut cmp = 10usize;
    let mut cnt = 1u32;
    loop {
        if n < cmp {
            break cnt;
        }
        cmp *= 10;
        cnt += 1;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn digits() {
        use super::digits;

        for exp in 0..9 {
            for n in 10usize.pow(exp)..10usize.pow(exp + 1) {
                assert_eq!(exp + 1, digits(n));
            }
        }
    }
}
