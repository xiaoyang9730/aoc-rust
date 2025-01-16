use std::env::args;
use std::fs::File;
use std::io::read_to_string;
use std::iter::Peekable;

struct Lexer<'a> {
    src: &'a str,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.src.is_empty() {
            return None;
        }

        let ret;
        if self.src.starts_with("do()") {
            (ret, self.src) = self.src.split_at("do()".len());
            return Some(ret);
        }
        if self.src.starts_with("don't()") {
            (ret, self.src) = self.src.split_at("don't()".len());
            return Some(ret);
        }
        if self.src.starts_with("mul(") {
            (ret, self.src) = self.src.split_at("mul(".len());
            return Some(ret);
        }
        if self.src.starts_with(",") {
            (ret, self.src) = self.src.split_at(",".len());
            return Some(ret);
        }
        if self.src.starts_with(")") {
            (ret, self.src) = self.src.split_at(")".len());
            return Some(ret);
        }
        for i in 0..self.src.len() {
            let (leading, remaining) = self.src.split_at(i);
            if remaining.starts_with("do()") || remaining.starts_with("don't()") || remaining.starts_with("mul(") || remaining.starts_with(",") || remaining.starts_with(")") {
                self.src = remaining;
                return Some(leading);
            }
        }
        (ret, self.src) = self.src.split_at(self.src.len());
        return Some(ret);
    }
}

#[derive(Debug)]
enum Instruct {
    Do,
    Dont,
    Mul(u32, u32),
    Noop,
}

struct Parser<'a> {
    tokens: Peekable<Lexer<'a>>,
}

impl<'a> Iterator for Parser<'a> {
    type Item = Instruct;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(token) = self.tokens.next() else { return None; };
        match token {
            "do()" => return Some(Instruct::Do),
            "don't()" => return Some(Instruct::Dont),
            "mul(" => {
                let Some(n1) = self.tokens.next_if(|n| n.parse::<u32>().is_ok()) else {
                    return Some(Instruct::Noop);
                };
                let Some(_comma) = self.tokens.next_if(|n| *n == ",") else {
                    return Some(Instruct::Noop);
                };
                let Some(n2) = self.tokens.next_if(|n| n.parse::<u32>().is_ok()) else {
                    return Some(Instruct::Noop);
                };
                let Some(_parentheses) = self.tokens.next_if(|n| *n == ")") else {
                    return Some(Instruct::Noop);
                };

                let (Ok(n1), Ok(n2)) = (n1.parse::<u32>(), n2.parse::<u32>()) else {
                    return Some(Instruct::Noop);
                };
                // if comma != "," || parentheses != ")" {
                //     return Some(Instruct::Noop);
                // }
                return Some(Instruct::Mul(n1, n2));
            },
            _ => return Some(Instruct::Noop),
        }
    }
}

fn main() {
    let filename = args().skip(1).next().unwrap();
    let input = read_to_string(File::open(filename).unwrap()).unwrap();

    // println!("part 1: {}", run(&input, false));
    // println!("part 2: {}", run(&input, true));

    let (mut part_1, mut part_2) = (0, 0);
    let mut flag = true;
    for src in input.lines() {
        let parser = Parser { tokens: Lexer { src }.peekable() };
        for instruct in parser.into_iter() {
            if let Instruct::Do = instruct {
                flag = true;
            }
            if let Instruct::Dont = instruct {
                flag = false;
            }
            if let Instruct::Mul(n1, n2) = instruct {
                part_1 += n1 * n2;
                if flag {
                    part_2 += n1 * n2;
                }
            }
        }
    }
    println!("part 1: {part_1}");
    println!("part 2: {part_2}");
}
