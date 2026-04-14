use day7::{Operator, parse_operators, solve_part_1};
use std::io::{self, Read};

fn get_input() -> Vec<Operator> {
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);

    parse_operators(&buf)
}

fn main() {
    let input = get_input();
    let result = solve_part_1(&input, "a");
    println!("{result}");
}
