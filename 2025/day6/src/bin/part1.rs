use std::io::{self, Read};

use day6::{Problem, parse_homework_1, solve_part_1};

fn get_input() -> Vec<Problem> {
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);

    parse_homework_1(&buf)
}

fn main() {
    let input = get_input();
    let result = solve_part_1(&input);
    println!("{result}");
}
