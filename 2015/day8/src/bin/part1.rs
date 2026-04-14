use day8::{parse_lines, solve_part_1};
use std::io::{self, Read};

fn get_input() -> Vec<String> {
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);

    parse_lines(&buf)
}

fn main() {
    let input = get_input();
    let result = solve_part_1(&input);
    println!("{result}");
}
