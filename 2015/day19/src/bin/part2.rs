use day19::{Replacement, parse_replacements, solve_part_2};
use std::io::{self, Read};

fn get_input() -> (Vec<Replacement>, String) {
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);

    parse_replacements(&buf)
}

fn main() {
    let (replacements, molecule) = get_input();
    let result = solve_part_2(&replacements, &molecule);
    println!("{result}");
}
