use std::env::args;
use std::fs::File;
use std::io::read_to_string;

type Block = Option<u64>;

fn checksum(blocks: Vec<Block>) -> u64 {
    blocks.iter()
        .enumerate()
        .map(|(i, id)| if let Some(id) = id { id * i as u64 } else { 0 })
        .fold(0, |acc, v| acc + v)
}

fn part_1(input: &str) -> u64 {
    // parse input
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

    // reorder
    for i in (0..blocks.len()).rev() {
        if blocks[i].is_none() {
            continue;
        }
        for j in 0..i {
            if blocks[j].is_none() {
                blocks[j] = blocks[i];
                blocks[i] = None;
                break;
            }
        }
    }

    checksum(blocks)
}

#[derive(Clone, Copy, PartialEq)]
enum BlockType {
    File(u64, u64),
    Free(u64),
}

fn part_2(input: &str) -> u64 {
    // parse
    let mut blocks = vec![];
    let mut is_file = true;
    let mut id = 0;
    for n in input.trim().chars().map(|ch| ch.to_digit(10).unwrap() as u64) {
        if is_file {
            blocks.push(BlockType::File(id, n));
            id += 1;
        } else {
            blocks.push(BlockType::Free(n));
        }
        is_file = !is_file;
    }

    // reorder
    for i in (0..blocks.len()).rev() {
        let BlockType::File(_, n_file) = blocks[i] else {
            continue;
        };

        for j in 0..i {
            let BlockType::Free(n_free) = blocks[j] else {
                continue;
            };
            if n_file < n_free {
                let tmp = blocks[i];
                blocks[i] = BlockType::Free(n_file);
                blocks[j] = tmp;
                blocks.insert(j + 1, BlockType::Free(n_free - n_file));
                break;
            }
            if n_file == n_free {
                let tmp = blocks[i];
                blocks[i] = blocks[j];
                blocks[j] = tmp;
                break;
            }
        }
    }

    let list = block_to_list(&blocks);
    checksum(list)
}

fn block_to_list(blocks: &Vec<BlockType>) -> Vec<Block> {
    let mut list = vec![];
    for block in blocks {
        match block {
            &BlockType::File(id, n) => {
                for _ in 0..n {
                    list.push(Some(id));
                }
            },
            &BlockType::Free(n) => {
                for _ in 0..n {
                    list.push(None);
                }
            }
        }
    }
    list
}

fn main() {
    let filename = args().skip(1).next().unwrap();
    let input = read_to_string(File::open(filename).unwrap()).unwrap();

    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}
