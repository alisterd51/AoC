use day10::{parse_sequences, solve};
use std::io::{self, Read};

fn get_input() -> Vec<Vec<u32>> {
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);

    parse_sequences(&buf)
}

fn main() {
    let input = get_input();
    let result = solve(&input, 50);
    println!("{result}");
}
