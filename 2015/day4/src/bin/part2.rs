use std::io::{self, Read};

use day4::solve_part_2;

fn get_input() -> String {
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);

    buf
}

fn main() {
    let input = get_input();
    if let Some(result) = solve_part_2(&input) {
        println!("{result}");
    }
}
