use day12::{parse_json, solve_part_1};
use std::io::{self, Read};

fn get_input() -> Result<serde_json::Value, serde_json::Error> {
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);

    parse_json(&buf)
}

fn main() {
    if let Ok(input) = get_input() {
        let result = solve_part_1(&input);
        println!("{result}");
    }
}
