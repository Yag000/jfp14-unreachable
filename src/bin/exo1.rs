use std::env::args;

use jfp14_unreachable::program::{Mode, Program};

fn decompress(path: String) -> String {
    let input = std::fs::read_to_string(path).unwrap();

    let mut line_iter = input.lines();

    let mut intrs: Vec<(String, String)> = Vec::new();

    for l in line_iter.by_ref() {
        if l.is_empty() {
            break;
        }

        let (left, right) = l.split_once('=').unwrap();
        intrs.push((left.to_string(), right.to_string()));
    }

    let str_input = line_iter.next().unwrap();

    let prog = Program::new(intrs, Mode::Decompress);

    prog.eval(str_input.to_string())
}

fn main() {
    println!("{}", decompress(args().nth(1).unwrap()));
}
