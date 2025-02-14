use std::env::args;
use std::fs::read_to_string;

#[derive(Clone, Copy, Debug)]
struct IVec2 { x: isize, y: isize }

impl From<&str> for IVec2 {
    fn from(value: &str) -> Self {
        let (x, y) = value.split_once(',').unwrap();
        IVec2 { x: x.parse().unwrap(), y: y.parse().unwrap() }
    }
}

#[derive(Clone, Copy, Debug)]
struct Robot { p: IVec2, v: IVec2 }

impl From<&str> for Robot {
    fn from(value: &str) -> Self {
        let (p, v) = value.split_once(' ').unwrap();
        Robot {
            p: p.trim_start_matches("p=").into(),
            v: v.trim_start_matches("v=").into(),
        }
    }
}

impl Robot {
    fn step(&mut self, count: isize) {
        self.p = IVec2 {
            x: self.p.x + count * self.v.x,
            y: self.p.y + count * self.v.y,
        }
    }

    fn constraint(&mut self, region: &IVec2) {
        self.p = IVec2 {
            x: self.p.x % region.x,
            y: self.p.y % region.y,
        };
        if self.p.x < 0 { self.p.x += region.x; }
        if self.p.y < 0 { self.p.y += region.y; }
    }

    fn is_inside(&self, top_left: &IVec2, bottom_right: &IVec2) -> bool {
        self.p.x >= top_left.x && self.p.x <= bottom_right.x &&
        self.p.y >= top_left.y && self.p.y <= bottom_right.y
    }
}

#[derive(Debug)]
struct Quadrant {
    top_left: IVec2,
    bottom_right: IVec2,
    count: usize,
}

impl Quadrant {
    fn new(top_left: IVec2, bottom_right: IVec2) -> Self {
        Self { top_left, bottom_right, count: 0 }
    }

    fn check(&mut self, robot: &Robot) -> bool {
        if robot.is_inside(&self.top_left, &self.bottom_right) {
            self.count += 1;
            return true;
        }
        false
    }
}

fn parse(input: &str) -> Vec<Robot> {
    input.lines().map(Robot::from).collect()
}

fn main() {
    let filename = args().skip(1).next().unwrap();
    let input = read_to_string(filename).unwrap();

    let mut parsed = parse(&input);

    let wide = args().skip(2).next().unwrap().parse::<isize>().unwrap();
    let tall = args().skip(3).next().unwrap().parse::<isize>().unwrap();

    let region = IVec2 { x: wide, y: tall };
    let mut quadrants = [
        Quadrant::new(IVec2 { x: 0, y: 0 }, IVec2 { x: wide / 2 - 1, y: tall / 2 - 1 }),
        Quadrant::new(IVec2 { x: (wide + 1) / 2, y: 0 }, IVec2 { x: wide - 1, y: tall / 2 - 1 }),
        Quadrant::new(IVec2 { x: 0, y: (tall + 1) / 2 }, IVec2 { x: wide / 2 - 1, y: tall - 1 }),
        Quadrant::new(IVec2 { x: (wide + 1) / 2, y: (tall + 1) / 2 }, IVec2 { x: wide - 1, y: tall - 1 }),
    ];
    for robot in &mut parsed {
        robot.step(100);
        robot.constraint(&region);
        println!("{robot:?}");

        for quadrant in &mut quadrants {
            if quadrant.check(robot) {
                println!("    belongs to {quadrant:?}");
            }
        }
    }

    let result = quadrants.iter().fold(1, |acc, q| acc * q.count);
    println!("part 1: {result}");
}
