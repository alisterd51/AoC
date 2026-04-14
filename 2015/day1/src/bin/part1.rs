use std::io::{self, Read};

use day1::{parse_directions, solve_part_1};

fn get_input() -> Vec<i64> {
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);

    parse_directions(&buf)
}

fn main() {
    let input = get_input();
    let result = solve_part_1(&input);
    println!("{result}");
}
