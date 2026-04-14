use day21::{Stats, parse_stats, solve_part_1};
use std::io::{self, Read};

fn get_input() -> Stats {
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);

    parse_stats(&buf)
}

fn main() {
    let input = get_input();
    let result = solve_part_1(&input);
    println!("{result}");
}
