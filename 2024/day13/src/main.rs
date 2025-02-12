use std::env::args;
use std::fs::read_to_string;

#[derive(Clone, Copy, Debug)]
struct Pos {
    x: isize,
    y: isize,
}

#[derive(Clone, Copy, Debug)]
struct ClawMachine {
    btn_a: Pos,
    btn_b: Pos,
    prize: Pos,
}

fn parse_line(prefix: &str, split: char, line: &str) -> Pos {
    let line = line.trim_start_matches(prefix);
    let line = line.trim_start_matches(split);
    let (x, line) = line.split_once(", Y").unwrap();
    let y = line.trim_start_matches(split);
    Pos { x: x.parse().unwrap(), y: y.parse().unwrap() }
}

fn parse(input: String) -> Vec<ClawMachine> {
    let mut claw_machines = vec![];
    let mut lines = input.trim().lines();
    loop {
        let btn_a = parse_line("Button A: X", '+', lines.next().unwrap());
        let btn_b = parse_line("Button B: X", '+', lines.next().unwrap());
        let prize = parse_line("Prize: X", '=', lines.next().unwrap());
        claw_machines.push(ClawMachine { btn_a, btn_b, prize });
        if lines.next().is_none() {
            break claw_machines;
        }
    }
}

fn solve(&ClawMachine { btn_a, btn_b, prize }: &ClawMachine) -> Option<isize> {
    let mut minimum = None;
    for na in 0..=100 {
        if btn_a.x * na > prize.x {
            continue;
        }
        let nb = (prize.x - btn_a.x * na) / btn_b.x;
        if nb > 100 {
            continue;
        }
        if btn_a.x * na + btn_b.x * nb == prize.x && btn_a.y * na + btn_b.y * nb == prize.y {
            //calculate prize
            minimum = match minimum {
                None => Some((na, nb)),
                Some((ma, mb)) => {
                    if ma * 3 + mb > na * 3 + nb {
                        Some((na, nb))
                    } else {
                        minimum
                    }
                }
            };
        }
    }
    Some(minimum?.0 * 3 + minimum?.1)
}

fn solve_2(&ClawMachine { btn_a, btn_b, prize }: &ClawMachine) -> Option<isize> {
    let mut minimum = None;
    let prize_fixed = Pos { x: prize.x + 10000000000000, y: prize.y + 10000000000000 };

    let denominator = btn_b.y * btn_a.x - btn_a.y * btn_b.x;
    let guess_a = (prize_fixed.x * btn_b.y - prize_fixed.y * btn_b.x) / denominator;
    // let guess_b = (prize.y * btn_a.x - prize.x * btn_a.y) / denominator;

    for na in guess_a - prize.x .. guess_a + prize.x {
        if btn_a.x * na > prize_fixed.x {
            continue;
        }
        let nb = (prize_fixed.x - btn_a.x * na) / btn_b.x;
        if btn_a.x * na + btn_b.x * nb == prize_fixed.x && btn_a.y * na + btn_b.y * nb == prize_fixed.y {
            //calculate prize
            minimum = match minimum {
                None => Some((na, nb)),
                Some((ma, mb)) => {
                    if ma * 3 + mb > na * 3 + nb {
                        Some((na, nb))
                    } else {
                        minimum
                    }
                }
            };
        }
    }
    Some(minimum?.0 * 3 + minimum?.1)
}

fn main() {
    let filename = args().skip(1).next().unwrap();
    let input = read_to_string(filename).unwrap();

    let claw_machines = parse(input);
    let result = claw_machines.iter().map(|cm| solve(cm).unwrap_or(0)).fold(0, |acc, x| acc + x);
    println!("part 1: {result}");
    let result = claw_machines.iter().map(|cm| solve_2(cm).unwrap_or(0)).fold(0, |acc, x| acc + x);
    println!("part 2: {result}");
}

// dinominator = (btn_b.y * btn_a.x - btn_a.y * btn_b.x)
// a = (prize.x * btn_b.y - prize.y * btn_b.x) / dinominator
// b = (prize.y * btn_a.x - prize.x * btn_a.y) / dinominator
