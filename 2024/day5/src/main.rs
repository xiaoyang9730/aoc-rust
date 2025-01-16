use std::env::args;
use std::fs::File;
use std::io::read_to_string;

fn main() {
    let filename = args().skip(1).next().unwrap();
    let input = read_to_string(File::open(filename).unwrap()).unwrap();

    let mut is_rule = true;
    let (mut rules, mut updates) = (vec![], vec![]);
    for line in input.lines() {
        if line.is_empty() {
            is_rule = false;
            continue;
        }

        if is_rule {
            let (s1, s2) = line.split_once('|').unwrap();
            rules.push((s1.parse::<u32>().unwrap(), s2.parse::<u32>().unwrap()));
        } else {
            updates.push(line.split(',').map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>());
        }
    }

    let result = updates.iter()
        .filter(|pages| {
            for rule in &rules {
                let (mut head, mut tail) = (None, None);
                for (i, &page) in pages.iter().enumerate() {
                    if page == rule.0 {
                        head = Some(i);
                        break;
                    }
                }
                for (i, &page) in pages.iter().enumerate() {
                    if page == rule.1 {
                        tail = Some(i);
                        break;
                    }
                }
                if let (Some(h), Some(t)) = (head, tail) {
                    if h > t {
                        return false;
                    }
                }
            }
            return true;
        })
        .map(|pages| pages[(pages.len() - 1) / 2])
        .fold(0, |acc, v| acc + v);

    println!("part 1: {}", result);
}
