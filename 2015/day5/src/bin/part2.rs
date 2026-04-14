use std::io::{self, Read};

use day5::{parse_string, solve_part_2};

fn get_input() -> String {
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);

    buf
}

fn main() {
    let input = get_input();
    let input = parse_string(&input);
    let result = solve_part_2(&input);
    println!("{result}");
}
