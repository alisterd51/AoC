use day2::{Dimension, parse_dimensions, solve_part_2};
use std::io::{self, Read};

fn get_input() -> Vec<Dimension> {
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);

    parse_dimensions(&buf)
}

fn main() {
    let input = get_input();
    let result = solve_part_2(&input);
    println!("{result}");
}
