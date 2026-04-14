use day1::{parse_directions, solve_part_2};
use std::io::{self, Read};

fn get_input() -> Vec<i64> {
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);

    parse_directions(&buf)
}

fn main() {
    let input = get_input();
    if let Some(result) = solve_part_2(&input) {
        println!("{result}");
    }
}
