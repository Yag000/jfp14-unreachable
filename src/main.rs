use std::env::args;

fn decompress(path: String) -> String {
    let input = std::fs::read_to_string(path).unwrap();
    input.lines();

    return "y".to_string();
}

fn main() {
    println!("{}",decompress(args().nth(1).unwrap()));
}
