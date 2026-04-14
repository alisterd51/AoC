use std::io::{self, Read};

use day3::{Direction, parse_directions, solve_part_2};

fn get_input() -> Vec<Direction> {
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);

    parse_directions(&buf)
}

fn main() {
    let input = get_input();
    let result = solve_part_2(&input);
    println!("{result}");
}
