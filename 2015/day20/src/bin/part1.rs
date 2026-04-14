use day20::{parse_input, solve_part_1};
use std::io::{self, Read};

fn get_input() -> Option<u64> {
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);

    parse_input(&buf)
}

fn main() {
    if let Some(input) = get_input() {
        let result = solve_part_1(input);
        println!("{result}");
    }
}
