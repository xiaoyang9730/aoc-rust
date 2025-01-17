use std::cmp::Ordering;
use std::env::args;
use std::fs::File;
use std::io::read_to_string;

type Rules = Vec<(u32, u32)>;
type Updates = Vec<Vec<u32>>;
type Pages = Vec<u32>;

#[derive(Default)]
struct Solution {
    // Intermediates
    rules: Rules,
    updates: Updates,
    correctly_ordered: Updates,
    incorrectly_ordered: Updates,
    // Results
    part_1: Option<u32>,
    part_2: Option<u32>,
}

impl Solution {
    fn solve(input: String) -> Self {
        let mut solution = Solution::parse(input).split_updates();
        solution.part_1();
        solution.part_2();
        solution
    }

    fn parse(input: String) -> Self {
        let rules = input.lines()
            .filter(|l| l.contains('|'))
            .map(|l| l.split_once('|').unwrap())
            .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
            .collect::<Rules>();

        let updates = input.lines()
            .filter(|l| l.contains(','))
            .map(|l| l.split(',').map(|s| s.parse().unwrap()).collect::<Pages>())
            .collect::<Updates>();

        Self { rules, updates, ..Self::default() }
    }

    fn check(&self, pages: &Pages) -> bool {
        for &(previous, next) in self.rules.iter() {
            match (find(pages, previous), find(pages, next)) {
                (Some(p), Some(n)) => if p > n { return false },
                _ => {},
            }
        }
        return true;
    }

    fn split_updates(self) -> Self {
        let mut solution = Self { rules: self.rules, ..Self::default() };
        for pages in self.updates {
            if solution.check(&pages) {
                solution.correctly_ordered.push(pages);
            } else {
                solution.incorrectly_ordered.push(pages);
            }
        }
        solution
    }

    fn compare(&self, a: u32, b: u32) -> Ordering {
        for rule in &self.rules {
            if rule == &(a, b) {
                return Ordering::Less;
            }
            if rule == &(b, a) {
                return Ordering::Greater;
            }
        }
        return Ordering::Equal;
    }

    fn part_1(&mut self) -> u32 {
        if self.part_1.is_none() {
            // println!("Heavy compution for part 1");
            self.part_1 = Some(
                self.correctly_ordered.iter()
                    .map(middle)
                    .fold(0, |acc, v| acc + v)
            );
        }
        self.part_1.expect("Result for part 1 shouldn't be None")
    }

    fn part_2(&mut self) -> u32 {
        if self.part_2.is_none() {
            // println!("Heavy compution for part 2");
            self.part_2 = Some({
                let mut incorrectly_ordered = self.incorrectly_ordered.clone();
                incorrectly_ordered.iter_mut()
                    .map(|pages| {
                        pages.sort_by(|&a, &b| self.compare(a, b));
                        middle(&pages)
                    })
                    .fold(0, |acc, v| acc + v)
            });
        }
        self.part_2.expect("Result for part 2 shouldn't be None")
    }
}

#[inline]
fn find(pages: &Pages, target: u32) -> Option<usize> {
    for (i, &page) in pages.iter().enumerate() {
        if page == target {
            return Some(i);
        }
    }
    None
}

#[inline]
fn middle(pages: &Pages) -> u32 { pages[(pages.len() - 1) / 2] }

fn main() {
    let filename = args().skip(1).next().unwrap();
    let input = read_to_string(File::open(filename).unwrap()).unwrap();

    let mut solution = Solution::solve(input);
    println!("part 1: {}", solution.part_1());
    println!("part 2: {}", solution.part_2());
}
