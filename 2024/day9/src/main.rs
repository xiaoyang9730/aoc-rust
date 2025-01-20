use std::env::args;
use std::fs::File;
use std::io::read_to_string;

type Block = Option<u64>;

fn parse(input: String) -> Vec<Block> {
    let mut is_file = true;
    let mut id = 0;
    let mut blocks = vec![];
    for n in input.trim().chars().map(|ch| ch.to_digit(10).unwrap()) {
        if is_file {
            for _ in 0..n {
                blocks.push(Some(id));
            }
            id += 1;
        } else {
            for _ in 0..n {
                blocks.push(None);
            }
        }
        is_file = !is_file;
    }
    blocks
}

fn reorder(blocks: &mut Vec<Block>) {
    for i in (0..blocks.len()).rev() {
        if blocks[i].is_some() {
            for j in 0..i {
                if blocks[j].is_none() {
                    blocks[j] = blocks[i];
                    blocks[i] = None;
                    break;
                }
            }
        }
    }
}

fn checksum(blocks: &Vec<Block>) -> u64 {
    blocks.iter()
        .enumerate()
        .map(|(i, id)| if let Some(id) = id { id * i as u64 } else { 0 })
        .fold(0, |acc, v| acc + v)
}

fn main() {
    let filename = args().skip(1).next().unwrap();
    let input = read_to_string(File::open(filename).unwrap()).unwrap();

    let mut blocks = parse(input);
    reorder(&mut blocks);
    println!("part 1: {}", checksum(&blocks));
}
