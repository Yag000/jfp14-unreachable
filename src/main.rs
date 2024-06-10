use std::env::args;

fn decompress(path: String) -> String {
    let input = std::fs::read_to_string(path).unwrap();

    let mut line_iter = input.lines();

    let mut intrs: Vec<(String, String)> = Vec::new();

    while let Some(l) = line_iter.next() {
        if l.is_empty() {
            break;
        }

        let split: Vec<&str> = l.split("=").collect();
        intrs.push((split[0].to_string(), split[1].to_string()));
    }

    let str_input = line_iter.next().unwrap();

    for (a, b) in intrs {
        println!("{a}={b}");
    }

    println!("{str_input}");

    return "y".to_string();
}

fn main() {
    println!("{}", decompress(args().nth(1).unwrap()));
}
