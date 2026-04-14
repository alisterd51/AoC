use itertools::Itertools;
use std::collections::HashMap;

pub struct Happiness {
    a: String,
    b: String,
    units: i64,
}

#[must_use]
pub fn parse_happiness(input: &str) -> Vec<Happiness> {
    let mut happiness = vec![];

    for line in input.lines() {
        if let Some(line) = line.strip_suffix(".")
            && let words = line.split_whitespace().collect::<Vec<&str>>()
            && words.len() == 11
            && let a = words[0].to_string()
            && let b = words[10].to_string()
            && let Ok(mut units) = words[3].parse::<i64>()
        {
            if words[2] == "lose" {
                units = -units;
            }
            happiness.push(Happiness { a, b, units });
        }
    }

    happiness
}

fn build_graph(happiness: &[Happiness]) -> HashMap<String, HashMap<String, i64>> {
    let mut graph: HashMap<String, HashMap<String, i64>> = HashMap::new();
    for node in happiness {
        for (from, to) in [(&node.a, &node.b), (&node.b, &node.a)] {
            graph
                .entry(from.clone())
                .or_default()
                .entry(to.clone())
                .and_modify(|units| *units += node.units)
                .or_insert(node.units);
        }
    }

    graph
}

#[must_use]
pub fn solve_part_1(happiness: &[Happiness]) -> i64 {
    let graph = build_graph(happiness);
    let mut max_units = i64::MIN;

    for path in graph.keys().permutations(graph.len()) {
        let mut current_units = 0;
        for (from, to) in path.into_iter().circular_tuple_windows() {
            if let Some(node) = graph.get(from)
                && let Some(units) = node.get(to)
            {
                current_units += units;
            }
        }
        if max_units < current_units {
            max_units = current_units;
        }
    }

    max_units
}

#[must_use]
pub fn solve_part_2(happiness: &[Happiness]) -> i64 {
    let graph = build_graph(happiness);
    let mut max_units = i64::MIN;

    for path in graph.keys().permutations(graph.len()) {
        let mut current_units = 0;
        for (from, to) in path.into_iter().tuple_windows() {
            if let Some(node) = graph.get(from)
                && let Some(units) = node.get(to)
            {
                current_units += units;
            }
        }
        if max_units < current_units {
            max_units = current_units;
        }
    }

    max_units
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solve_part_1() {
        let input = String::from(
            "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.",
        );
        let input = parse_happiness(&input);
        let result = solve_part_1(&input);
        assert_eq!(result, 330);
    }

    #[test]
    fn example_solve_part_2() {}
}
