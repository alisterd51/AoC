use day17::{parse_containers, solve_part_1};
use std::io::{self, Read};

fn get_input() -> Vec<usize> {
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);

    parse_containers(&buf)
}

fn main() {
    let input = get_input();
    let result = solve_part_1(&input, 150);
    println!("{result}");
}
