use day14::{Reindeer, parse_reindeer, solve_part_2};
use std::io::{self, Read};

fn get_input() -> Vec<Reindeer> {
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);

    parse_reindeer(&buf)
}

fn main() {
    let input = get_input();
    if let Some(result) = solve_part_2(&input, 2503) {
        println!("{result}");
    }
}
