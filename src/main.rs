use std::{collections::HashMap, env::args};

use jfp14_unreachable::compression::compress_word;
use jfp14_unreachable::program::{Mode, Program};

fn decompress(path: String) -> String {
    let input = std::fs::read_to_string(path).unwrap();

    let mut line_iter = input.lines();

    let mut intrs: Vec<(String, String)> = Vec::new();

    while let Some(l) = line_iter.next() {
        if l.is_empty() {
            break;
        }

        let (left, right) = l.split_once("=").unwrap();
        intrs.push((left.to_string(), right.to_string()));
    }

    let str_input = line_iter.next().unwrap();

    let prog = Program::new(intrs, Mode::Decompress);

    prog.eval(str_input.to_string())
}

fn compress(path: String) -> String {
    let input = std::fs::read_to_string(path).unwrap();

    if let Some(l) = input.lines().next() {
        if l.is_empty() {
            unreachable!()
        }

        compress_word(l.to_string())
    } else {
        unreachable!()
    }
}

fn read_words() -> HashMap<String, u32> {
    let input = std::fs::read_to_string("tables_mots").unwrap();

    let mut hash = HashMap::new();

    let mut line_iter = input.lines();

    while let Some(word) = line_iter.next() {
        if word.is_empty() {
            break;
        }

        if let Some(value) = line_iter.next() {
            if value.is_empty() {
                break;
            }

            hash.insert(word.to_string(), value.parse::<u32>().unwrap() + 10000);
        } else {
            break;
        }
    }

    hash
}

fn main() {
    println!("{}", compress(args().nth(1).unwrap()));
}
