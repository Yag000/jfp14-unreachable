use std::env::args;

use jfp14_unreachable::compression::compress_word;

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

fn main() {
    println!("{}", compress(args().nth(1).unwrap()));
}
