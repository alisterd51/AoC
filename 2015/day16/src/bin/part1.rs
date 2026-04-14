use day16::{Sue, parse_sues, solve_part_1};
use std::io::{self, Read};

fn get_input() -> Vec<Sue> {
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);

    parse_sues(&buf)
}

fn main() {
    let input = get_input();
    let searched_sue = Sue {
        id: 0,
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
    };
    let result = solve_part_1(&input, &searched_sue);
    println!("{result}");
}
